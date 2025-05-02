//! Outlining curves.

use std::collections::HashMap;

use monoxide_spiro::{SpiroCp, SpiroCpTy};

use crate::{CubicBezier, CubicSegment, debug::CurveDebugger, point::Point2D};

/// Represent the in and out tangent at a control point. Both tangents are
/// represented in the out direction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tangent {
    pub in_: Option<Point2D>,
    pub out: Option<Point2D>,
}

impl Tangent {
    /// Apply a tangent override to this tangent.
    ///
    /// If the override is `None`, the original value is used.
    /// If the original value is `Some(x1)` and the override is `Some(x2)`, the
    /// override is used.
    fn with_override(&self, override_: &Tangent) -> Tangent {
        Tangent {
            in_: self.in_.map(|x| override_.in_.unwrap_or(x)),
            out: self.out.map(|x| override_.out.unwrap_or(x)),
        }
    }
}

/// Test if two normalized tangents are approximately the same direction.
fn approx_eq(tan1: Point2D, tan2: Point2D) -> bool {
    tan1.dot(tan2) > 0.99
}

fn is_point_curved(ty: SpiroCpTy) -> bool {
    matches!(
        ty,
        SpiroCpTy::G2 | SpiroCpTy::G4 | SpiroCpTy::Left | SpiroCpTy::Right
    )
}

fn is_single_piece(curve: &[SpiroCp]) -> bool {
    use SpiroCpTy::*;

    match curve {
        [] => false, // No curve
        [
            SpiroCp { ty: Open, .. },
            open @ ..,
            SpiroCp { ty: EndOpen, .. },
        ] => !open.iter().any(|cp| matches!(cp.ty, Open | EndOpen)),
        [SpiroCp { ty: Open, .. }, ..] | [SpiroCp { ty: EndOpen, .. }, ..] => false,
        closed => !closed.iter().any(|cp| matches!(cp.ty, Open | EndOpen)),
    }
}

pub type TangentOverride = HashMap<usize, Tangent>;

/// The result of the stroke operation. It can be either a single spiro curve,
/// or two curves if the input curve is closed.
#[derive(Debug, Clone)]
pub enum StrokeResult {
    One(Vec<SpiroCp>),
    Two(Vec<SpiroCp>, Vec<SpiroCp>),
}

/// Stroke a spiro curve. Returns a single spiro curve representing the stroke.
/// The curve might contain more than one segment if the input curve is closed,
/// or has multiple disconnected parts.
///
/// The result should be fed into another pass removing self-loops.
pub fn stroke_spiro(
    curve: &[SpiroCp],
    width: f64,
    tangent_override: &TangentOverride,
    dbg: &mut impl CurveDebugger,
) -> StrokeResult {
    if !is_single_piece(curve) {
        panic!("Spiro curve is not valid: not single piece");
    }

    let is_closed = curve[0].ty != SpiroCpTy::Open;
    let (left, mut right) = stroke_spiro_raw(curve, is_closed, width, tangent_override, dbg);

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
        return StrokeResult::Two(left, right);
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

    StrokeResult::One(result)
}

