use num_traits::Num;

use super::Point;

#[derive(Clone, Debug, PartialEq)]
pub enum CubicSegment<P> {
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

#[deprecated = "Use flo_curves::bezier::CubicBezier instead"]
#[derive(Clone, Debug, PartialEq)]
pub struct CubicBezier<P> {
    pub start: P,
    pub segments: Vec<CubicSegment<P>>,
    pub closed: bool,
}

impl<P: Point<Scalar = N> + Copy, N: Num + Copy> CubicBezier<P> {
    pub fn cast<P1>(&self, cast: impl Fn(P) -> P1) -> CubicBezier<P1> {
        CubicBezier {
            start: cast(self.start),
            segments: self
                .segments
                .iter()
                .map(|seg| match seg {
                    CubicSegment::Line(p) => CubicSegment::Line(cast(*p)),
                    CubicSegment::Curve(p1, p2, p3) => {
                        CubicSegment::Curve(cast(*p1), cast(*p2), cast(*p3))
                    }
                })
                .collect(),
            closed: self.closed,
        }
    }

    pub fn iter(&self) -> CubicBezierPointIter<P> {
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

    pub fn segment(&self, idx: usize) -> Option<(P, CubicSegment<P>)> {
        if idx >= self.segments.len() {
            return None;
        }
        if idx == 0 {
            Some((self.start, self.segments[0].clone()))
        } else {
            let prev = self.segments[idx - 1].last_point();
            Some((prev, self.segments[idx].clone()))
        }
    }

    pub fn segment_iter(&self) -> impl Iterator<Item = (P, CubicSegment<P>)> + '_ {
        (0..self.segments.len()).filter_map(move |i| self.segment(i))
    }

    pub fn point_at(&self, segment: usize, t: N) -> P {
        let seg = &self.segments[segment];
        let start_point = if segment == 0 {
            self.start
        } else {
            self.segments[segment - 1].last_point()
        };

        match seg {
            CubicSegment::Line(end) => {
                let one_minus_t = N::one() - t;
                (start_point.mul_scalar(one_minus_t)).point_add(&end.mul_scalar(t))
            }
            CubicSegment::Curve(p1, p2, p3) => sample(start_point, *p1, *p2, *p3, t),
        }
    }
}

pub fn sample<P, N>(p1: P, p2: P, p3: P, p4: P, t: N) -> P
where
    P: Point<Scalar = N> + Copy,
    N: Num + Copy,
{
    let one_minus_t = N::one() - t;
    let three = N::one() + N::one() + N::one();
    let p0 = p1;
    let c0 = one_minus_t * one_minus_t * one_minus_t;
    let c1 = three * one_minus_t * one_minus_t * t;
    let c2 = three * one_minus_t * t * t;
    let c3 = t * t * t;
    (p0.mul_scalar(c0))
        .point_add(&p2.mul_scalar(c1))
        .point_add(&p3.mul_scalar(c2))
        .point_add(&p4.mul_scalar(c3))
}

pub struct CubicBezierBuilder<P> {
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
    let mut curve = CubicBezier::builder((0, 0));
    curve.line_to((1, 1)).curve_to((2, 2), (3, 3), (4, 4));
    let curve = curve.build();

    let points: Vec<_> = curve.iter().collect();
    assert_eq!(
        points,
        vec![
            (PointPosition::Start, (0, 0)),
            (PointPosition::End, (1, 1)),
            (PointPosition::Control1, (2, 2)),
            (PointPosition::Control2, (3, 3)),
            (PointPosition::End, (4, 4)),
        ]
    );
}

#[test]
fn test_cubic_bezier_interpolation() {
    let mut curve = CubicBezier::<(f64, f64)>::builder((0., 0.));
    curve
        .line_to((1., 0.))
        .curve_to((2., 0.), (2., 1.), (2., 2.));
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

    println!("{:?}", points);
    assert_eq!(points.len(), 22);
    assert_eq!(points[0], (0., 0.));
    assert_eq!(points[1], (0.1, 0.));
    assert_eq!(points[11], (1., 0.));
    assert_eq!(points[21], (2., 2.));
}
