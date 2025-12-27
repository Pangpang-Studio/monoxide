//! Outlining curves.

use std::collections::HashMap;

use itertools::Itertools;
use monoxide_spiro::{SpiroCp, SpiroCpTy};

use crate::{
    CubicBezier, CubicSegment, SpiroCurve,
    cube::CubicSegmentFull,
    debug::CurveDebugger,
    error::{Error, Result},
    point::Point2D,
    stroke::{
        tangents::{make_line_join, move_point_normal_both},
        widths::solve_stroke_attrs,
    },
};

/// Represent the in and out tangent at a control point. Both tangents are
/// represented in the out direction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tangent {
    Continuous(Point2D),
    Corner { in_: Point2D, out: Point2D },
}

/// Test if two normalized tangents are approximately the same direction.
fn approx_eq(tan1: Point2D, tan2: Point2D) -> bool {
    tan1.dot(tan2) > 0.99
}

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

pub type TangentOverride = HashMap<usize, Tangent>;

/// The result of the stroke operation. It can be either a single spiro curve,
/// or two curves if the input curve is closed.
#[derive(Debug, Clone)]
pub enum StrokedSpiroCurve {
    One(SpiroCurve),
    Two(SpiroCurve, SpiroCurve),
}

/// Stroke a spiro curve. Returns a single spiro curve representing the stroke.
/// The curve might contain more than one segment if the input curve is closed,
/// or has multiple disconnected parts.
///
/// The result should be fed into another pass removing self-loops.
pub fn stroke_spiro(
    curve: &SpiroCurve,
    width: f64,
    dbg: &mut impl CurveDebugger,
) -> Result<StrokedSpiroCurve> {
    // This ensures that the `curve` has >= 2 points, so operations like `[0]` and
    // `.last().unwrap()` should be safe.
    ensure_single_piece(&curve.points)?;

    let is_closed = curve.points[0].ty != SpiroCpTy::Open;
    let (left, mut right) = stroke_spiro_raw(curve, is_closed, width, dbg)?;

    // Anyway, we should reverse the right curve first.
    right.reverse();
    for cp in &mut right {
        *cp = reverse_spiro_point(*cp);
    }

    // Both `left` and `right` should be either:
    //
    // - A closed curve. In this case, the two curves are simply concatenated (the
    //   right one reversed because we need to decrease the winding number).
    if is_closed {
        return Ok(StrokedSpiroCurve::Two(
            SpiroCurve::from_points(left, true),
            SpiroCurve::from_points(right, true),
        ));
    }

    // - [Open, ..., EndOpen], i.e. an open curve. In this case, we need to replace
    //   all `Open` and `EndOpen` with `Corner`, except the first one which should
    //   be `End`.
    let mut result = left;
    debug_assert_eq!(result[0].ty, SpiroCpTy::Open);
    result[0].ty = SpiroCpTy::Corner;
    // Rewrite the last point
    debug_assert_eq!(result.last().unwrap().ty, SpiroCpTy::EndOpen);
    result.last_mut().unwrap().ty = SpiroCpTy::Corner;
    // and those in the right curve (remember we have reversed it)
    debug_assert_eq!(right[0].ty, SpiroCpTy::Open);
    right[0].ty = SpiroCpTy::Corner;
    debug_assert_eq!(right.last().unwrap().ty, SpiroCpTy::EndOpen);
    right.last_mut().unwrap().ty = SpiroCpTy::Corner;

    result.extend(right);

    Ok(StrokedSpiroCurve::One(SpiroCurve::from_points(
        result, true,
    )))
}

#[allow(dead_code)]
fn debug_spiro_points<C: CurveDebugger>(
    cube_curve: &CubicBezier<Point2D>,
    spiro_indices: &[usize],
    dbg: &mut C,
) {
    let mut spiro_index = 0;
    // indices are segment indices
    for (idx, CubicSegmentFull { start, rest: seg }) in cube_curve.segment_iter().enumerate() {
        if idx == 0 {
            if spiro_indices[spiro_index] == 0 {
                dbg.point(
                    crate::debug::DebugPointKind::Curve,
                    start,
                    format_args!("spiro(0)"),
                );
            }
            spiro_index += 1;
        }

        // start is irrelevant for the rest of the segments
        let spiro_point_idx = if spiro_indices[spiro_index] == idx + 1 {
            spiro_index += 1;
            Some(spiro_index - 1)
        } else {
            None
        };

        let tag = |dbg: &mut C, at| match spiro_point_idx {
            Some(idx) => dbg.point(
                crate::debug::DebugPointKind::Curve,
                at,
                format_args!("spiro({idx})"),
            ),
            None => dbg.point(crate::debug::DebugPointKind::Curve, at, format_args!("")),
        };
        match seg {
            CubicSegment::Line(end) => {
                tag(dbg, end);
                dbg.line(start, end, format_args!(""));
            }
            CubicSegment::Curve(p1, p2, p3) => {
                dbg.point(crate::debug::DebugPointKind::Control, p1, format_args!(""));
                dbg.point(crate::debug::DebugPointKind::Control, p2, format_args!(""));
                tag(dbg, p3);
                dbg.line(start, p1, format_args!(""));
                dbg.line(p2, p3, format_args!(""));
            }
        }
    }
}

