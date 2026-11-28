use kurbo::{BezPath, Circle, ParamCurve, ParamCurveDeriv, Point, Shape};
use monoxide_curves::{
    CubicBezier, SpiroCurve,
    convert::spiro_to_cube,
    stroke::{stroke_bezier, stroke_spiro},
};
use monoxide_spiro::SpiroCpTy::{Corner, EndOpen, G4, Open};

fn path(contours: &[CubicBezier]) -> BezPath {
    assert!(contours.iter().all(|c| c.closed));
    contours.iter().flat_map(CubicBezier::to_kurbo).collect()
}

#[test]
fn stroke_follows_both_bends_between_spiro_knots() {
    // Refitting displaced knots can cut across the original S instead of
    // keeping the authored distance from its centerline between knots.
    let mut spine = SpiroCurve::from(vec![
        Open.at((0., 0.)),
        G4.at((0.25, 0.12)),
        G4.at((0.5, 0.)),
        G4.at((0.75, -0.12)),
        EndOpen.at((1., 0.)),
    ]);
    spine.alignment.insert(1, 0.25);
    let ink = path(&stroke_spiro(&spine, 0.04).unwrap());
    let cubics = spiro_to_cube(&spine.points).unwrap();
    for seg in cubics[0].to_kurbo().segments() {
        let cubic = seg.to_cubic();
        for t in [0.2, 0.5, 0.8] {
            let p = cubic.eval(t);
            let normal = cubic.deriv().eval(t).to_vec2().normalize().turn_90();
            for d in [-0.029, 0.009] {
                assert_eq!(ink.winding(p + normal * d), -1);
            }
            for d in [-0.031, 0.011] {
                assert_eq!(ink.winding(p + normal * d), 0, "p={p:?} d={d}, seg={seg:?}");
            }
        }
    }
}

#[test]
fn tapered_diagonal_keeps_its_authored_caps() {
    // The slash through zero meets the bowl on vertical caps, including
    // when the end widths differ and the heading opposes travel.
    let mut line = SpiroCurve::from(vec![Open.at((0., 0.)), EndOpen.at((1., 0.5))]);
    line.tangents
        .extend([(0, (-1., 0.).into()), (1, (-1., 0.).into())]);
    line.width_factors.extend([(0, 0.5), (1, 1.)]);
    line.alignment.extend([(0, 0.), (1, 1.)]);
    let band = path(&stroke_spiro(&line, 0.1).unwrap());
    let bounds = band.bounding_box();
    assert!(bounds.x0.abs() < 1e-8 && (bounds.x1 - 1.).abs() < 1e-8);
    assert!((band.area() + 0.075 * 1.25_f64.sqrt()).abs() < 1e-8);
}

#[test]
fn closed_alignment_depends_on_travel_direction() {
    let mut square = SpiroCurve::from(vec![
        Corner.at((0., 0.)),
        Corner.at((1., 0.)),
        Corner.at((1., 1.)),
        Corner.at((0., 1.)),
    ]);
    square.alignment.insert(0, 1.);
    for inside in [true, false] {
        let contours = stroke_spiro(&square, 0.1).unwrap();
        assert_eq!(contours.len(), 2);
        let ink = path(&contours);
        assert_eq!(ink.winding(Point::new(0.5, 0.5)), 0);
        assert_eq!(ink.winding(Point::new(0.5, 0.05)) == -1, inside);
        assert_eq!(ink.winding(Point::new(0.5, -0.05)) == -1, !inside);
        square.points.reverse();
    }
}

#[test]
fn wide_closed_bezier_has_no_inverted_counter() {
    let circle = Circle::new((0., 0.), 0.1).to_path(0.0001);
    let ink = path(&stroke_bezier(&CubicBezier::from_kurbo(&circle), 0.3).unwrap());
    assert_eq!(ink.winding(Point::ORIGIN), -1);
    assert_eq!(ink.winding(Point::new(0.2, 0.)), -1);
    assert_eq!(ink.winding(Point::new(0.3, 0.)), 0);
}

#[test]
fn curved_taper_changes_sides_at_its_middle_knot() {
    // As on an S, the spine starts on one edge, crosses the band, and
    // finishes on the other edge. The middle knot also carries more ink.
    let mut arc = SpiroCurve::from(vec![
        Open.at((0., 0.)),
        G4.at((0.5, 0.25)),
        EndOpen.at((1., 0.)),
    ]);
    arc.alignment.extend([(0, 0.), (2, 1.)]);
    arc.width_factors.extend([(0, 1.), (1, 2.), (2, 1.)]);
    let ink = path(&stroke_spiro(&arc, 0.1).unwrap());
    for y in [0.151, 0.349] {
        assert_eq!(ink.winding(Point::new(0.5, y)), -1);
    }
    for y in [0.149, 0.351] {
        assert_eq!(ink.winding(Point::new(0.5, y)), 0);
    }
    let centerline = spiro_to_cube(&arc.points).unwrap()[0].to_kurbo();
    let segments: Vec<_> = centerline.segments().collect();
    for (seg, t, side) in [
        (segments[0], 0.1, -1.),
        (*segments.last().unwrap(), 0.9, 1.),
    ] {
        let cubic = seg.to_cubic();
        let p = cubic.eval(t);
        let normal = cubic.deriv().eval(t).to_vec2().normalize().turn_90();
        assert_eq!(ink.winding(p + normal * (0.05 * side)), -1);
        assert_eq!(ink.winding(p - normal * (0.05 * side)), 0);
    }
}
