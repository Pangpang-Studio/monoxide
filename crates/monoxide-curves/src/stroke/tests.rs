use kurbo::{
    ParamCurve, ParamCurveArclen, ParamCurveDeriv, ParamCurveNearest, PathSeg, Point, Shape,
};
use monoxide_spiro::{SpiroCp, SpiroCpTy};

use super::*;
use crate::convert::spiro_to_cube;

fn point(x: f64, y: f64, ty: SpiroCpTy) -> SpiroCp {
    SpiroCp { x, y, ty }
}

fn line() -> SpiroCurve {
    SpiroCurve::from_points(
        vec![
            point(0.0, 0.0, SpiroCpTy::Open),
            point(10.0, 0.0, SpiroCpTy::EndOpen),
        ],
        false,
    )
}

fn s_curve() -> SpiroCurve {
    SpiroCurve::from_points(
        vec![
            point(1.0, 0.85, SpiroCpTy::Open),
            point(0.5, 1.0, SpiroCpTy::G4),
            point(0.0, 0.75, SpiroCpTy::G4),
            point(0.5, 0.5, SpiroCpTy::G4),
            point(1.0, 0.25, SpiroCpTy::G4),
            point(0.5, 0.0, SpiroCpTy::G4),
            point(0.0, 0.15, SpiroCpTy::EndOpen),
        ],
        false,
    )
}

fn nearest(contours: &[CubicBezier], point: Point) -> f64 {
    contours
        .iter()
        .flat_map(|contour| contour.to_kurbo().segments().collect::<Vec<_>>())
        .map(|seg| seg.nearest(point, 1e-9).distance_sq)
        .fold(f64::INFINITY, f64::min)
        .sqrt()
}

fn winding(contours: &[CubicBezier], point: Point) -> i32 {
    contours
        .iter()
        .map(|contour| contour.to_kurbo().winding(point))
        .sum()
}

#[test]
fn directional_alignments_keep_butt_caps_and_clockwise_winding() {
    for (alignment, bottom, top) in [(0.0, -2.0, 0.0), (0.5, -1.0, 1.0), (1.0, 0.0, 2.0)] {
        let mut curve = line();
        curve.alignment.insert(0, alignment);
        let contours = stroke_spiro(&curve, 2.0, &mut ()).unwrap();
        assert_eq!(contours.len(), 1);
        let path = contours[0].to_kurbo();
        let bounds = path.bounding_box();
        assert!((bounds.x0 - 0.0).abs() < 1e-9);
        assert!((bounds.x1 - 10.0).abs() < 1e-9);
        assert!((bounds.y0 - bottom).abs() < 1e-9);
        assert!((bounds.y1 - top).abs() < 1e-9);
        assert!(path.area() < 0.0);
    }
}

#[test]
fn variable_width_and_alignment_interpolate_along_the_curve() {
    let mut curve = line();
    curve.width_factors.extend([(0, 0.5), (1, 1.5)]);
    curve.alignment.extend([(0, 0.0), (1, 1.0)]);
    let contours = stroke_spiro(&curve, 2.0, &mut ()).unwrap();
    for expected in [
        Point::new(0.0, 0.0),
        Point::new(0.0, -1.0),
        Point::new(5.0, 1.0),
        Point::new(5.0, -1.0),
        Point::new(10.0, 3.0),
        Point::new(10.0, 0.0),
    ] {
        assert!(nearest(&contours, expected) < 5e-4, "{expected:?}");
    }
    assert!(winding(&contours, Point::new(5.0, 0.0)) != 0);
}

#[test]
fn varying_alignment_of_a_straight_stroke_stays_straight() {
    let mut curve = line();
    curve.alignment.extend([(0, 0.0), (1, 1.0)]);
    let contours = stroke_spiro(&curve, 2.0, &mut ()).unwrap();
    assert!(
        contours[0].segments.iter().all(CubicSegment::is_line),
        "{:?}",
        contours[0]
    );
    assert!(nearest(&contours, Point::new(5.0, 1.0)) < 1e-5);
    assert!(nearest(&contours, Point::new(5.0, -1.0)) < 1e-5);
}

#[test]
fn variable_stroke_can_taper_to_zero_width() {
    let mut curve = line();
    curve.width_factors.extend([(0, 1.0), (1, 0.0)]);
    let contours = stroke_spiro(&curve, 2.0, &mut ()).unwrap();
    assert_eq!(contours.len(), 1);
    assert!(nearest(&contours, Point::new(10.0, 0.0)) < 1e-5);
    assert_ne!(winding(&contours, Point::new(5.0, 0.25)), 0);
    assert_eq!(winding(&contours, Point::new(9.0, 0.25)), 0);
}

#[test]
fn fractional_alignment_and_heading_are_preserved() {
    let mut curve = line();
    curve.alignment.insert(0, 0.25);
    curve.tangents.insert(0, Point::new(1.0, 1.0));
    let contours = stroke_spiro(&curve, 2.0, &mut ()).unwrap();
    assert!(nearest(&contours, Point::new(-0.5, 0.5)) < 1e-4);
    assert!(nearest(&contours, Point::new(1.5, -1.5)) < 1e-4);
    assert!(nearest(&contours, Point::new(10.0, 0.5)) < 1e-4);
    assert!(nearest(&contours, Point::new(10.0, -1.5)) < 1e-4);
}

