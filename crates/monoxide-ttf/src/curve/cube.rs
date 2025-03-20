#[derive(Clone, Debug, PartialEq)]
pub enum CubicSegment<P> {
    Line(P),
    Curve(P, P, P),
}

impl<P> CubicSegment<P> {
    pub fn is_line(&self) -> bool {
        matches!(self, CubicSegment::Line(_))
    }

    pub fn points_count(&self) -> usize {
        match self {
            CubicSegment::Line(_) => 1,
            CubicSegment::Curve(_, _, _) => 3,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CubicBezier<P> {
    pub start: P,
    pub segments: Vec<CubicSegment<P>>,
    pub closed: bool,
}

impl<P: Copy> CubicBezier<P> {
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

    pub fn build(self) -> CubicBezier<P> {
        self.bezier
    }
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
    type Item = P;

    fn next(&mut self) -> Option<P> {
        if self.current_segment == self.curve.segments.len() {
            return None;
        }

        let seg = &self.curve.segments[self.current_segment];
        let point = match seg {
            CubicSegment::Line(p) => {
                if self.current_in_segment_idx == 0 {
                    Some(*p)
                } else {
                    panic!("Invalid state: line segment has only one point");
                }
            }
            CubicSegment::Curve(p1, p2, p3) => match self.current_in_segment_idx {
                0 => Some(*p1),
                1 => Some(*p2),
                2 => Some(*p3),
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
