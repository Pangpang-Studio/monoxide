use num_traits::real::Real;

use crate::{CubicBezier, CubicSegment, Point, QuadBezier, RealPoint, quad::QuadBezierBuilder};

/*
Conventions/terminology

    Cubic defined by: P1/2 - anchor points, C1/C2 control points
    |x| is the euclidean norm of x
    mid-point approx of cubic: a quad that shares the same anchors with the cubic and has the control point at C = (3·C2 - P2 + 3·C1 - P1)/4

Algorithm

    pick an absolute precision (prec)
    Compute the Tdiv as the root of (cubic) equation sqrt(3)/18 · |P2 - 3·C2 + 3·C1 - P1|/2 · Tdiv ^ 3 = prec
    if Tdiv < 0.5 divide the cubic at Tdiv. First segment [0..Tdiv] can be approximated with by a quadratic, with a defect less than prec, by the mid-point approximation. Repeat from step 2 with the second resulted segment (corresponding to 1-Tdiv)
    0.5<=Tdiv<1 - simply divide the cubic in two. The two halves can be approximated by the mid-point approximation
    Tdiv>=1 - the entire cubic can be approximated by the mid-point approximation

*/

/// Convert a cubic bezier curve into a quadratic bezier curve
pub fn cube_to_quad<P, S>(cube: CubicBezier<P>, prec: S) -> QuadBezier<P>
where
    P: RealPoint<Scalar = S> + Copy,
    S: Real + Copy,
{
    let mut quad = QuadBezier::builder(cube.start);

    for seg in cube.segment_iter() {
        match seg.rest {
            CubicSegment::Line(end) => {
                quad.line_to(end);
            }
            CubicSegment::Curve(c1, c2, p2) => {
                cube_to_quad_segment(&mut quad, seg.start, c1, c2, p2, prec)
            }
        }
    }
    if cube.closed {
        quad.close();
    }

    quad.build()
}

fn cube_to_quad_segment<P, S>(quad: &mut QuadBezierBuilder<P>, p1: P, c1: P, c2: P, p2: P, prec: S)
where
    P: RealPoint<Scalar = S> + Copy,
    S: Real + Copy,
{
    if !cube_to_quad_segment_ff(quad, p1, c1, c2, p2, prec) {
        cube_to_quad_segment_naive(quad, p1, c1, c2, p2, prec);
    }
}

/// The maximum number of segments that can be used to approximate one cubic
/// bezier segment in the FontForge algorithm.
const MAX_SEGMENTS_ALLOWED: usize = 4;

/// Convert a cubic segment into quadratic segment using the smarter approach
/// as described in FontForge documentation. Returns `false` if this algorithm
/// cannot find a good enough approximation.
///
/// The quadratic bezier will only be modified if this algorithm can find a
/// good enough approximation, otherwise it is untouched.
///
/// <https://fontforge.org/docs/techref/bezier.html#converting-postscript-to-truetype>
fn cube_to_quad_segment_ff<P, S>(
    quad: &mut QuadBezierBuilder<P>,
    p1: P,
    c1: P,
    c2: P,
    p2: P,
    prec: S,
) -> bool
where
    P: RealPoint<Scalar = S> + Copy,
    S: Real + Copy,
{
    // FontForge's algorithm tries to add evenly-spaced points along the cubic
    // to approximate it with a quadratic.
    //
    // Their documentation didn't specify which metric to use when calculating
    // the "goodness" of the approximation. We will use the `tdiv` value (which
    // is also used in the naive algorithm) as the metric.
    //
    // TODO: we're calculating the `tdiv` of the original curve twice.
    // Probably can be optimized in the future.

    // Cache the parameters for each segment so we don't have to recalculate
    // them again. Filled with dummy values for now.
    let mut params = [(c1, c2, p2); MAX_SEGMENTS_ALLOWED];

    for n_segments in 1..=MAX_SEGMENTS_ALLOWED {
        let mut max_tdiv = S::zero();
        // Parameters for the remaining curve segment.
        let (mut p1, mut c1, mut c2) = (p1, c1, c2);
        #[allow(clippy::needless_range_loop)] // False positive
        for i in 0..n_segments {
            // The segment we're working with
            let (dp1, dc1, dc2, dp2) = if i == n_segments - 1 {
                // Last segment, save one calculation
                (p1, c1, c2, p2)
            } else {
                // 1 / remaining segments. i.e. (1/n) / ((n - i) / n)
                let t_in_remain = S::one() / S::from(n_segments - i).unwrap();
                let (c11, c12, p12, c21, c22) = divide_cube(p1, c1, c2, p2, t_in_remain);
                (p1, c1, c2) = (p12, c21, c22);
                (p1, c11, c12, p12)
            };

            let tdiv = tdiv(dp1, dc1, dc2, dp2, prec);
            max_tdiv = max_tdiv.max(tdiv);

            // Cache params so we don't have to recalculate them again
            params[i] = (dc1, dc2, dp2);
        }

        if max_tdiv >= S::one() {
            // The approximation is good enough. Revive the parameters we cached
            // and convert them to quadratic segments.
            for i in 0..n_segments {
                let (dc1, dc2, dp2) = params[i];
                let dp1 = if i == 0 { p1 } else { params[i - 1].2 };
                let c = midpoint_approx(dp1, dc1, dc2, dp2);
                quad.quad_to(c, dp2);
            }
            return true;
        } else {
            // The approximation is not good enough. Try with more segments.
            continue;
        }
    }

    false
}

