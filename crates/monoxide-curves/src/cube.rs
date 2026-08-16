//! A cubic bezier path, implemented as a thin wrapper over `kurbo`'s curve
//! types.

mod exchange;

use kurbo::{Affine, CubicBez, Line, ParamCurve, PathEl, PathSeg};
use serde::{Deserialize, Serialize};

use crate::point::Point2D;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(
    from = "exchange::SerdeForCubicSegment<P>",
    into = "exchange::SerdeForCubicSegment<P>",
    bound(
        serialize = "P: Clone + Serialize",
        deserialize = "P: Clone + Deserialize<'de>"
    )
)]
pub enum CubicSegment<P = Point2D> {
    Line(P),
    Curve(P, P, P),
}

impl<P: Copy> CubicSegment<P> {
    pub fn is_line(&self) -> bool {
        matches!(self, CubicSegment::Line(_))
    }

    pub fn points_count(&self) -> usize {
        match self {
            CubicSegment::Line(_) => 1,
            CubicSegment::Curve(_, _, _) => 3,
        }
    }

    pub fn last_point(&self) -> P {
        match self {
            CubicSegment::Line(p) => *p,
            CubicSegment::Curve(_, _, p) => *p,
        }
    }
}

impl CubicSegment {
    /// Applies the given affine transformation to every point in the
    /// segment.
    pub fn xform(&self, xform: Affine) -> Self {
        match self {
            CubicSegment::Line(p) => CubicSegment::Line(xform * *p),
            CubicSegment::Curve(p1, p2, p3) => {
                CubicSegment::Curve(xform * *p1, xform * *p2, xform * *p3)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CubicSegmentFull<P = Point2D> {
    pub start: P,
    pub rest: CubicSegment<P>,
}

impl From<CubicSegmentFull> for PathSeg {
    fn from(seg: CubicSegmentFull) -> Self {
        match seg.rest {
            CubicSegment::Line(end) => PathSeg::Line(Line::new(seg.start, end)),
            CubicSegment::Curve(c1, c2, end) => {
                PathSeg::Cubic(CubicBez::new(seg.start, c1, c2, end))
            }
        }
    }
}

/// Represents a cubic bezier path: a single contour made of line and cubic
/// curve segments.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(
    serialize = "P: Clone + Serialize",
    deserialize = "P: Clone + Deserialize<'de>"
))]
pub struct CubicBezier<P = Point2D> {
    pub start: P,
    pub segments: Vec<CubicSegment<P>>,
    pub closed: bool,
}

impl<P: Copy> CubicBezier<P> {
    pub fn iter(&self) -> CubicBezierPointIter<'_, P> {
        CubicBezierPointIter {
            curve: self,
            current_segment: 0,
            current_in_segment_idx: 0,
        }
    }

    pub fn builder(start: P) -> CubicBezierBuilder<P> {
        CubicBezierBuilder::new(start)
    }

    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }

    pub fn segment(&self, idx: usize) -> Option<CubicSegmentFull<P>> {
        if idx >= self.segments.len() {
            return None;
        }
        Some(CubicSegmentFull {
            start: if idx == 0 {
                self.start
            } else {
                self.segments[idx - 1].last_point()
            },
            rest: self.segments[idx].clone(),
        })
    }

    pub fn segment_iter(&self) -> impl DoubleEndedIterator<Item = CubicSegmentFull<P>> + '_ {
        (0..self.segments.len()).filter_map(move |i| self.segment(i))
    }

    pub fn reversed(&self) -> Self {
        let end_point = if let Some(last_seg) = self.segments.last() {
            last_seg.last_point()
        } else {
            self.start
        };
        let reversed_segments = self
            .segment_iter()
            .rev()
            .map(|x| match x.rest {
                CubicSegment::Line(_) => CubicSegment::Line(x.start),
                CubicSegment::Curve(c1, c2, _) => CubicSegment::Curve(c2, c1, x.start),
            })
            .collect();
        CubicBezier {
            start: end_point,
            segments: reversed_segments,
            closed: self.closed,
        }
    }
}

impl CubicBezier {
    /// Evaluates the point at parameter `t` (in `[0, 1]`) on the given
    /// segment.
    pub fn point_at(&self, segment: usize, t: f64) -> Point2D {
        PathSeg::from(self.segment(segment).expect("segment index out of bounds")).eval(t)
    }

    /// Applies the given affine transformation to the cubic bezier curve.
    ///
    /// Applying an affine transformation to a cubic bezier curve is equivalent
    /// to applying the same transformation to each point in the curve.
    pub fn xform(&self, xform: Affine) -> Self {
        CubicBezier {
            start: xform * self.start,
            segments: self.segments.iter().map(|seg| seg.xform(xform)).collect(),
            closed: self.closed,
        }
    }

