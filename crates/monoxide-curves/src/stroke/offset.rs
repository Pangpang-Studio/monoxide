//! Fit the two edges from samples of the solved centerline. Distances across
//! a stroke are measured on the local normal, in font em units.

use kurbo::{CubicBez, ParamCurve, ParamCurveArclen, ParamCurveDeriv, PathSeg, Point, Vec2};

use crate::{CubicBezier, CubicSegment, debug::CurveDebugger};

/// Smaller than one tenth of a unit at 2048 units per em.
pub(super) const ACCURACY: f64 = 1e-5;

#[derive(Clone, Copy)]
pub(super) struct Knot {
    pub segment: usize,
    pub width: f64,
    pub alignment: f64,
    pub heading: Option<Vec2>,
}

#[derive(Clone, Copy, PartialEq)]
struct Section {
    width: f64,
    alignment: f64,
    // Tangential travel per unit of normal displacement, for an oblique cut.
    skew: f64,
}

impl Section {
    fn lerp(self, other: Self, t: f64) -> Self {
        Self {
            width: self.width + (other.width - self.width) * t,
            alignment: self.alignment + (other.alignment - self.alignment) * t,
            skew: self.skew + (other.skew - self.skew) * t,
        }
    }
}

fn tangent(seg: PathSeg, t: f64) -> Vec2 {
    if let PathSeg::Line(line) = seg {
        return (line.p1 - line.p0).normalize();
    }
    let cubic = seg.to_cubic();
    let velocity = cubic.deriv();
    let derivative = velocity.eval(t).to_vec2();
    if derivative != Vec2::ZERO {
        return derivative.normalize();
    }
    // A repeated control handle has zero velocity. Use the first nonzero
    // derivative, taking the incoming limit at the end of the segment.
    let acceleration = velocity.deriv().eval(t).to_vec2();
    if acceleration != Vec2::ZERO {
        return if t == 1. { -acceleration } else { acceleration }.normalize();
    }
    (cubic.p3 - cubic.p0).normalize()
}

/// The same cut must meet both incident edges at the knot. A miter uses the
/// angle bisector. At near-reversals it would be unbounded, so each side
/// keeps its own normal and the two endpoints form a bevel.
fn section(knot: Knot, tangent: Vec2, adjacent: Option<Vec2>) -> Section {
    let skew = match adjacent {
        Some(other) if other.dot(tangent) < 1. - 1e-8 => {
            let denominator = 1. + other.dot(tangent);
            // Limit the miter to four times the side's offset.
            if denominator >= 2. / 4_f64.powi(2) {
                other.turn_90().dot(tangent) / denominator
            } else {
                0.
            }
        }
        _ => knot
            .heading
            .filter(|h| h.dot(tangent).abs() > 1e-8)
            .map_or(0., |h| h.turn_90().dot(tangent) / h.dot(tangent)),
    };
    Section {
        width: knot.width,
        alignment: knot.alignment,
        skew,
    }
}

/// One side of one centerline segment. The cut vector always has unit
/// projection on the normal, including when a heading tilts the cut.
struct Edge {
    segment: PathSeg,
    length: f64,
    start: Section,
    end: Section,
    side: f64,
}

impl Edge {
    fn sample(&self, t: f64) -> (Point, Vec2) {
        let seg = self.segment;
        let section = if self.start == self.end || t == 0. {
            self.start
        } else if t == 1. {
            self.end
        } else {
            let distance = seg.subsegment(0.0..t).arclen(ACCURACY * 0.1);
            self.start.lerp(self.end, distance / self.length)
        };
        let (velocity, acceleration) = match seg {
            PathSeg::Line(line) => (line.p1 - line.p0, Vec2::ZERO),
            _ => {
                let derivative = seg.to_cubic().deriv();
                (
                    derivative.eval(t).to_vec2(),
                    derivative.deriv().eval(t).to_vec2(),
                )
            }
        };
        let speed = velocity.hypot();
        let tan = if speed == 0. {
            tangent(seg, t)
        } else {
            velocity / speed
        };
        let normal = tan.turn_90();
        let cut = normal + tan * section.skew;
        let alignment = section.alignment - self.side;
        let offset = section.width * alignment;
        let point = seg.eval(t) + cut * offset;

        // Differentiate center + cut * offset. Matching the actual edge
        // tangents avoids small loops at horizontal and vertical extrema.
        let du = speed / self.length;
        let d_offset = du
            * ((self.end.width - self.start.width) * alignment
                + section.width * (self.end.alignment - self.start.alignment));
        let d_tan = if speed > 0. {
            normal * (tan.cross(acceleration) / speed)
        } else {
            Vec2::ZERO
        };
        let d_cut =
            d_tan.turn_90() + d_tan * section.skew + tan * (du * (self.end.skew - self.start.skew));
        (point, velocity + d_cut * offset + cut * d_offset)
    }