#[allow(dead_code)]
fn debug_spiro_points(
    cube_curve: &CubicBezier<Point2D>,
    spiro_indices: &[usize],
    dbg: &mut impl CurveDebugger,
) {
    let mut spiro_index = 0;
    // indices are segment indices
    for (idx, (start, seg)) in cube_curve.segment_iter().enumerate() {
        if idx == 0 {
            if spiro_indices[spiro_index] == 0 {
                dbg.point(crate::debug::DebugPointKind::Curve, start, "spiro(0)");
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
        let tag = match spiro_point_idx {
            Some(idx) => format!("spiro({})", idx),
            None => String::new(),
        };

        match seg {
            CubicSegment::Line(end) => {
                dbg.point(crate::debug::DebugPointKind::Curve, end, &tag);
            }
            CubicSegment::Curve(p1, p2, p3) => {
                dbg.point(crate::debug::DebugPointKind::Control, p1, "");
                dbg.point(crate::debug::DebugPointKind::Control, p2, "");
                dbg.point(crate::debug::DebugPointKind::Curve, p3, &tag);
                dbg.line(start, p1, "");
                dbg.line(p2, p3, "");
            }
        }
    }
}

// TODO: allow a variable width, and position within the stroke.
/// Stroke a spiro curve. Returns two spiro curves of the inner and outer
/// boundaries of the stroke. The spiro curve should be _one_ continuous curve,
/// with no gaps.
///
/// The result should be fed into another pass removing self-loops.
pub fn stroke_spiro_raw(
    curve: &[SpiroCp],
    is_closed: bool,
    width: f64,
    tangent_override: &TangentOverride,
    dbg: &mut impl CurveDebugger,
) -> (Vec<SpiroCp>, Vec<SpiroCp>) {
    assert!(!curve.is_empty(), "curve should not be empty");

    // Before stroking the curve, we first need to determine the normal
    // direction at each control point. To make things simpler, we just convert
    // the curve into bezier segments and extract the normal from them.
    let (curves, indices) = crate::convert::spiro_to_cube_with_indices(curve);

    // There will be only one curve, since we know the spiro curve is a single
    // piece.
    let cubic = curves.into_iter().next().unwrap();
    // And we can transform the indices into a vector of raw indices too
    let mut indices = indices
        .into_iter()
        .map(|x| x.segment_index)
        .collect::<Vec<_>>();

    if !is_closed {
        // open curves don't have the last point logged, so we add it
        // manually.
        let last_index = cubic.segment_count();
        indices.push(last_index);
    }

    // debug_spiro_points(&cubic, &indices, dbg);

    // Calculate tangent for each control point.
    let mut tangents = calc_tangents(curve, &cubic, &indices);
    for (&index, override_) in tangent_override {
        tangents[index] = tangents[index].with_override(override_);
    }
    let tangents = tangents;

    let half_width = width / 2.0;
    let left_offset = half_width;
    let right_offset = half_width;

    // the curve on the left side of the stroke
    let mut left_curve = Vec::new();
    // the curve on the right side of the stroke
    let mut right_curve = Vec::new();

    assert_eq!(
        curve.len(),
        tangents.len(),
        "`curve` and `tangents` do not have the same length"
    );
    for (&cp, tangent) in curve.iter().zip(tangents) {
        if is_point_curved(cp.ty) {
            // This is a curved point, so it should be moved in its normal
            // direction, and should not be split.
            // We'll just use the in tangent for now.
            let tangent = tangent.in_.expect("curved point should have in tangent");
            let (left, right) =
                move_point_normal_both(cp.into(), tangent, left_offset, right_offset);

            dbg.line(cp.into(), right, "");

            push_point(left, right, cp, &mut left_curve, &mut right_curve);
        } else {
            // This is not a curved point. If the in and out tangents are
            // approximately the same direction, we can just move the point in
            // that direction. Otherwise, we need to split the point into two
            // points, one for each tangent, and bridge them with a straight
            // cap.
            //
            // It also might not have in and out tangents, in which case we just
            // move it in whatever direction we can.
            let (left, right) = match tangent {
                Tangent {
                    in_: None,
                    out: None,
                } => {
                    panic!("point should have at least one tangent")
                }

                Tangent {
                    in_: Some(in_),
                    out: Some(out),
                } if approx_eq(in_, out) => {
                    // two tangents are equal, just use the common tangent
                    move_point_normal_both(cp.into(), in_, left_offset, right_offset)
                }
                Tangent {
                    in_: Some(in_),
                    out: Some(out),
                } => make_line_join(cp, in_, out, left_offset, right_offset),

                // Single-sided tangents
                Tangent {
                    in_: Some(in_),
                    out: None,
                } => move_point_normal_both(cp.into(), in_, left_offset, right_offset),
                Tangent {
                    in_: None,
                    out: Some(out),
                } => move_point_normal_both(cp.into(), out, left_offset, right_offset),
            };

            dbg.line(cp.into(), right, "");

            push_point(left, right, cp, &mut left_curve, &mut right_curve);
        }
    }

    (left_curve, right_curve)
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

/// Calculate the tangent at each spiro control point. Returns a vector of
/// normalized tangent vectors, one for each control point.
fn calc_tangents(
    curve: &[SpiroCp],
    cube_curve: &CubicBezier<Point2D>,
    indices: &[usize],
) -> Vec<Tangent> {
    assert_eq!(
        curve.len(),
        indices.len(),
        "`curve` and `indices` do not have the same length"
    );
    let mut tangents = Vec::new();

    for &index in indices {
        let out_seg = if index < cube_curve.segments.len() {
            Some(cube_curve.segment(index).expect("segment exists"))
        } else if cube_curve.closed {
            Some(
                cube_curve
                    .segment(0)
                    .expect("closed curve has at least one segment"),
            )
        } else {
            None
        };
        let out_tangent = match out_seg {
            Some((p1, CubicSegment::Curve(c1, _, _))) => Some(c1 - p1),
            Some((p1, CubicSegment::Line(p2))) => Some(p2 - p1),
            None => None,
        }
        .map(Point2D::normalize);

        let in_seg = if index != 0 {
            Some(cube_curve.segment(index - 1).expect("segment exists"))
        } else if cube_curve.closed {
            Some(
                cube_curve
                    .segment(cube_curve.segments.len() - 1)
                    .expect("segment exists"),
            )
        } else {
            None
        };
        let in_tangent = match in_seg {
            Some((_, CubicSegment::Curve(_, c2, p2))) => Some(p2 - c2),
            Some((p1, CubicSegment::Line(p2))) => Some(p2 - p1),
            None => None,
        }
        .map(Point2D::normalize);

        tangents.push(Tangent {
            in_: in_tangent,
            out: out_tangent,
        });
    }
    tangents
}

/// Move a point in its normal direction by a given offset. Both left and right
/// offsets should be the signed distance in the corresponding direction.
///
/// Returns `(left_point, right_point)`.
fn move_point_normal_both(
    point: Point2D,
    tangent: Point2D,
    left_offset: f64,
    right_offset: f64,
) -> (Point2D, Point2D) {
    let normal = tangent.normal_left(); // on the left side
    let left_offset_ = normal * left_offset;
    let right_offset_ = -normal * right_offset;
    ((point + left_offset_), (point + right_offset_))
}

fn make_line_join(
    cp: SpiroCp,
    in_tangent: Point2D,
    out_tangent: Point2D,
    left_offset: f64,
    right_offset: f64,
) -> (Point2D, Point2D) {
    // Move separately with in and out tangent
    let (in_left, in_right) =
        move_point_normal_both(cp.into(), in_tangent, left_offset, right_offset);
    let (out_left, _out_right) =
        move_point_normal_both(cp.into(), out_tangent, left_offset, right_offset);

    /*
    Now we're getting these 4 points:

                /    /    /
              /    /    /
    out_left *   /    /
    in_left *   *   * in_right
            | cp|  *|out_right
            |   |   |
            |   |   |

    To make the correct cap (without trying to solve the intersection of two
    bezier curves; that's for another day), we need to move the left and
    right points along their tangents to meet at a point. That is, to
    solve:

        k1 * in_tangent + in_left == -k2 * out_tangent + out_left
        (same for right side)

    Splitting the x and y components, we get the following linear equations:

        [ (A)
            in_tangent.x,  -out_tangent.x;
            in_tangent.y,  -out_tangent.y;
        ] * [k1; -k2] == [ (B)
            out_left.x - in_left.x;
            out_left.y - in_left.y;
        ]

    Since in_tangent.x != out_tangent.x and in_tangent.y != out_tangent.y, the
    matrix is invertible, and we can solve for k1 and k2. Actually, we can just
    solve for k1, since our result can only depend on one of them.

    According to Cramer's rule, for the following equation:

        [a1, b1; a2, b2] * [x; y] = [c1; c2]
    */

    let a1 = in_tangent.x;
    let b1 = -out_tangent.x;
    let a2 = in_tangent.y;
    let b2 = -out_tangent.y;
    let c1 = out_left.x - in_left.x;
    let c2 = out_left.y - in_left.y;

    // ... the solution for x is:
    let k1_left = (c1 * b2 - b1 * c2) / (a1 * b2 - b1 * a2);

    // And our join point on the left side is just:
    let join_left = in_left + k1_left * in_tangent;

    // Additionally, for the right side, k1 can be directly derived from that
    // of the left side:
    let k1_right = -(right_offset / left_offset) * k1_left;
    let join_right = in_right + k1_right * in_tangent;

    (join_left, join_right)
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
