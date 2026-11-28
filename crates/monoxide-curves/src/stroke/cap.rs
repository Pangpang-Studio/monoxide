//! Intersect the offset edges with terminal cut lines. Both cuts pass through
//! the original centerline endpoints.

use kurbo::{Line, ParamCurve, PathSeg, Point, Vec2};

use crate::{CubicBezier, CubicSegment, cube::unit_tangent};

#[derive(Clone, Copy)]
pub(super) struct Cap {
    center: Point,
    inward: Vec2,
    normal: Vec2,
}

impl Cap {
    pub fn new(center: Point, inward: Vec2, heading: Option<Vec2>) -> Self {
        // A cut parallel to the stroke cannot span its width. Use a butt
        // cap in that limit. Reversing an authored heading gives the same cut.
        let normal = heading
            .filter(|h| h.dot(inward).abs() > 1e-8)
            .map_or(inward, |h| h * h.dot(inward).signum());
        Self {
            center,
            inward,
            normal,
        }
    }

    fn distance(&self, p: Point) -> f64 {
        (p - self.center).dot(self.normal)
    }

    // Construct the point in the cut's local frame so axis-aligned caps
    // share the authored coordinate exactly when glyph parts meet.
    fn project(&self, p: Point) -> Point {
        let axis = self.normal.turn_90();
        self.center + axis * ((p - self.center).dot(axis) / axis.hypot2())
    }

    fn extension(&self, seg: PathSeg) -> Option<PathSeg> {
        let p = seg.start();
        let distance = self.distance(p);
        if distance <= super::offset::ACCURACY * 0.01 {
            return None;
        }
        // Continue the edge tangent, including any taper.
        let tangent = unit_tangent(seg, 0.)
            .filter(|v| v.dot(self.normal) > 1e-8)
            .unwrap_or(self.inward);
        let tip = p - tangent * (distance / tangent.dot(self.normal));
        Some(PathSeg::Line(Line::new(tip, p)))
    }

    /// Search monotone intervals of signed distance in traversal order.
    /// Bisection also handles a quadratic taper encoded as a cubic, where
    /// roundoff in the leading power coefficient can spoil a cubic root.
    fn crossing(&self, segments: impl Iterator<Item = PathSeg>) -> Option<(usize, f64)> {
        for (i, seg) in segments.enumerate() {
            if self.distance(seg.start()) >= -super::offset::ACCURACY * 0.01 {
                return Some((i, 0.));
            }
            let mut extrema = if let PathSeg::Cubic(c) = seg {
                let d0 = (c.p1 - c.p0).dot(self.normal);
                let d1 = (c.p2 - c.p1).dot(self.normal);
                let d2 = (c.p3 - c.p2).dot(self.normal);
                kurbo::common::solve_quadratic(d0, 2. * (d1 - d0), d0 - 2. * d1 + d2)
            } else {
                Default::default()
            };
            let mut lo = 0.0_f64;
            extrema.sort_by(f64::total_cmp);
            for mut hi in extrema
                .into_iter()
                .filter(|&t| t > 0. && t < 1.)
                .chain([1.])
            {
                let distance = self.distance(seg.eval(hi));
                if distance.abs() <= 1e-13 {
                    return Some((i, hi));
                }
                if distance > 0. {
                    for _ in 0..48 {
                        let mid = lo.midpoint(hi);
                        if self.distance(seg.eval(mid)) < 0. {
                            lo = mid;
                        } else {
                            hi = mid;
                        }
                    }
                    return Some((i, lo.midpoint(hi)));
                }
                lo = hi;
            }
        }
        None
    }

    fn line(&self) -> Line {
        Line::new(self.center, self.center + self.normal.turn_90())
    }
}

pub(super) fn cut_edge(edge: &mut CubicBezier, start: Cap, end: Cap) {
    let mut segments: Vec<_> = edge.segment_iter().map(PathSeg::from).collect();
    // Extend both ends before trimming. On a short stroke one cap can meet
    // the tangent extension from the other end.
    if let Some(extension) = end.extension(segments.last().unwrap().reverse()) {
        segments.push(extension.reverse());
    }
    if let Some(extension) = start.extension(segments[0]) {
        segments.insert(0, extension);
    }

    let first = start.crossing(segments.iter().copied());
    let last = end
        .crossing(segments.iter().rev().map(PathSeg::reverse))
        .map(|(i, t)| (segments.len() - 1 - i, 1. - t));
    edge.segments.clear();
    if let Some((first, last)) = first.zip(last).filter(|(a, b)| a <= b) {
        edge.start = start.project(segments[first.0].eval(first.1));
        for (i, seg) in segments.iter().enumerate().take(last.0 + 1).skip(first.0) {
            let t0 = if i == first.0 { first.1 } else { 0. };
            let t1 = if i == last.0 { last.1 } else { 1. };
            if t0 == t1 {
                continue;
            }
            let segment = if (t0, t1) == (0., 1.) {
                *seg
            } else {
                seg.subsegment(t0..t1)
            };
            edge.segments.push(match segment {
                PathSeg::Line(l) => CubicSegment::Line(l.p1),
                PathSeg::Cubic(c) => CubicSegment::Curve(c.p1, c.p2, c.p3),
                PathSeg::Quad(_) => unreachable!("stroke edges contain only cubics and lines"),
            });
        }
        if let Some(CubicSegment::Line(p) | CubicSegment::Curve(_, _, p)) = edge.segments.last_mut()
        {
            *p = end.project(*p);
        }
    } else {
        // The two cuts meet inside the ribbon and consume this entire side.
        edge.start = start
            .line()
            .crossing_point(end.line())
            .unwrap_or(edge.start);
    }
}
