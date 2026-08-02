//! Convert a cubic bezier curve into a quadratic bezier curve.
//!
//! This is a thin wrapper around [`kurbo::CubicBez::to_quads`].

use kurbo::PathSeg;

use crate::{CubicBezier, CubicSegment, QuadBezier};

/// Convert a cubic bezier curve into a quadratic bezier curve, with `prec` as
/// the maximum allowed distance between the original curve and its
/// approximation.
pub fn cube_to_quad(cube: CubicBezier, prec: f64) -> QuadBezier {
    let mut quad = QuadBezier::builder(cube.start);

    for seg in cube.segment_iter() {
        match seg.rest {
            CubicSegment::Line(end) => {
                quad.line_to(end);
            }
            CubicSegment::Curve(..) => {
                let PathSeg::Cubic(cubic) = PathSeg::from(seg) else {
                    unreachable!("CubicSegment::Curve always converts to PathSeg::Cubic");
                };
                for (_, _, q) in cubic.to_quads(prec) {
                    quad.quad_to(q.p1, q.p2);
                }
            }
        }
    }
    if cube.closed {
        quad.close();
    }

    quad.build()
}