    /// Fit a cubic Hermite segment and split wherever the sampled edge
    /// deviates from it. Lines and polynomial tapers need only one segment.
    fn fit(&self, t0: f64, t1: f64, depth: u8, path: &mut Vec<CubicSegment>) {
        let (p0, v0) = self.sample(t0);
        let (p1, v1) = self.sample(t1);
        let dt = t1 - t0;
        let cubic = CubicBez::new(p0, p0 + v0 * (dt / 3.), p1 - v1 * (dt / 3.), p1);
        if depth < 16
            && [0.25, 0.5, 0.75]
                .into_iter()
                .any(|u| cubic.eval(u).distance(self.sample(t0 + dt * u).0) > ACCURACY)
        {
            let mid = t0.midpoint(t1);
            self.fit(t0, mid, depth + 1, path);
            self.fit(mid, t1, depth + 1, path);
        } else if cubic
            .p1
            .distance(p0.lerp(p1, 1. / 3.))
            .max(cubic.p2.distance(p0.lerp(p1, 2. / 3.)))
            < ACCURACY * 0.1
        {
            path.push(CubicSegment::Line(p1));
        } else {
            path.push(CubicSegment::Curve(cubic.p1, cubic.p2, p1));
        }
    }
}

pub(super) fn outline(
    segments: &[PathSeg],
    mut knots: Vec<Knot>,
    closed: bool,
    dbg: &mut impl CurveDebugger,
) -> Vec<CubicBezier> {
    let lengths: Vec<_> = segments.iter().map(|s| s.arclen(ACCURACY * 0.1)).collect();
    // Skip empty spans before finding adjacent tangents, including at the seam.
    if closed && let Some(&first) = knots.first() {
        knots.push(Knot {
            segment: segments.len(),
            ..first
        });
    }
    let spans: Vec<_> = knots
        .array_windows()
        .filter_map(|&[start, end]| {
            let range = start.segment..end.segment;

            let mut nonempty = range.filter(|&j| lengths[j] > 0.);
            let first = nonempty.next()?;
            let last = nonempty.next_back().unwrap_or(first);
            Some((start, end, first..=last))
        })
        .collect();
    let mut sides: [Option<CubicBezier>; 2] = [None, None];
    for (i, (start, end, range)) in spans.iter().enumerate() {
        let prev = if i == 0 && closed {
            spans.last()
        } else {
            i.checked_sub(1).and_then(|j| spans.get(j))
        };
        let next = spans
            .get(i + 1)
            .or_else(|| if closed { spans.first() } else { None });
        let start_section = section(
            *start,
            tangent(segments[*range.start()], 0.),
            prev.map(|(_, _, r)| tangent(segments[*r.end()], 1.)),
        );
        let end_section = section(
            *end,
            tangent(segments[*range.end()], 1.),
            next.map(|(_, _, r)| tangent(segments[*r.start()], 0.)),
        );
        let length: f64 = lengths[range.clone()].iter().sum();
        let mut distance = 0.;
        for j in range.clone().filter(|&j| lengths[j] > 0.) {
            let seg = segments[j];
            let start = start_section.lerp(end_section, distance / length);
            distance += lengths[j];
            let end = start_section.lerp(end_section, distance / length);
            for (side, path) in sides.iter_mut().enumerate() {
                let edge = Edge {
                    segment: seg,
                    length: lengths[j],
                    start,
                    end,
                    side: side as f64,
                };
                let from = edge.sample(0.).0;
                let path = path.get_or_insert_with(|| CubicBezier {
                    start: from,
                    segments: vec![],
                    closed,
                });
                if let Some(last) = path.segments.last()
                    && last.last_point().distance(from) > ACCURACY * 0.01
                {
                    path.segments.push(CubicSegment::Line(from));
                }
                edge.fit(0., 1., 0, &mut path.segments);
                if side == 1 && j == *range.start() {
                    dbg.line(seg.start(), from, format_args!(""));
                }
            }
        }
    }
    let [Some(mut left), Some(right)] = sides else {
        return vec![];
    };
    let right = right.reversed();
    let mut outlines = if closed {
        vec![left, right]
    } else {
        left.segments.push(CubicSegment::Line(right.start));
        left.segments.extend(right.segments);
        left.closed = true;
        vec![left]
    };
    for outline in &mut outlines {
        // TrueType omits the final on-curve point of a closed contour.
        if outline.segments.last().unwrap().last_point() != outline.start {
            outline.segments.push(CubicSegment::Line(outline.start));
        }
    }

    // Remove self-crossings in tight turns and figure eights. Linesweeper
    // uses the opposite winding convention to our font contours.
    let path = outlines
        .iter()
        .flat_map(|c| c.to_kurbo().reverse_subpaths())
        .collect();
    let topology = linesweeper::topology::Topology::from_path(&path, ACCURACY * 0.1)
        .expect("stroke boundaries are closed");
    if topology.has_normal_contours(()) {
        outlines
    } else {
        topology
            .contours(|w| *w > 0)
            .contours()
            .map(|c| CubicBezier::from_kurbo(&c.path.reverse_subpaths()))
            .collect()
    }
}