#[test]
fn s_curve_offsets_follow_the_centerline_between_knots() {
    let curve = s_curve();
    let centerlines = spiro_to_cube(&curve.points).unwrap();
    let contours = stroke_spiro(&curve, 0.2, &mut ()).unwrap();
    for segment in centerlines[0].segment_iter() {
        if let PathSeg::Cubic(cubic) = segment.into() {
            for t in [0.2, 0.5, 0.8] {
                let center = cubic.eval(t);
                let normal = cubic.deriv().eval(t).to_vec2().normalize().turn_90();
                for side in [-1.0, 1.0] {
                    let expected = center + side * 0.1 * normal;
                    assert!(
                        nearest(&contours, expected) < 3e-4,
                        "offset missed {expected:?}"
                    );
                }
            }
        }
    }
}

#[test]
fn variable_s_curve_tracks_width_and_alignment_between_knots() {
    let mut curve = s_curve();
    let last = curve.len() - 1;
    curve.width_factors.extend([(0, 0.6), (last, 1.4)]);
    curve.alignment.extend([(0, 0.0), (last, 1.0)]);
    let centerlines = spiro_to_cube(&curve.points).unwrap();
    let segments: Vec<PathSeg> = centerlines[0].segment_iter().map(Into::into).collect();
    let total: f64 = segments.iter().map(|seg| seg.arclen(1e-7)).sum();
    let contours = stroke_spiro(&curve, 0.1, &mut ()).unwrap();
    let mut preceding = 0.0;
    for segment in segments {
        let length = segment.arclen(1e-7);
        if let PathSeg::Cubic(cubic) = segment {
            for t in [0.2, 0.5, 0.8] {
                let progress = (preceding + cubic.subsegment(0.0..t).arclen(1e-7)) / total;
                let width = 0.1 * (0.6 + 0.8 * progress);
                let center = cubic.eval(t);
                let normal = cubic.deriv().eval(t).to_vec2().normalize().turn_90();
                for distance in [width * progress, width * (progress - 1.0)] {
                    let expected = center + distance * normal;
                    assert!(
                        nearest(&contours, expected) < 8e-5,
                        "variable offset missed {expected:?}"
                    );
                }
            }
        }
        preceding += length;
    }
}

#[test]
fn closed_strokes_keep_holes_and_directional_alignment() {
    for reverse in [false, true] {
        let mut points = vec![
            point(0.0, 0.0, SpiroCpTy::Corner),
            point(4.0, 0.0, SpiroCpTy::Corner),
            point(4.0, 4.0, SpiroCpTy::Corner),
            point(0.0, 4.0, SpiroCpTy::Corner),
        ];
        if reverse {
            points.reverse();
        }
        let mut curve = SpiroCurve::from_points(points, true);
        curve.alignment.insert(0, 1.0);
        let contours = stroke_spiro(&curve, 0.5, &mut ()).unwrap();
        assert_eq!(contours.len(), 2);
        let mut areas: Vec<_> = contours.iter().map(|c| c.to_kurbo().area()).collect();
        areas.sort_by(|a, b| a.abs().total_cmp(&b.abs()));
        assert!(areas[0] > 0.0 && areas[1] < 0.0, "{areas:?}");
        assert_eq!(winding(&contours, Point::new(2.0, 2.0)), 0);
        let inside = winding(&contours, Point::new(0.25, 2.0)) != 0;
        let outside = winding(&contours, Point::new(-0.25, 2.0)) != 0;
        assert_eq!((inside, outside), (!reverse, reverse));
    }
}

#[test]
fn closed_variable_stroke_preserves_its_hole() {
    let mut curve = SpiroCurve::from_points(
        vec![
            point(0.0, 0.0, SpiroCpTy::Corner),
            point(4.0, 0.0, SpiroCpTy::Corner),
            point(4.0, 4.0, SpiroCpTy::Corner),
            point(0.0, 4.0, SpiroCpTy::Corner),
        ],
        true,
    );
    curve.width_factors.extend([(0, 0.5), (2, 1.5)]);
    let contours = stroke_spiro(&curve, 1.0, &mut ()).unwrap();
    assert_eq!(contours.len(), 2);
    assert_eq!(winding(&contours, Point::new(2.0, 2.0)), 0);
    assert_ne!(winding(&contours, Point::new(2.0, -0.2)), 0);
    assert_ne!(winding(&contours, Point::new(2.0, 0.2)), 0);
    assert_eq!(winding(&contours, Point::new(2.0, -1.0)), 0);
}

#[test]
fn invalid_stroke_attributes_are_reported() {
    let mut curve = line();
    assert!(stroke_spiro(&curve, -1.0, &mut ()).is_err());
    assert!(stroke_spiro(&curve, f64::NAN, &mut ()).is_err());
    assert!(stroke_spiro(&curve, 0.0, &mut ()).unwrap().is_empty());
    curve.width_factors.insert(0, -0.5);
    assert!(stroke_spiro(&curve, 1.0, &mut ()).is_err());
    curve.width_factors.clear();
    curve.tangents.insert(0, Point::new(0.0, 1.0));
    assert!(stroke_spiro(&curve, 1.0, &mut ()).is_err());
    curve.tangents.clear();
    curve.alignment.insert(0, f64::MAX);
    assert!(stroke_spiro(&curve, 2.0, &mut ()).is_err());
}
