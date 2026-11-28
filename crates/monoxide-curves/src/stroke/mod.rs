//! Outlining curves.

use kurbo::{ParamCurve, PathSeg};
use monoxide_spiro::{SpiroCp, SpiroCpTy};

use self::offset::Knot;
use crate::{
    CubicBezier,
    debug::CurveDebugger,
    error::{Error, Result},
    spiro::{SpiroCurve, default_alignment},
};

mod cap;
mod offset;
mod widths;

fn ensure_single_piece(curve: &[SpiroCp]) -> Result<()> {
    use SpiroCpTy::*;

    let is_single_piece = match curve {
        [] | [_] => false, // No curve
        [
            SpiroCp { ty: Open, .. },
            open @ ..,
            SpiroCp { ty: EndOpen, .. },
        ] => !open.iter().any(|cp| matches!(cp.ty, Open | EndOpen)),
        [SpiroCp { ty: Open, .. }, ..] | [SpiroCp { ty: EndOpen, .. }, ..] => false,
        closed => !closed.iter().any(|cp| matches!(cp.ty, Open | EndOpen)),
    };
    if !is_single_piece {
        return Err(Error::SpiroBroken);
    }
    Ok(())
}

/// Stroke one Spiro contour into closed cubic boundaries. Width and alignment
/// are independent, piecewise-linear functions of centerline arc length.
pub fn stroke_spiro(
    curve: &SpiroCurve,
    width: f64,
    dbg: &mut impl CurveDebugger,
) -> Result<Vec<CubicBezier>> {
    ensure_single_piece(&curve.points)?;
    let (cubics, markers) = crate::convert::spiro_to_cube_with_indices(&curve.points)?;
    let [cubic] = &cubics[..] else {
        return Err(Error::SpiroBroken);
    };
    let mut indices: Vec<_> = markers.into_iter().map(|k| k.segment_index).collect();
    if !cubic.closed {
        // Spiro emits markers only for the starts of segments.
        indices.push(cubic.segment_count());
    }
    if indices.len() != curve.len() {
        return Err(Error::internal("expected one cubic index per Spiro knot"));
    }
    let attrs = widths::solve_stroke_attrs(curve, cubic.closed, cubic, &indices)?;
    let knots: Vec<_> = indices
        .into_iter()
        .enumerate()
        .map(|(i, segment)| Knot {
            point: curve.points[i].proj(kurbo::Point::new),
            segment,
            width: width * attrs.width_factors[i],
            alignment: attrs.alignments[i],
            heading: curve.tangents.get(&i).map(|p| p.to_vec2()),
        })
        .collect();
    let segments: Vec<_> = cubic.segment_iter().map(PathSeg::from).collect();
    Ok(offset::outline(&segments, knots, cubic.closed, dbg))
}

/// Stroke a cubic path with the default centered alignment and butt caps.
pub fn stroke_bezier(
    curve: &CubicBezier,
    width: f64,
    dbg: &mut impl CurveDebugger,
) -> Vec<CubicBezier> {
    let mut segments: Vec<_> = curve.segment_iter().map(PathSeg::from).collect();
    if curve.closed
        && let Some(last) = segments.last()
        && last.end() != curve.start
    {
        segments.push(PathSeg::Line(kurbo::Line::new(last.end(), curve.start)));
    }
    let knots: Vec<_> = segments
        .iter()
        .map(ParamCurve::start)
        .chain(
            segments
                .last()
                .filter(|_| !curve.closed)
                .map(ParamCurve::end),
        )
        .enumerate()
        .map(|(segment, point)| Knot {
            point,
            segment,
            width,
            alignment: default_alignment(),
            heading: None,
        })
        .collect();
    offset::outline(&segments, knots, curve.closed, dbg)
}

#[cfg(test)]
mod tests {
    use kurbo::{ParamCurveArclen, ParamCurveDeriv, ParamCurveNearest, Point, Shape};

    use super::*;

    fn assert_on_outline(outlines: &[CubicBezier], expected: Point) {
        let distance = outlines
            .iter()
            .flat_map(|c| c.to_kurbo().segments().collect::<Vec<_>>())
            .map(|s| s.nearest(expected, 1e-9).distance_sq.sqrt())
            .fold(f64::INFINITY, f64::min);
        assert!(
            distance < 2e-5,
            "{expected:?} is {distance} from the outline"
        );
    }

