use bytes::BufMut;
use thiserror::Error;

use crate::model::{f2dot14, fword};

use super::GlyphCommon;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Args {
    /// This component glyph should be offsetted by the given amount
    Offset { x: fword, y: fword },
    /// The control point in child component with index `child`
    /// (before renumbering) should be moved to the same place as
    /// the `parent`th point (after renumbering) currently added to
    /// the glyph.
    AlignCtrlPoints { parent: u16, child: u16 },
}

impl Args {
    pub fn in_short_format(&self) -> bool {
        match *self {
            Args::Offset { x, y } => {
                x >= i8::MIN as i16
                    && x <= i8::MAX as i16
                    && y >= i8::MIN as i16
                    && y <= i8::MAX as i16
            }
            Args::AlignCtrlPoints { parent, child } => {
                parent <= u8::MAX as u16 && child <= u8::MAX as u16
            }
        }
    }

    pub fn add_my_flags(&self, mut flags: ComponentFlags) -> ComponentFlags {
        if self.in_short_format() {
            flags |= ComponentFlags::ARG_1_AND_2_ARE_WORDS
        }
        if matches!(self, Self::Offset { .. }) {
            flags |= ComponentFlags::ARGS_ARE_XY_VALUES
        }
        flags
    }

    pub fn write(&self, w: &mut impl BufMut) {
        let short = self.in_short_format();
        match (short, self) {
            (true, &Args::Offset { x, y }) => {
                w.put_i8(x as i8);
                w.put_i8(y as i8)
            }
            (false, &Args::Offset { x, y }) => {
                w.put_i16(x);
                w.put_i16(y);
            }
            (true, &Args::AlignCtrlPoints { parent, child }) => {
                w.put_u8(parent as u8);
                w.put_u8(child as u8);
            }
            (false, &Args::AlignCtrlPoints { parent, child }) => {
                w.put_u16(parent);
                w.put_u16(child);
            }
        }
    }
}

pub enum Scale {
    One,
    Simple(f2dot14),
    XY {
        x: f2dot14,
        y: f2dot14,
    },
    /// Scale in a 2x2 matrix. Names are (component_axis, project_to_axis)
    TwoByTwo {
        xx: f2dot14,
        yx: f2dot14,
        xy: f2dot14,
        yy: f2dot14,
    },
}

impl Scale {
    pub fn add_my_flags(&self, flags: ComponentFlags) -> ComponentFlags {
        match self {
            Scale::One => flags,
            Scale::Simple(_) => flags | ComponentFlags::WE_HAVE_A_SCALE,
            Scale::XY { .. } => flags | ComponentFlags::WE_HAVE_AN_X_AND_Y_SCALE,
            Scale::TwoByTwo { .. } => flags | ComponentFlags::WE_HAVE_A_TWO_BY_TWO,
        }
    }

