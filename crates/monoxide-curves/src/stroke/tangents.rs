use monoxide_spiro::SpiroCp;

use crate::{
    CubicBezier,
    point::Point2D,
    stroke::{Tangent, approx_eq},
};

/// Calculate the tangent at each spiro control point. Returns a vector of
/// normalized tangent vectors, one for each control point.
pub fn calc_tangents(
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

    use crate::cube::CubicSegment::*;
    use crate::cube::CubicSegmentFull as CSF;

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
            Some(CSF {
                start: p1,
                rest: Curve(c1, _, _),
            }) => Some(c1 - p1),
            Some(CSF {
                start: p1,
                rest: Line(p2),
            }) => Some(p2 - p1),
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
            Some(CSF {
                rest: Curve(_, c2, p2),
                ..
            }) => Some(p2 - c2),
            Some(CSF {
                start: p1,
                rest: Line(p2),
            }) => Some(p2 - p1),
            None => None,
        }
        .map(Point2D::normalize);

        let tan = match (in_tangent, out_tangent) {
            (Some(in_), Some(out)) if approx_eq(in_, out) => Tangent::Continuous(in_),
            (Some(in_), Some(out)) => Tangent::Corner { in_, out },
            (Some(in_), None) => Tangent::Continuous(in_),
            (None, Some(out_)) => Tangent::Continuous(out_),
            (None, None) => {
                panic!("Both tangents are None for index {index}, this should not happen")
            }
        };
        tangents.push(tan);
    }
    tangents
}

/// Move a point in its normal direction by a given offset. Both left and right
/// offsets should be the signed distance in the corresponding direction.
///
/// Returns `(left_point, right_point)`.
pub fn move_point_normal_both(
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

pub fn make_line_join(
    cp: SpiroCp,
    in_tangent: Point2D,
    out_tangent: Point2D,
    left_offset: f64,
    right_offset: f64,
) -> (Point2D, Point2D) {
    // Move separately with in and out tangent
    let (in_left, in_right) =
        move_point_normal_both(cp.into(), in_tangent, left_offset, right_offset);
    let (out_left, out_right) =
        move_point_normal_both(cp.into(), out_tangent, left_offset, right_offset);

    // {left,right}_offset might be close to zero due to stroke alignments,
    // making the thing numerically unstable/might get NaN/Infinity. Choose a
    // better side to calculate from.
    //
    // We should not rely on the absolute values of the offsets (stroke width).
    // Calculate a ratio instead:
    let lr_ratio = left_offset / right_offset;
    // - Left close to zero: |lr_ratio| < 1e-9 -> use right side
    // - Right close to zero: |lr_ratio| > 1e9 -> use left side
    // - Regular case -> use left side
    // - Both close to zero: dude why?!
    if lr_ratio < 1e-9 {
        let (k1_right, join_right) = calc_join(in_tangent, out_tangent, in_right, out_right);

        let k1_left = -(left_offset / right_offset) * k1_right;
        let join_left = in_left + k1_left * in_tangent;
        (join_left, join_right)
    } else {
        let (k1_left, join_left) = calc_join(in_tangent, out_tangent, in_left, out_left);

        let k1_right = -(right_offset / left_offset) * k1_left;
        let join_right = in_right + k1_right * in_tangent;
        (join_left, join_right)
    }
}

fn calc_join(
    in_tangent: Point2D,
    out_tangent: Point2D,
    in_tip: Point2D,
    out_tip: Point2D,
) -> (f64, Point2D) {
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
    let c1 = out_tip.x - in_tip.x;
    let c2 = out_tip.y - in_tip.y;

    // ... the solution for x is:
    let k1 = (c1 * b2 - b1 * c2) / (a1 * b2 - b1 * a2);

    // And our join point on the left side is just:
    let res = in_tip + k1 * in_tangent;
    (k1, res)
}