    // Check the interior of each cubic, where re-solving a Spiro through
    // displaced knots used to lose both the width and the alignment.
    #[test]
    fn curved_stroke_tracks_two_independent_profiles() {
        use SpiroCpTy::*;
        let mut curve = SpiroCurve::from(vec![
            Open.at((0., 0.)),
            G4.at((0.3, 0.4)),
            G4.at((0.8, 0.15)),
            EndOpen.at((1.2, 0.6)),
        ]);
        curve.width_factors.extend([(0, 0.5), (3, 1.5)]);
        curve.alignment.extend([(0, -0.25), (3, 1.25)]);
        let outline = stroke_spiro(&curve, 0.04, &mut ()).unwrap();
        let center = crate::convert::spiro_to_cube(&curve.points).unwrap();
        let segments: Vec<_> = center[0].segment_iter().map(PathSeg::from).collect();
        let length: f64 = segments.iter().map(|s| s.arclen(1e-10)).sum();
        let mut travelled = 0.;
        for seg in segments {
            for t in [0.17, 0.49, 0.83] {
                let u = (travelled + seg.subsegment(0.0..t).arclen(1e-10)) / length;
                let width = 0.04 * (0.5 + u);
                let alignment = -0.25 + 1.5 * u;
                let normal = seg
                    .to_cubic()
                    .deriv()
                    .eval(t)
                    .to_vec2()
                    .normalize()
                    .turn_90();
                for side in [alignment, alignment - 1.] {
                    assert_on_outline(&outline, seg.eval(t) + normal * (width * side));
                }
            }
            travelled += seg.arclen(1e-10);
        }
    }

    #[test]
    fn diagonal_caps_keep_the_requested_heading_and_normal_width() {
        let mut curve = SpiroCurve::from(vec![
            SpiroCpTy::Open.at((0., 0.)),
            SpiroCpTy::EndOpen.at((3., 4.)),
        ]);
        curve
            .tangents
            .extend([(0, (0., 1.).into()), (1, (0., -1.).into())]);
        curve.alignment.insert(0, 0.25);
        let outline = stroke_spiro(&curve, 0.8, &mut ()).unwrap();
        for (x, y) in [(0., 0.), (3., 4.)] {
            for dx in [-0.25, 0.75] {
                assert_on_outline(&outline, Point::new(x + dx, y));
            }
        }
        assert!((outline[0].to_kurbo().area() + 4.).abs() < 1e-6);
    }