    pub fn write(&self, w: &mut impl BufMut) {
        match self {
            Scale::One => {}
            Scale::Simple(scale) => w.put_i16(scale.to_bits()),
            Scale::XY { x, y } => {
                w.put_i16(x.to_bits());
                w.put_i16(y.to_bits());
            }
            Scale::TwoByTwo { xx, yx, xy, yy } => {
                w.put_i16(xx.to_bits());
                w.put_i16(xy.to_bits());
                w.put_i16(yx.to_bits());
                w.put_i16(yy.to_bits());
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ComponentFlags(u16);
bitflags::bitflags! {
    impl ComponentFlags: u16 {
        /// Bit 0: If this is set, the arguments are 16-bit (uint16 or int16); otherwise, they are bytes (uint8 or int8).
        ///
        /// Should not be set by user.
        const ARG_1_AND_2_ARE_WORDS = 0x0001;
        /// Bit 1: If this is set, the arguments are signed xy values; otherwise, they are unsigned point numbers.
        ///
        /// Should not be set by user.
        const ARGS_ARE_XY_VALUES = 0x0002;
        /// Bit 2: If set and ARGS_ARE_XY_VALUES is also set, the xy values are rounded to the nearest grid line. Ignored if ARGS_ARE_XY_VALUES is not set.
        const ROUND_XY_TO_GRID = 0x0004;
        /// Bit 3: This indicates that there is a simple scale for the component. Otherwise, scale = 1.0.
        ///
        /// Should not be set by user.
        const WE_HAVE_A_SCALE = 0x0008;
        /// Bit 5: Indicates at least one more glyph after this one.
        ///
        /// Should not be set by user.
        const MORE_COMPONENTS = 0x0020;
        /// Bit 6: The x direction will use a different scale from the y direction.
        ///
        /// Should not be set by user.
        const WE_HAVE_AN_X_AND_Y_SCALE = 0x0040;
        /// Bit 7: There is a 2 by 2 transformation that will be used to scale the component.
        ///
        /// Should not be set by user.
        const WE_HAVE_A_TWO_BY_TWO = 0x0080;
        /// Bit 8: Following the last component are instructions for the composite glyph.
        ///
        /// Should not be set by user.
        const WE_HAVE_INSTRUCTIONS = 0x0100;
        /// Bit 9: If set, this forces the aw and lsb (and rsb) for the composite to be equal to those from this component glyph. This works for hinted and unhinted glyphs.
        const USE_MY_METRICS = 0x0200;
        /// Bit 10: If set, the components of the compound glyph overlap. Use of this flag is not required — that is, component glyphs may overlap without having this flag set. When used, it must be set on the flag word for the first component. Some rasterizer implementations may require fonts to use this flag to obtain correct behavior — see additional remarks, above, for the similar OVERLAP_SIMPLE flag used in simple-glyph descriptions.
        const OVERLAP_COMPOUND = 0x0400;
        /// Bit 11: The composite is designed to have the component offset scaled. Ignored if ARGS_ARE_XY_VALUES is not set.
        const SCALED_COMPONENT_OFFSET = 0x0800;
        /// Bit 12: The composite is designed not to have the component offset scaled. Ignored if ARGS_ARE_XY_VALUES is not set.
        const UNSCALED_COMPONENT_OFFSET = 0x1000;

        /// Reserved flags that should be set to 0.
        const RESERVED = 0xE010;
    }
}

impl ComponentFlags {
    /// Forbidden flags that should not set by user.
    pub fn forbidden() -> Self {
        Self::ARG_1_AND_2_ARE_WORDS
            | Self::ARGS_ARE_XY_VALUES
            | Self::WE_HAVE_A_SCALE
            | Self::WE_HAVE_AN_X_AND_Y_SCALE
            | Self::WE_HAVE_A_TWO_BY_TWO
            | Self::MORE_COMPONENTS
            | Self::WE_HAVE_INSTRUCTIONS
            | Self::RESERVED
    }
}

pub struct Component {
    pub flags: ComponentFlags,
    pub glyph_index: u16,
    pub args: Args,
    pub scale: Scale,
}

impl Component {
    pub fn write(&self, w: &mut impl BufMut, is_last: bool, has_instructions: bool) {
        let mut flags = self.flags;
        if !is_last {
            flags |= ComponentFlags::MORE_COMPONENTS
        }
        if has_instructions {
            flags |= ComponentFlags::WE_HAVE_INSTRUCTIONS
        }
        flags = self.args.add_my_flags(flags);
        flags = self.scale.add_my_flags(flags);

        w.put_u16(flags.bits());
        w.put_u16(self.glyph_index);
        self.args.write(w);
        self.scale.write(w);
    }
}

pub struct CompoundGlyph {
    // n_countours = -1
    pub common: GlyphCommon,
    pub components: Vec<Component>,
    pub instructions: Vec<u8>,
}

impl CompoundGlyph {
    pub fn verify(&self) -> Result<(), CompoundGlyphVerifyError> {
        if self.components.is_empty() {
            return Err(CompoundGlyphVerifyError::NoComponents);
        }

        for (idx, comp) in self.components.iter().enumerate() {
            if comp.flags.contains(ComponentFlags::forbidden()) {
                return Err(CompoundGlyphVerifyError::ForbiddenFlags(idx));
            }
        }

        Ok(())
    }

    pub fn write(&self, w: &mut impl BufMut) {
        let has_instructions = !self.instructions.is_empty();

        w.put_i16(-1);
        self.common.write(w);
        for (idx, comp) in self.components.iter().enumerate() {
            comp.write(w, idx == self.components.len() - 1, has_instructions);
        }
        if has_instructions {
            w.put_u16(self.instructions.len() as u16);
            w.put_slice(&self.instructions);
        }
    }
}

#[derive(Debug, Error)]
pub enum CompoundGlyphVerifyError {
    #[error("A compound glyph must have at least one component")]
    NoComponents,
    #[error("The flags for component {0} contain ones that are not allowed to be set by user")]
    ForbiddenFlags(usize),
}
