//! Adapt an arc-length-dependent offset to kurbo's Bézier fitter.

use std::ops::Range;

use kurbo::{
    BezPath, CubicBez, CurveFitSample, ParamCurve, ParamCurveArclen, ParamCurveDeriv,
    ParamCurveFit, PathSeg, Point, Vec2, fit_to_bezpath, offset::offset_cubic,
};

use super::TOLERANCE;
use crate::error::{Error, Result};

/// Offsets are expressed in the moving tangent/left-normal frame. A straight
/// centerline with a linear taper therefore still produces an exact line.
#[derive(Clone, Copy)]
pub(super) struct OffsetSpan {
    pub start: Vec2,
    pub end: Vec2,
    pub length: f64,
}

impl OffsetSpan {
    pub fn append(self, out: &mut BezPath, seg: PathSeg, s0: f64, s1: f64) -> Result<()> {
        let from = self.start;
        let to = self.end;
        if !from.is_finite() || !to.is_finite() {
            return Err(Error::InvalidStroke);
        }
        let mut path = BezPath::new();
        match seg {
            PathSeg::Line(line) => {
                let t = (line.p1 - line.p0).normalize();
                let offset = |v: Vec2| t * v.x + t.turn_90() * v.y;
                path.move_to(line.p0 + offset(from.lerp(to, s0 / self.length)));
                path.line_to(line.p1 + offset(from.lerp(to, s1 / self.length)));
            }
            _ if from == Vec2::ZERO && to == Vec2::ZERO => {
                path.move_to(seg.start());
                path.push(seg.as_path_el());
            }
            _ if from == to && from.x == 0.0 => {
                offset_cubic(seg.to_cubic(), from.y, TOLERANCE, &mut path);
            }
            _ => {
                let cubic = seg.to_cubic();
                let edge = ProfileEdge {
                    cubic,
                    span: self,
                    s0,
                };
                path = fit_to_bezpath(&edge, TOLERANCE);
            }
        }
        let start = path.elements()[0].end_point().unwrap();
        match out.current_position() {
            None => out.move_to(start),
            Some(prev) if prev.distance(start) > TOLERANCE => out.line_to(start),
            _ => {}
        }
        out.extend(path.into_iter().skip(1));
        Ok(())
    }
}

struct ProfileEdge {
    cubic: CubicBez,
    span: OffsetSpan,
    s0: f64,
}

impl ParamCurveFit for ProfileEdge {
    fn sample_pt_deriv(&self, t: f64) -> (Point, Vec2) {
        let derivative = self.cubic.deriv();
        let velocity = derivative.eval(t).to_vec2();
        let speed = velocity.hypot();
        let tangent = velocity / speed;
        let normal = tangent.turn_90();
        let s = self.s0 + self.cubic.subsegment(0.0..t).arclen(TOLERANCE * 0.1);
        let u = (s / self.span.length).clamp(0., 1.);
        // Ease the offset into each knot so an alignment change on a smooth
        // centerline doesn't introduce a corner in its boundary.
        let offset = self.span.start.lerp(self.span.end, u * u * (3. - 2. * u));
        let slope = (self.span.end - self.span.start) * (6. * u * (1. - u) / self.span.length);
        // Derivatives of the moving frame: T' = angle' N and N' = -angle' T.
        let turn = normal.dot(derivative.deriv().eval(t).to_vec2()) / speed;
        let p = self.cubic.eval(t) + tangent * offset.x + normal * offset.y;
        let d = tangent * (speed * (1.0 + slope.x) - turn * offset.y)
            + normal * (speed * slope.y + turn * offset.x);
        (p, d)
    }

    fn sample_pt_tangent(&self, t: f64, _: f64) -> CurveFitSample {
        let (p, tangent) = self.sample_pt_deriv(t);
        CurveFitSample { p, tangent }
    }

    fn break_cusp(&self, _: Range<f64>) -> Option<f64> {
        None
    }
}
