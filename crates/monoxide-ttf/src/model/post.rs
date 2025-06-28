//! The `post` table.

use super::{Fixed, ITable, fword};

pub struct TableV3 {
    // version: u32 = 0x00030000,
    pub italic_angle: Fixed,
    pub underline_position: fword,
    pub underline_thickness: fword,
    pub is_fixed_pitch: bool, // u32

    /// Memory usage of the font.
    ///
    /// set to 0 if unknown.
    pub min_mem_type42: u32,
    pub max_mem_type42: u32,
    pub min_mem_type1: u32,
    pub max_mem_type1: u32,
}

impl TableV3 {
    pub const VERSION: u32 = 0x00030000;
}

impl ITable for TableV3 {
    fn name(&self) -> &'static [u8; 4] {
        b"post"
    }

    fn write(&self, writer: &mut impl bytes::BufMut) {
        writer.put_u32(Self::VERSION);
        writer.put_i32(self.italic_angle.to_bits());
        writer.put_i16(self.underline_position);
        writer.put_i16(self.underline_thickness);
        writer.put_u32(if self.is_fixed_pitch { 1 } else { 0 });
        writer.put_u32(self.min_mem_type42);
        writer.put_u32(self.max_mem_type42);
        writer.put_u32(self.min_mem_type1);
        writer.put_u32(self.max_mem_type1);
    }
}
