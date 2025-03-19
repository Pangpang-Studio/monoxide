//! High-level structures to generate glyph data.

use crate::model::{
    fword,
    glyf::{
        simple::{Coord, FlagOrRepeat, OutlineFlag, SimpleGlyph, SimpleGlyphVerifyError},
        GlyphCommon,
    },
};

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

            let (xc, xf) = encode_delta(dx, OutlineFlag::X_SHORT_VECTOR, OutlineFlag::LONG_X_SAME);
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

        glyph_data
            .end_points_of_countours
            .push(raw_flags.len() as u16 - 1);
    }

    // Simplify flags by using repeat flag
    let mut new_flags = Vec::new();
    let mut last_flag = raw_flags[0];
    let mut repeat_count = 0;
    for flag in raw_flags {
        if flag == last_flag && repeat_count < (u8::MAX as usize) {
            repeat_count += 1;
        } else {
            if repeat_count > 1 {
                new_flags.push(FlagOrRepeat::Repeat {
                    flag: last_flag,
                    times_minus_1: repeat_count as u8 - 1,
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
            times_minus_1: repeat_count as u8 - 1,
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
    } else if delta < 0 && (-delta) <= (u8::MAX as fword) {
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

pub fn decode(
    glyph: &SimpleGlyph,
) -> Result<Vec<QuadBezier<(fword, fword)>>, SimpleGlyphVerifyError> {
    glyph.verify()?;

    let mut decoder = BezierDecoder::new(glyph);
    Ok(decoder.decode())
}

/// A decoder for a simple glyph. All methods assume that the glyph has been
/// verified and is not malformed.
struct BezierDecoder<'a> {
    glyph: &'a SimpleGlyph,

    // current position
    x: fword,
    y: fword,

    /// The next countour end index to read
    countour_ix: usize,

    /// Total number of points in the glyph
    n_points: usize,
    /// Current point index. Note that this does not correspond to any specific
    /// index in the glyph's data. The actual index is tracked below.
    point_ix: usize,

    /// The next flag to read
    flag_ptr: usize,
    /// The current flag
    flag: OutlineFlag,
    /// The remaining number of times the current flag should be repeated
    flag_repeats_remaining: usize,
    /// The next X coordinate to read
    x_ptr: usize,
    /// The next Y coordinate to read
    y_ptr: usize,
}

impl<'a> BezierDecoder<'a> {
    fn new(glyph: &'a SimpleGlyph) -> Self {
        BezierDecoder {
            glyph,
            x: 0,
            y: 0,
            countour_ix: 0,
            n_points: glyph.n_points(),
            point_ix: 0,
            flag_ptr: 0,
            flag: OutlineFlag::empty(),
            flag_repeats_remaining: 0,
            x_ptr: 0,
            y_ptr: 0,
        }
    }

    #[allow(clippy::collapsible_else_if)]
    fn next_point(&mut self) -> (fword, fword, OutlineFlag) {
        if self.point_ix >= self.n_points {
            panic!("Point index out of bounds");
        }

        self.point_ix += 1;

        // Read flags
        if self.flag_repeats_remaining == 0 {
            let flag = self.glyph.flags[self.flag_ptr];
            self.flag = flag.get_flag();
            self.flag_repeats_remaining = flag.get_repeat_times();
            self.flag_ptr += 1;
        }
        self.flag_repeats_remaining -= 1;

        let flag = self.flag;

        // Read X coord
        let dx = if flag.contains(OutlineFlag::X_SHORT_VECTOR) {
            if flag.contains(OutlineFlag::SHORT_X_SIGN) {
                self.glyph.x_coords[self.x_ptr].unwrap_short() as i16
            } else {
                -(self.glyph.x_coords[self.x_ptr].unwrap_short() as i16)
            }
        } else {
            if flag.contains(OutlineFlag::LONG_X_SAME) {
                0
            } else {
                self.glyph.x_coords[self.x_ptr].unwrap_long()
            }
        };
        if flag.contains(OutlineFlag::X_SHORT_VECTOR) || !flag.contains(OutlineFlag::LONG_X_SAME) {
            self.x_ptr += 1;
        }

        // Read Y coord
        let dy = if flag.contains(OutlineFlag::Y_SHORT_VECTOR) {
            if flag.contains(OutlineFlag::SHORT_Y_SIGN) {
                self.glyph.y_coords[self.y_ptr].unwrap_short() as i16
            } else {
                -(self.glyph.y_coords[self.y_ptr].unwrap_short() as i16)
            }
        } else {
            if flag.contains(OutlineFlag::LONG_Y_SAME) {
                0
            } else {
                self.glyph.y_coords[self.y_ptr].unwrap_long()
            }
        };
        if flag.contains(OutlineFlag::Y_SHORT_VECTOR) || !flag.contains(OutlineFlag::LONG_Y_SAME) {
            self.y_ptr += 1;
        }

        self.x += dx as fword;
        self.y += dy as fword;
        let x_coord = self.x;
        let y_coord = self.y;

        (x_coord, y_coord, flag)
    }

    fn end_of_contour_p(&self) -> bool {
        self.point_ix > self.glyph.end_points_of_countours[self.countour_ix] as usize
    }

    fn decode(&mut self) -> Vec<QuadBezier<(fword, fword)>> {
        let mut curves = Vec::new();

        // Outer loop, iterating on contours
        while self.countour_ix < self.glyph.end_points_of_countours.len() {
            let curve = self.decode_curve();
            curves.push(curve.build());

            self.countour_ix += 1;
        }

        curves
    }

    fn decode_curve(&mut self) -> QuadBezierBuilder<(i16, i16)> {
        let mut curve;
        let start;
        // start point
        {
            let (x, y, flags) = self.next_point();
            assert!(
                flags.contains(OutlineFlag::ON_CURVE),
                "Curve with off-curve start point is not supported"
            );
            assert!(
                !self.end_of_contour_p(),
                "Contour with only one point is malformed"
            );
            curve = QuadBezier::builder((x, y));
            start = (x, y);
        }

        let mut off_curve = None;

        // inner loop, iterating points
        while !self.end_of_contour_p() {
            let (x, y, flags) = self.next_point();
            if flags.contains(OutlineFlag::ON_CURVE) {
                if let Some((cx, cy)) = off_curve {
                    curve.quad_to((cx, cy), (x, y));
                    off_curve = None;
                } else {
                    curve.line_to((x, y));
                }
            } else {
                match off_curve {
                    None => {
                        off_curve = Some((x, y));
                    }
                    Some((cx, cy)) => {
                        // Two consecutive off-curve points is technically
                        // possible in TrueType spec, although will not be
                        // generated by our encoder. In this case, the semantic
                        // is to add a virtual on-curve point at the midpoint of
                        // the two off-curve points.
                        let midx = (cx + x) / 2;
                        let midy = (cy + y) / 2;
                        curve.quad_to((cx, cy), (midx, midy));
                        off_curve = Some((x, y));
                    }
                }
            }
        }

        // Close the curve
        match off_curve {
            None => curve.line_to(start),
            Some((cx, cy)) => curve.quad_to((cx, cy), start),
        };
        curve.close();
        curve
    }
}

mod test;
