/// A quadratic bezier segment, with `P` as the point type
#[derive(Debug, Clone, PartialEq)]
pub struct QuadSegment<P> {
    /// The control point. If the segment is a straight line, this point should
    /// be the same as `end`.
    pub control: P,
    /// The end point of the segment.
    pub end: P,
}

impl<P: Eq> QuadSegment<P> {
    pub fn is_line(&self) -> bool {
        self.control == self.end
    }
}

/// A Bezier curve with `P` as the point type and `S` as the segment type.
#[derive(Debug, Clone, PartialEq)]
pub struct QuadBezier<P> {
    /// The start point of the curve.
    pub start: P,
    /// The segments of the curve.
    pub segments: Vec<QuadSegment<P>>,
    /// Whether the curve is closed.
    pub closed: bool,
}

impl<P: Copy> QuadBezier<P> {
    /// Converts the curve to a curve with a different point type.
    /// Often used to downsample a curve.
    pub fn cast<P1>(&self, cast: impl Fn(P) -> P1) -> QuadBezier<P1> {
        QuadBezier {
            start: cast(self.start),
            segments: self
                .segments
                .iter()
                .map(|seg| QuadSegment {
                    control: cast(seg.control),
                    end: cast(seg.end),
                })
                .collect(),
            closed: self.closed,
        }
    }

    pub fn iter(&self) -> QuadBezierPointIter<P> {
        QuadBezierPointIter {
            curve: self,
            current_segment: 0,
            is_off_curve: false,
        }
    }

    pub fn builder(start: P) -> QuadBezierBuilder<P> {
        QuadBezierBuilder::new(start)
    }
}

pub struct QuadBezierBuilder<P> {
    bezier: QuadBezier<P>,
}

impl<P: Copy> QuadBezierBuilder<P> {
    pub fn new(start: P) -> Self {
        QuadBezierBuilder {
            bezier: QuadBezier {
                start,
                segments: Vec::new(),
                closed: false,
            },
        }
    }

    pub fn line_to(&mut self, end: P) -> &mut Self {
        self.bezier.segments.push(QuadSegment { control: end, end });
        self
    }

    pub fn quad_to(&mut self, control: P, end: P) -> &mut Self {
        self.bezier.segments.push(QuadSegment { control, end });
        self
    }

    pub fn close(&mut self) -> &mut Self {
        self.bezier.closed = true;
        self
    }

    pub fn build(self) -> QuadBezier<P> {
        self.bezier
    }
}

pub struct QuadBezierPointIter<'a, P> {
    curve: &'a QuadBezier<P>,
    /// Start point is seg 0, segments start from 1
    current_segment: usize,
    /// Whether the upcoming point is the off-curve point of the current segment.
    /// The off-curve should be outputted before the on-curve point.
    is_off_curve: bool,
}

impl<P: Copy + Eq> Iterator for QuadBezierPointIter<'_, P> {
    type Item = (bool, P);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_segment == 0 {
            self.current_segment += 1;
            self.is_off_curve = true;
            Some((true, self.curve.start))
        } else {
            let seg_id = self.current_segment - 1;
            if seg_id >= self.curve.segments.len() {
                return None;
            }

            let seg = &self.curve.segments[seg_id];
            if seg.is_line() || !self.is_off_curve {
                // If we're on the last segment and it's a closed curve, we
                // should omit the last point.
                if seg_id == self.curve.segments.len() - 1 && self.curve.closed {
                    return None;
                }
                self.current_segment += 1;
                self.is_off_curve = true;
                Some((true, seg.end))
            } else {
                self.is_off_curve = false;
                Some((false, seg.control))
            }
        }
    }
}
