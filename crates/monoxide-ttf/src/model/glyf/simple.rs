use bytes::BufMut;
use thiserror::Error;

use super::GlyphCommon;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Coord {
    Short(u8),
    Long(i16),
}

impl Coord {
    pub fn unwrap_short(self) -> u8 {
        match self {
            Coord::Short(x) => x,
            Coord::Long(_) => panic!("Expected a short coordinate, got a long one"),
        }
    }

    pub fn unwrap_long(self) -> i16 {
        match self {
            Coord::Short(_) => panic!("Expected a long coordinate, got a short one"),
            Coord::Long(x) => x,
        }
    }
}

impl From<i16> for Coord {
    fn from(value: i16) -> Self {
        Coord::Long(value)
    }
}

impl From<u8> for Coord {
    fn from(value: u8) -> Self {
        Coord::Short(value)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct OutlineFlag(u8);
bitflags::bitflags! {
    impl OutlineFlag: u8 {
        /// If set, the point is on the curve; otherwise, it is off the curve.
        const ON_CURVE = 0b00000001;
        /// If set, the corresponding x-coordinate is 1 byte long; otherwise, it is 2 bytes long.
        const X_SHORT_VECTOR = 0b00000010;
        /// If set, the corresponding y-coordinate is 1 byte long; otherwise, it is 2 bytes long.
        const Y_SHORT_VECTOR = 0b00000100;
        /// If set, the next byte specifies the number of additional times this set of flags is to be repeated.
        ///
        /// This flag should not be set by the user, and should be automatically deduced from the type.
        const REPEAT = 0b00001000;

        /// **Only if `X_SHORT_VECTOR` is unset.**
        /// This X coordinate is not present in the coordinate list, and should be the same as the previous one.
        const LONG_X_SAME = 0b00010000;
        /// **Only if `X_SHORT_VECTOR` is set.**
        /// Describes the sign of the value, with a value of 1 equalling positive and a zero value negative.
        const SHORT_X_SIGN = 0b00010000;
        /// **Only if `Y_SHORT_VECTOR` is unset.**
        /// This Y coordinate is not present in the coordinate list, and should be the same as the previous one.
        const LONG_Y_SAME = 0b00100000;
        /// **Only if `Y_SHORT_VECTOR` is set.**
        /// Describes the sign of the value, with a value of 1 equalling positive and a zero value negative.
        const SHORT_Y_SIGN = 0b00100000;
    }
}

impl std::fmt::Debug for OutlineFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_set();
        if self.intersects(OutlineFlag::ON_CURVE) {
            f.entry(&"ON_CURVE");
        }
        if self.intersects(OutlineFlag::X_SHORT_VECTOR) {
            f.entry(&"X_SHORT_VECTOR");
        }
        if self.intersects(OutlineFlag::Y_SHORT_VECTOR) {
            f.entry(&"Y_SHORT_VECTOR");
        }
        if self.intersects(OutlineFlag::REPEAT) {
            f.entry(&"REPEAT");
        }
        if self.intersects(OutlineFlag::LONG_X_SAME) {
            if self.intersects(OutlineFlag::X_SHORT_VECTOR) {
                f.entry(&"SHORT_X_SIGN");
            } else {
                f.entry(&"LONG_X_SAME");
            }
        }
        if self.intersects(OutlineFlag::LONG_Y_SAME) {
            if self.intersects(OutlineFlag::Y_SHORT_VECTOR) {
                f.entry(&"SHORT_Y_SIGN");
            } else {
                f.entry(&"LONG_Y_SAME");
            }
        }
        f.finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlagOrRepeat {
    Single(OutlineFlag),
    Repeat {
        flag: OutlineFlag,
        times_minus_1: u8,
    },
}

impl FlagOrRepeat {
    pub fn write(&self, writer: &mut impl BufMut) {
        match *self {
            Self::Single(flag) => writer.put_u8(flag.bits()),
            Self::Repeat {
                flag,
                times_minus_1,
            } => {
                let flag = flag | OutlineFlag::REPEAT;
                writer.put_u8(flag.bits());
                writer.put_u8(times_minus_1);
            }
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Self::Single(_) => 1,
            Self::Repeat { .. } => 2,
        }
    }

    pub fn get_flag(&self) -> OutlineFlag {
        match self {
            Self::Single(flag) => *flag,
            Self::Repeat { flag, .. } => *flag,
        }
    }

    pub fn get_repeat_times(&self) -> usize {
        match self {
            Self::Single(_) => 1,
            Self::Repeat { times_minus_1, .. } => (*times_minus_1 + 1) as usize,
        }
    }
}

impl From<OutlineFlag> for FlagOrRepeat {
    fn from(value: OutlineFlag) -> Self {
        Self::Single(value)
    }
}

#[derive(Debug, Clone)]
pub struct SimpleGlyph {
    // pub n_countours: u16, // encoded in the 1st field, as positive number
    pub common: GlyphCommon,
    /// Array of last points of each contour; array entries are point indices.
    /// The total number of points is determined by the last entry in this
    /// field.
    pub end_points_of_countours: Vec<u16>,
    pub instructions: Vec<u8>,
    pub flags: Vec<FlagOrRepeat>,
    /// Relative X coordinates
    pub x_coords: Vec<Coord>,
    /// Relative Y coordinates
    pub y_coords: Vec<Coord>,
}

#[derive(Error, Debug)]
pub enum SimpleGlyphVerifyError {
    #[error("The number of countours is too large to be represented!")]
    NCountourTooLarge,
    #[error("The `REPEAT` flag is set in the {0}th flag entry")]
    RepeatSetInFlags(usize),
    #[error("The number of flags is different from the number of points")]
    FlagCountMismatch,
    #[error("Too many coordinates in axis {axis}, expecting {count}")]
    TooManyCoords { axis: char, count: usize },
    #[error("Too few coordinates in axis {axis}, expecting at least {count}")]
    TooFewCoords { axis: char, count: usize },
    #[error("Format mismatch for coordinate {idx} in axis {axis}, expected is_short={expected}, got is_short={got}")]
    CoordFormatMismatch {
        axis: char,
        idx: usize,
        expected: bool,
        got: bool,
    },
}

impl SimpleGlyph {
    pub fn n_points(&self) -> usize {
        self.end_points_of_countours
            .last()
            .copied()
            .map_or(0, |x| x + 1) as usize
    }

    pub fn n_contours(&self) -> usize {
        self.end_points_of_countours.len()
    }

    pub fn verify(&self) -> Result<(), SimpleGlyphVerifyError> {
        use SimpleGlyphVerifyError::*;

        let n_countours = self.end_points_of_countours.len();
        if n_countours > i16::MAX as usize {
            return Err(NCountourTooLarge);
        }

        let n_points = self.n_points();

        let mut flag_count = 0;
        let mut x_coords_count = 0;
        let mut y_coords_count = 0;
        for (i, f) in self.flags.iter().enumerate() {
            let (flags, repeat_times) = match f {
                FlagOrRepeat::Single(f) => (f, 1),
                FlagOrRepeat::Repeat {
                    flag,
                    times_minus_1,
                } => (flag, times_minus_1 + 1),
            };
            if flags.intersects(OutlineFlag::REPEAT) {
                return Err(RepeatSetInFlags(i));
            }
            flag_count += repeat_times as usize;

            for _ in 0..repeat_times {
                if x_coords_count > self.x_coords.len() {
                    return Err(TooFewCoords {
                        axis: 'x',
                        count: x_coords_count,
                    });
                }
                if flags.intersects(OutlineFlag::X_SHORT_VECTOR) {
                    if !matches!(self.x_coords[x_coords_count], Coord::Short(_)) {
                        return Err(CoordFormatMismatch {
                            axis: 'x',
                            idx: x_coords_count,
                            expected: true,
                            got: false,
                        });
                    }
                    x_coords_count += 1;
                } else if flags.intersects(OutlineFlag::LONG_X_SAME) {
                } else {
                    if !matches!(self.x_coords[x_coords_count], Coord::Long(_)) {
                        return Err(CoordFormatMismatch {
                            axis: 'x',
                            idx: x_coords_count,
                            expected: false,
                            got: true,
                        });
                    }
                    x_coords_count += 1;
                }

                if y_coords_count > self.y_coords.len() {
                    return Err(TooFewCoords {
                        axis: 'y',
                        count: y_coords_count,
                    });
                }
                if flags.intersects(OutlineFlag::Y_SHORT_VECTOR) {
                    if !matches!(self.y_coords[y_coords_count], Coord::Short(_)) {
                        return Err(CoordFormatMismatch {
                            axis: 'y',
                            idx: y_coords_count,
                            expected: true,
                            got: false,
                        });
                    }
                    y_coords_count += 1;
                } else if flags.intersects(OutlineFlag::LONG_Y_SAME) {
                } else {
                    if !matches!(self.y_coords[y_coords_count], Coord::Long(_)) {
                        return Err(CoordFormatMismatch {
                            axis: 'y',
                            idx: y_coords_count,
                            expected: false,
                            got: true,
                        });
                    }
                    y_coords_count += 1;
                }
            }
        }
        if flag_count != n_points {
            return Err(FlagCountMismatch);
        }
        if x_coords_count != self.x_coords.len() {
            return Err(TooManyCoords {
                axis: 'x',
                count: x_coords_count,
            });
        }
        if y_coords_count != self.y_coords.len() {
            return Err(TooManyCoords {
                axis: 'y',
                count: y_coords_count,
            });
        }

        Ok(())
    }

    pub fn write(&self, w: &mut impl BufMut) {
        w.put_u16(self.end_points_of_countours.len() as u16);
        self.common.write(w);
        for &x in &self.end_points_of_countours {
            w.put_u16(x);
        }
        w.put_u16(self.instructions.len() as u16);
        for &i in &self.instructions {
            w.put_u8(i);
        }
        for &f in &self.flags {
            f.write(w);
        }
        for &x in &self.x_coords {
            match x {
                Coord::Short(x) => w.put_u8(x),
                Coord::Long(x) => w.put_i16(x),
            }
        }
        for &x in &self.y_coords {
            match x {
                Coord::Short(x) => w.put_u8(x),
                Coord::Long(x) => w.put_i16(x),
            }
        }
    }
}