    pub fn from_kurbo(path: &kurbo::BezPath) -> Self {
        let elems = path.elements();
        let mut res = Self {
            segments: Vec::with_capacity(elems.len() - 1),
            start: Point2D::default(),
            closed: false,
        };

        for elem in elems {
            match elem {
                PathEl::MoveTo(p) => res.start = *p,
                PathEl::LineTo(p) => res.segments.push(CubicSegment::Line(*p)),
                PathEl::QuadTo(_, _) => {
                    unimplemented!("quadratic segments are not supported in CubicBezier")
                }
                PathEl::CurveTo(c1, c2, p) => res.segments.push(CubicSegment::Curve(*c1, *c2, *p)),
                PathEl::ClosePath => res.closed = true,
            }
        }

        res
    }

    pub fn to_kurbo(&self) -> kurbo::BezPath {
        let mut path = kurbo::BezPath::new();

        path.move_to(self.start);
        for seg in &self.segments {
            match seg {
                CubicSegment::Line(end) => path.line_to(*end),
                CubicSegment::Curve(c1, c2, end) => path.curve_to(*c1, *c2, *end),
            }
        }
        if self.closed {
            path.close_path();
        }

        path
    }
}

pub struct CubicBezierBuilder<P = Point2D> {
    bezier: CubicBezier<P>,
}

impl<P: Copy> CubicBezierBuilder<P> {
    pub fn new(start: P) -> Self {
        CubicBezierBuilder {
            bezier: CubicBezier {
                start,
                segments: Vec::new(),
                closed: false,
            },
        }
    }

    pub fn line_to(&mut self, end: P) -> &mut Self {
        self.bezier.segments.push(CubicSegment::Line(end));
        self
    }

    pub fn curve_to(&mut self, control1: P, control2: P, end: P) -> &mut Self {
        self.bezier
            .segments
            .push(CubicSegment::Curve(control1, control2, end));
        self
    }

    pub fn close(&mut self) -> &mut Self {
        self.bezier.closed = true;
        self
    }

    pub fn segment_count_so_far(&self) -> usize {
        self.bezier.segments.len()
    }

    pub fn build(self) -> CubicBezier<P> {
        self.bezier
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PointPosition {
    Start,
    Control1,
    Control2,
    End,
}

pub struct CubicBezierPointIter<'a, P> {
    curve: &'a CubicBezier<P>,
    /// the next segment to be processed
    current_segment: usize,
    /// The index of the current point in the current segment.
    /// Line segments have 1 point, curve segments have 3 points.
    current_in_segment_idx: usize,
}

impl<P: Copy> Iterator for CubicBezierPointIter<'_, P> {
    type Item = (PointPosition, P);

    fn next(&mut self) -> Option<(PointPosition, P)> {
        use PointPosition::*;

        if self.current_segment > self.curve.segments.len() {
            return None;
        }
        if self.current_segment == 0 {
            self.current_segment += 1;
            return Some((Start, self.curve.start));
        }

        let seg = &self.curve.segments[self.current_segment - 1];
        let point = match seg {
            CubicSegment::Line(p) => {
                if self.current_in_segment_idx == 0 {
                    Some((End, *p))
                } else {
                    panic!("Invalid state: line segment has only one point");
                }
            }
            CubicSegment::Curve(p1, p2, p3) => match self.current_in_segment_idx {
                0 => Some((Control1, *p1)),
                1 => Some((Control2, *p2)),
                2 => Some((End, *p3)),
                _ => panic!("Invalid state: curve segment has only three points"),
            },
        };

        self.current_in_segment_idx += 1;
        if self.current_in_segment_idx == seg.points_count() {
            self.current_segment += 1;
            self.current_in_segment_idx = 0;
        }

        point
    }
}

#[test]
fn test_cubic_bezier_builder() {
    let mut curve = CubicBezier::builder(Point2D::new(0., 0.));
    curve.line_to(Point2D::new(1., 1.)).curve_to(
        Point2D::new(2., 2.),
        Point2D::new(3., 3.),
        Point2D::new(4., 4.),
    );
    let curve = curve.build();

    let points: Vec<_> = curve.iter().collect();
    assert_eq!(
        points,
        vec![
            (PointPosition::Start, Point2D::new(0., 0.)),
            (PointPosition::End, Point2D::new(1., 1.)),
            (PointPosition::Control1, Point2D::new(2., 2.)),
            (PointPosition::Control2, Point2D::new(3., 3.)),
            (PointPosition::End, Point2D::new(4., 4.)),
        ]
    );
}

#[test]
fn test_cubic_bezier_interpolation() {
    let mut curve = CubicBezier::builder(Point2D::new(0., 0.));
    curve.line_to(Point2D::new(1., 0.)).curve_to(
        Point2D::new(2., 0.),
        Point2D::new(2., 1.),
        Point2D::new(2., 2.),
    );
    let curve = curve.build();

    let points: Vec<_> = (0..2)
        .flat_map(|seg_id| {
            let curve = &curve;
            (0..=10).map(move |i| {
                let t = i as f64 / 10.;
                curve.point_at(seg_id, t)
            })
        })
        .collect();

    println!("{points:?}");
    assert_eq!(points.len(), 22);
    assert_eq!(points[0], Point2D::new(0., 0.));
    assert_eq!(points[1], Point2D::new(0.1, 0.));
    assert_eq!(points[11], Point2D::new(1., 0.));
    assert_eq!(points[21], Point2D::new(2., 2.));
}