/// Convert a cubic segment into quadratic segment using the more naive approach
/// described in [this algorithm](https://stackoverflow.com/a/3334952).
fn cube_to_quad_segment_naive<P, S>(
    quad: &mut QuadBezierBuilder<P>,
    mut p1: P,
    mut c1: P,
    mut c2: P,
    p2: P,
    prec: S,
) where
    P: RealPoint<Scalar = S> + Copy,
    S: Real + Copy,
{
    let half = S::from(0.5).unwrap();
    loop {
        let tdiv = tdiv(p1, c1, c2, p2, prec);
        if tdiv < half {
            // if Tdiv < 0.5 divide the cubic at Tdiv. First segment [0..Tdiv] can
            // be approximated with by a quadratic, with a defect less than prec, by
            // the mid-point approximation. Repeat from step 2 with the second
            // resulted segment (corresponding to 1-Tdiv)
            let (c11, c12, p12, c21, c22) = divide_cube(p1, c1, c2, p2, tdiv);
            let c01 = midpoint_approx(p1, c11, c12, p12);
            quad.quad_to(c01, p12);

            // Ah shit, here we go again
            (p1, c1, c2) = (p12, c21, c22);
        } else if tdiv < S::one() {
            // 0.5 <= Tdiv < 1 - simply divide the cubic in two. The two halves can
            // be approximated by the mid-point approximation
            let (c11, c12, p12, c21, c22) = divide_cube(p1, c1, c2, p2, half);
            let c01 = midpoint_approx(p1, c11, c12, p12);
            let c02 = midpoint_approx(p12, c21, c22, p2);
            quad.quad_to(c01, p12);
            quad.quad_to(c02, p2);
            break;
        } else {
            // Tdiv >= 1 - the entire cubic can be approximated by the mid-point
            // approximation
            let c = midpoint_approx(p1, c1, c2, p2);
            quad.quad_to(c, p2);
            break;
        }
    }
}

/// Approximate a cubic bezier curve with a quadratic bezier curve.
fn midpoint_approx<P, S>(p1: P, c1: P, c2: P, p2: P) -> P
where
    P: Point<Scalar = S> + Copy,
    S: Real + Copy,
{
    let three = S::from(3).unwrap();
    let one_over_four = S::from(4).unwrap().recip();
    let c = (c2.mul_scalar(three))
        .point_sub(&p2)
        .point_add(&c1.mul_scalar(three))
        .point_sub(&p1);
    c.mul_scalar(one_over_four)
}

/// Calculate the `tdiv` value for the cubic bezier curve.
fn tdiv<P, S>(p1: P, c1: P, c2: P, p2: P, prec: S) -> S
where
    P: RealPoint<Scalar = S> + Copy,
    S: Real + Copy,
{
    // Solve: sqrt(3)/18 · |P2 - 3·C2 + 3·C1 - P1|/2 · Tdiv ^ 3 = prec
    let two = S::from(2).unwrap();
    let three = S::from(3).unwrap();
    let sqrt3 = S::from(3).unwrap().sqrt();
    let eighteen = S::from(18).unwrap();

    // c = P2 - 3·C2 + 3·C1 - P1
    let c = (p2)
        .point_sub(&c2.mul_scalar(three))
        .point_add(&c1.mul_scalar(three))
        .point_sub(&p1);
    let c_norm = c.norm();
    // lhs = sqrt(3)/18 * (|c|/2)
    let lhs = sqrt3 / eighteen * c_norm / two;
    let tdiv_cubed = prec / lhs;
    tdiv_cubed.cbrt()
}

/// Performs a de Casteljau division of a cubic bezier segment at the given `t`
/// value. `t` must be in the range `[0, 1]`.
///
/// Returns five points `(c11, c12, p12, c21, c22)` that, combined with the
/// existing parameters `p1` and `p2`, define the two cubic bezier segments
/// `(p1, c11, c12, p12)` and `(p12, c21, c22, p2)`.
///
/// Also see: https://math.stackexchange.com/questions/3092244/
fn divide_cube<P, S>(p1: P, c1: P, c2: P, p2: P, t: S) -> (P, P, P, P, P)
where
    P: Point<Scalar = S> + Copy,
    S: Real + Copy,
{
    let one_minus_t = S::one() - t;
    // Calculate the points involved in de Casteljau's algorithm
    let m10 = p1.mul_scalar(one_minus_t).point_add(&c1.mul_scalar(t));
    let m11 = c1.mul_scalar(one_minus_t).point_add(&c2.mul_scalar(t));
    let m12 = c2.mul_scalar(one_minus_t).point_add(&p2.mul_scalar(t));
    let m20 = m10.mul_scalar(one_minus_t).point_add(&m11.mul_scalar(t));
    let m21 = m11.mul_scalar(one_minus_t).point_add(&m12.mul_scalar(t));
    let m30 = m20.mul_scalar(one_minus_t).point_add(&m21.mul_scalar(t));

    // According to de Casteljau division, the control points of the new
    // segments is simply: (m10, m20, m30) and (m30, m21, m12)
    (m10, m20, m30, m21, m12)
}
