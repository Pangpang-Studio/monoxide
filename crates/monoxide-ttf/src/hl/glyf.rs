//! High-level structures to generate glyph data.

use crate::model::{
    fword,
    glyf::{
        simple::{Coord, FlagOrRepeat, OutlineFlag, SimpleGlyph},
        GlyphCommon,
    },
};

/// A quadratic bezier segment, with `P` as the point type
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

#[derive(Debug, Clone, thiserror::Error)]
pub enum ConvertError {
    #[error("The {0}th segment is not closed")]
    SegmentNotClosed(usize),
}

/// Convert a list of quadratic bezier outlines to a simple glyph.
pub fn encode(outlines: &[QuadBezier<(fword, fword)>]) -> Result<SimpleGlyph, ConvertError> {
    let mut glyph_data = SimpleGlyph {
        common: GlyphCommon {
            x_max: 0,
            x_min: 0,
            y_max: 0,
            y_min: 0,
        },
        end_points_of_countours: Vec::new(),
        instructions: Vec::new(),
        flags: Vec::new(),
        x_coords: Vec::new(),
        y_coords: Vec::new(),
    };

    let mut last_x: fword = 0;
    let mut last_y: fword = 0;
    let mut raw_flags = Vec::new();
    for (i, outline) in outlines.iter().enumerate() {
        if !outline.closed {
            return Err(ConvertError::SegmentNotClosed(i));
        }

        for (on_curve, (x, y)) in outline.iter() {
            let dx = x - last_x;
            let dy = y - last_y;
            last_x = x;
            last_y = y;

            update_bb(&mut glyph_data, x, y);

            let (xc, xf) = encode_delta(dx, OutlineFlag::X_SHORT_VECTOR, OutlineFlag::LONG_Y_SAME);
            let (yc, yf) = encode_delta(dy, OutlineFlag::Y_SHORT_VECTOR, OutlineFlag::LONG_Y_SAME);
            let mut flags = xf | yf;
            if on_curve {
                flags |= OutlineFlag::ON_CURVE;
            }

            raw_flags.push(flags);
            if let Some(xc) = xc {
                glyph_data.x_coords.push(xc);
            }
            if let Some(yc) = yc {
                glyph_data.y_coords.push(yc);
            }
        }
    }

    // Simplify flags by using repeat flag
    let mut new_flags = Vec::new();
    let mut last_flag = OutlineFlag::empty();
    let mut repeat_count = 0;
    for flag in raw_flags {
        if flag == last_flag {
            repeat_count += 1;
        } else {
            if repeat_count > 1 {
                new_flags.push(FlagOrRepeat::Repeat {
                    flag: last_flag,
                    times_minus_1: repeat_count - 1,
                });
            } else {
                new_flags.push(FlagOrRepeat::Single(last_flag));
            }
            last_flag = flag;
            repeat_count = 1;
        }
    }
    if repeat_count > 1 {
        new_flags.push(FlagOrRepeat::Repeat {
            flag: last_flag,
            times_minus_1: repeat_count - 1,
        });
    } else {
        new_flags.push(FlagOrRepeat::Single(last_flag));
    }
    glyph_data.flags = new_flags;

    Ok(glyph_data)
}

fn encode_delta(
    delta: fword,
    short: OutlineFlag,
    sgn_same: OutlineFlag,
) -> (Option<Coord>, OutlineFlag) {
    if delta == 0 {
        (None, sgn_same)
    } else if delta > 0 && delta <= (u8::MAX as fword) {
        (Some(Coord::Short(delta as u8)), short | sgn_same)
    } else if (-delta) <= (u8::MAX as fword) {
        (Some(Coord::Short((-delta) as u8)), short)
    } else {
        (Some(Coord::Long(delta)), OutlineFlag::empty())
    }
}

fn update_bb(glyph_data: &mut SimpleGlyph, x: fword, y: fword) {
    glyph_data.common.x_max = glyph_data.common.x_max.max(x);
    glyph_data.common.x_min = glyph_data.common.x_min.min(x);
    glyph_data.common.y_max = glyph_data.common.y_max.max(y);
    glyph_data.common.y_min = glyph_data.common.y_min.min(y);
}