/// Stroke a spiro curve. Returns two spiro curves of the inner and outer
/// boundaries of the stroke. The spiro curve should be _one_ continuous curve,
/// with no gaps.
///
/// The result should be fed into another pass removing self-loops.
fn stroke_spiro_raw(
    curve: &SpiroCurve,
    is_closed: bool,
    width: f64,
    dbg: &mut impl CurveDebugger,
) -> Result<(Vec<SpiroCp>, Vec<SpiroCp>)> {
    if curve.points.is_empty() {
        return Err(Error::SpiroBroken);
    }

    // Before stroking the curve, we first need to determine the normal
    // direction at each control point. To make things simpler, we just convert
    // the curve into bezier segments and extract the normal from them.
    let (curves, indices) = crate::convert::spiro_to_cube_with_indices(&curve.points)?;

    // There will be only one curve, since we know the spiro curve is a single
    // piece.
    let cubic = curves.into_iter().next().ok_or_else(|| {
        Error::internal("spiro-to-cube conversion should return at least one curve")
    })?;
    // And we can transform the indices into a vector of raw indices too
    //
    // Semantics: each index means the point corresponds to the **start** of the
    // given segment, i.e. the i-th on-curve point of the cubic bezier.
    let mut indices = indices.into_iter().map(|x| x.segment_index).collect_vec();

    if !is_closed {
        // open curves don't have the last point logged, so we add it
        // manually.
        let last_index = cubic.segment_count();
        indices.push(last_index);
    }

    // debug_spiro_points(&cubic, &indices, dbg);

    // Determine the stroke attributes (width factors, alignments, etc.) at each
    // control point.
    let stroke_attrs = solve_stroke_attrs(curve, &cubic, &indices)?;

    // Calculate tangent for each control point.
    let actual_tangents = tangents::calc_tangents(&curve.points, &cubic, &indices);

    // the curve on the left side of the stroke
    let mut left_curve = Vec::new();
    // the curve on the right side of the stroke
    let mut right_curve = Vec::new();

    if curve.points.len() != actual_tangents.len() {
        return Err(Error::internal(
            "`curve` and `actual_tangents` do not have the same length",
        ));
    }
    for (idx, (&cp, tangent)) in curve.points.iter().zip(actual_tangents).enumerate() {
        let tangent_override = curve.tangents.get(&idx);
        let (tangent, tan_width_factor) = match tangent {
            Tangent::Continuous(tan) => {
                if let Some(tangent_override) = tangent_override {
                    let dir_orig = tan.x.atan2(tan.y);
                    let dir_override = tangent_override.x.atan2(tangent_override.y);
                    let angle_diff = (dir_override - dir_orig).abs();
                    let angle_diff = if angle_diff > std::f64::consts::PI {
                        2.0 * std::f64::consts::PI - angle_diff
                    } else {
                        angle_diff
                    };
                    let width_factor = 1.0 / angle_diff.cos();
                    (Tangent::Continuous(*tangent_override), width_factor)
                } else {
                    (tangent, 1.0)
                }
            }
            Tangent::Corner { .. } => (tangent, 1.0),
            // TODO: warn about cannot override corner
        };
        let stroke_width_factor = stroke_attrs.width_factors[idx];
        let width = width * stroke_width_factor * tan_width_factor;

        let left_offset_factor = stroke_attrs.alignments[idx];
        let right_offset_factor = 1.0 - left_offset_factor;

        let left_offset = width * left_offset_factor;
        let right_offset = width * right_offset_factor;

        let (left, right) = determine_stroked_points(left_offset, right_offset, cp, tangent);
        dbg.line(cp.into(), right, format_args!(""));
        push_point(left, right, cp, &mut left_curve, &mut right_curve);
    }

    Ok((left_curve, right_curve))
}

fn determine_stroked_points(
    left_offset: f64,
    right_offset: f64,
    cp: SpiroCp,
    tangent: Tangent,
) -> (Point2D, Point2D) {
    match tangent {
        Tangent::Continuous(tangent) => {
            move_point_normal_both(cp.into(), tangent, left_offset, right_offset)
        }
        Tangent::Corner { in_, out } => make_line_join(cp, in_, out, left_offset, right_offset),
    }
}

fn push_point(
    left: Point2D,
    right: Point2D,
    orig: SpiroCp,
    left_curve: &mut Vec<SpiroCp>,
    right_curve: &mut Vec<SpiroCp>,
) {
    left_curve.push(SpiroCp {
        x: left.x,
        y: left.y,
        ty: orig.ty,
    });
    right_curve.push(SpiroCp {
        x: right.x,
        y: right.y,
        ty: orig.ty,
    });
}

fn reverse_spiro_point(cp: SpiroCp) -> SpiroCp {
    let ty = match cp.ty {
        SpiroCpTy::Left => SpiroCpTy::Right,
        SpiroCpTy::Right => SpiroCpTy::Left,
        SpiroCpTy::EndOpen => SpiroCpTy::Open,
        SpiroCpTy::Open => SpiroCpTy::EndOpen,

        x => x,
    };
    SpiroCp { ty, ..cp }
}

mod tangents;
mod widths;