    // A tilted cap used to move an entire span's edge tangentially. Its inner
    // endpoint must be on the original offset, and its cut line must pass
    // through the authored knot.
    #[test]
    fn oblique_cap_intersects_the_curved_edge() {
        use SpiroCpTy::*;
        let mut curve = SpiroCurve::from(vec![
            Open.at((0., 0.)),
            G4.at((0.4, 0.4)),
            EndOpen.at((0.8, 0.)),
        ]);
        let center = crate::convert::spiro_to_cube(&curve.points)
            .unwrap()
            .remove(0);
        let tan = center.tangent_at(0, 0.).unwrap();
        let normal = center.normal_at(0, 0.).unwrap();
        let heading = (tan + normal * 0.8).normalize();
        curve.tangents.insert(0, heading.to_point());
        let outline = stroke_spiro(&curve, 0.1, &mut ()).unwrap();

        let right = |t| center.point_at(0, t) - center.normal_at(0, t).unwrap() * 0.05;
        let distance = |t| (right(t) - center.start).dot(heading);
        assert!(distance(0.) < 0. && distance(1.) > 0.);
        let (mut lo, mut hi) = (0.0_f64, 1.0_f64);
        for _ in 0..48 {
            let mid = lo.midpoint(hi);
            if distance(mid) < 0. {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        assert_on_outline(&outline, right(hi));
        assert_on_outline(&outline, right(hi.midpoint(1.)));
        assert_on_outline(&outline, center.start + normal * 0.05 - tan * 0.04);
        let cap = outline[0].segment_iter().any(|s| {
            s.rest.is_line()
                && (s.start - center.start).dot(heading).abs() < 1e-8
                && (s.rest.last_point() - center.start).dot(heading).abs() < 1e-8
                && s.start.distance(s.rest.last_point()) > 0.09
        });
        assert!(cap);
    }

    #[test]
    fn short_stroke_can_be_cut_on_the_opposite_extension() {
        use SpiroCpTy::*;
        let mut curve = SpiroCurve::from(vec![Open.at((0., 0.)), EndOpen.at((0.1, 0.))]);
        let heading = kurbo::Vec2::new(1., 1.).normalize().to_point();
        curve.tangents.extend([(0, heading), (1, heading)]);
        let outline = stroke_spiro(&curve, 0.5, &mut ()).unwrap();
        assert_eq!(outline.len(), 1);
        assert!((outline[0].to_kurbo().area() + 0.05).abs() < 1e-8);
        for p in [(-0.25, 0.25), (-0.15, 0.25), (0.25, -0.25), (0.35, -0.25)] {
            assert_on_outline(&outline, p.into());
        }
    }

    #[test]
    fn heading_at_a_corner_cuts_both_spans_on_the_same_line() {
        use SpiroCpTy::*;
        let mut curve = SpiroCurve::from(vec![
            Open.at((0., 0.)),
            Corner.at((0.5, 1.)),
            EndOpen.at((1., 0.)),
        ]);
        curve.tangents.insert(1, (0., 1.).into());
        curve.alignment.insert(1, 0.25);
        let outline = stroke_spiro(&curve, 0.1, &mut ()).unwrap();
        assert_eq!(outline.len(), 1);
        assert!((outline[0].to_kurbo().bounding_box().y1 - 1.).abs() < 2e-6);
        assert_on_outline(&outline, (0.5, 1.).into());
    }

    #[test]
    fn a_miter_keeps_the_varying_offset_on_its_incident_line() {
        use SpiroCpTy::*;
        let mut curve = SpiroCurve::from(vec![
            Open.at((0., 0.)),
            Corner.at((1., 0.)),
            EndOpen.at((1., 1.)),
        ]);
        curve.alignment.extend([(0, 0.1), (1, 0.9)]);
        let outline = stroke_spiro(&curve, 0.2, &mut ()).unwrap();
        for x in [0.25, 0.5, 0.75] {
            let left = 0.2 * (0.1 + 0.8 * x);
            assert_on_outline(&outline, (x, left).into());
            assert_on_outline(&outline, (x, left - 0.2).into());
        }
    }

    #[test]
    fn right_angle_keeps_fractional_miter_and_zero_width_tip() {
        use SpiroCpTy::*;
        let mut curve = SpiroCurve::from(vec![
            Open.at((0., 0.)),
            Corner.at((1., 0.)),
            EndOpen.at((1., 1.)),
        ]);
        curve.alignment.insert(0, 0.25);
        curve.width_factors.extend([(1, 1.), (2, 0.)]);
        let outline = stroke_spiro(&curve, 0.4, &mut ()).unwrap();
        for p in [(0.9, 0.1), (1.3, -0.3), (1., 1.)] {
            assert_on_outline(&outline, p.into());
        }
    }

    #[test]
    fn self_crossing_stroke_fills_the_intersection_once() {
        use SpiroCpTy::*;
        let curve = SpiroCurve::from(vec![
            Open.at((-1., -1.)),
            Corner.at((1., 1.)),
            Corner.at((-1., 1.)),
            EndOpen.at((1., -1.)),
        ]);
        let outline = stroke_spiro(&curve, 0.1, &mut ()).unwrap();
        let winding: i32 = outline
            .iter()
            .map(|o| o.to_kurbo().winding(Point::ORIGIN))
            .sum();
        assert_eq!(winding, -1);
    }

    #[test]
    fn cubic_input_handles_repeated_handles_and_an_implicit_closing_edge() {
        use crate::CubicSegment::{Curve, Line};
        let origin = Point::ORIGIN;
        let mut curve = CubicBezier {
            start: origin,
            segments: vec![
                Line(origin),
                Curve(origin, (0.8, 1.).into(), (1., 1.).into()),
                Line((1., 0.).into()),
            ],
            closed: true,
        };
        let outline = stroke_bezier(&curve, 0.04, &mut ());
        assert_eq!(outline.len(), 2);
        assert_on_outline(&outline, (0.5, -0.02).into());
        assert!(
            outline
                .iter()
                .all(|c| c.closed && c.iter().all(|(_, p)| p.is_finite()))
        );
        curve.segments.clear();
        assert!(stroke_bezier(&curve, 0.04, &mut ()).is_empty());
    }

    #[test]
    fn closed_stroke_preserves_both_boundaries() {
        let curve = SpiroCurve::from_points(
            vec![
                SpiroCp {
                    x: 0.70,
                    y: 1.10,
                    ty: SpiroCpTy::G4,
                },
                SpiroCp {
                    x: 0.50,
                    y: 1.20,
                    ty: SpiroCpTy::G4,
                },
                SpiroCp {
                    x: 0.30,
                    y: 1.10,
                    ty: SpiroCpTy::G4,
                },
                SpiroCp {
                    x: 0.10,
                    y: 0.80,
                    ty: SpiroCpTy::Left,
                },
                SpiroCp {
                    x: 0.10,
                    y: 0.20,
                    ty: SpiroCpTy::Right,
                },
                SpiroCp {
                    x: 0.30,
                    y: -0.10,
                    ty: SpiroCpTy::G4,
                },
                SpiroCp {
                    x: 0.50,
                    y: -0.20,
                    ty: SpiroCpTy::G4,
                },
                SpiroCp {
                    x: 0.70,
                    y: -0.10,
                    ty: SpiroCpTy::G4,
                },
                SpiroCp {
                    x: 0.90,
                    y: 0.20,
                    ty: SpiroCpTy::Left,
                },
                SpiroCp {
                    x: 0.90,
                    y: 0.80,
                    ty: SpiroCpTy::Right,
                },
            ],
            true,
        );

        let stroked = stroke_spiro(&curve, 0.08, &mut ()).expect("stroke should succeed");
        assert_eq!(stroked.len(), 2);
        for outline in &stroked {
            assert!(outline.closed);
            assert!(outline.segment_count() >= curve.len());
            assert!(outline.iter().all(|(_, point)| point.is_finite()));
        }
    }
}
