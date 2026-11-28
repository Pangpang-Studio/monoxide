//! Fit the two edges from samples of the solved centerline. Distances across
//! a stroke are measured on the local normal, in font em units.

use kurbo::{CubicBez, ParamCurve, ParamCurveArclen, ParamCurveDeriv, PathSeg, Point, Vec2};

use super::cap::{Cap, cut_edge};
use crate::{CubicBezier, CubicSegment, cube::unit_tangent, debug::CurveDebugger};

/// Smaller than one tenth of a unit at 2048 units per em.
pub(super) const ACCURACY: f64 = 1e-5;

#[derive(Clone, Copy)]
pub(super) struct Knot {
    pub point: Point,
    pub segment: usize,
    pub width: f64,
    pub alignment: f64,
    pub heading: Option<Vec2>,
}

#[derive(Clone, Copy, PartialEq)]
struct Section {
    width: f64,
    alignment: f64,
}

impl Section {
    fn lerp(self, other: Self, t: f64) -> Self {
        Self {
            width: self.width + (other.width - self.width) * t,
            alignment: self.alignment + (other.alignment - self.alignment) * t,
        }
    }
}

impl Knot {
    fn section(self) -> Section {
        Section {
            width: self.width,
            alignment: self.alignment,
        }
    }

    /// The angle bisector gives a miter at an unmarked corner. Beyond four
    /// times the side offset each span keeps its normal, producing a bevel.
    fn cut_heading(self, tangent: Vec2, adjacent: Option<Vec2>) -> Option<Vec2> {
        self.heading.or_else(|| {
            let other = adjacent?;
            let dot = other.dot(tangent);
            (dot < 1. - 1e-8 && 1. + dot >= 2. / 4_f64.powi(2))
                .then(|| (other + tangent).normalize())
        })
    }
}

/// One side of one centerline segment, measured on its local normal.
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
            unit_tangent(seg, t).unwrap()
        } else {
            velocity / speed
        };
        let normal = tan.turn_90();
        let alignment = section.alignment - self.side;
        let offset = section.width * alignment;
        let point = seg.eval(t) + normal * offset;

        // Differentiate center + normal * offset. Matching the actual edge
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
        (
            point,
            velocity + d_tan.turn_90() * offset + normal * d_offset,
        )
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
        let first = segments[*range.start()];
        let last = segments[*range.end()];
        let incoming = unit_tangent(first, 0.).unwrap();
        let outgoing = unit_tangent(last, 1.).unwrap();
        let start_heading = start.cut_heading(
            incoming,
            prev.and_then(|(_, _, r)| unit_tangent(segments[*r.end()], 1.)),
        );
        let end_heading = end.cut_heading(
            outgoing,
            next.and_then(|(_, _, r)| unit_tangent(segments[*r.start()], 0.)),
        );
        let mut edges = [0., 1.].map(|side| CubicBezier {
            start: first.start() + incoming.turn_90() * (start.width * (start.alignment - side)),
            segments: vec![],
            closed: false,
        });
        let length: f64 = lengths[range.clone()].iter().sum();
        let mut distance = 0.;
        for j in range.clone().filter(|&j| lengths[j] > 0.) {
            let seg = segments[j];
            let from = start.section().lerp(end.section(), distance / length);
            distance += lengths[j];
            let to = start.section().lerp(end.section(), distance / length);
            for (side, path) in edges.iter_mut().enumerate() {
                let edge = Edge {
                    segment: seg,
                    length: lengths[j],
                    start: from,
                    end: to,
                    side: side as f64,
                };
                edge.fit(0., 1., 0, &mut path.segments);
            }
        }

        for (side, mut edge) in edges.into_iter().enumerate() {
            if start_heading.is_some() || end_heading.is_some() {
                cut_edge(
                    &mut edge,
                    Cap::new(start.point, incoming, start_heading),
                    Cap::new(end.point, -outgoing, end_heading),
                );
            }
            if side == 1 {
                dbg.line(first.start(), edge.start, format_args!(""));
            }
            let path = sides[side].get_or_insert_with(|| CubicBezier {
                start: edge.start,
                segments: vec![],
                closed,
            });
            let tip = path
                .segments
                .last()
                .map_or(path.start, CubicSegment::last_point);
            if tip.distance(edge.start) > ACCURACY * 0.01 {
                path.segments.push(CubicSegment::Line(edge.start));
            }
            path.segments.extend(edge.segments);
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
        if outline
            .segments
            .last()
            .is_some_and(|s| s.last_point() != outline.start)
        {
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
