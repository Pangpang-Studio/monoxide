use super::ITable;

/// `maxp` table version 0.5. For use with CFF/CFF2 outlines.
///
/// For usage with TrueType outlines, use [`TableV1`].
#[derive(Debug)]
pub struct TableV0_5 {
    pub n_glyphs: u16,
}

impl ITable for TableV0_5 {
    fn name(&self) -> &'static [u8; 4] {
        b"maxp"
    }

    fn write(&self, writer: &mut impl bytes::BufMut) {
        let version = 0x00005000u32;
        writer.put_u32(version);
        writer.put_u16(self.n_glyphs);
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(u16)]
pub enum MaxZonesKind {
    #[default]
    DoesNotUseTwilightZone = 1,
    UsesTwilightZone = 2,
}

/// `maxp` table version 1. For use with TrueType outlines.
///
/// For usage with CFF/CFF2 outlines, use [`TableV0_5`].
#[derive(Debug, Default)]
pub struct TableV1 {
    pub n_glyphs: u16,
    pub max_points: u16,
    pub max_contours: u16,
    pub max_composite_points: u16,
    pub max_composite_contours: u16,
    pub max_zones: MaxZonesKind,
    pub max_twilight_points: u16,
    pub max_storage: u16,
    pub max_func_defs: u16,
    pub max_instruction_defs: u16,
    pub max_stack_elements: u16,
    pub max_size_of_instructions: u16,
    pub max_component_elements: u16,
    pub max_component_depth: u16,
}

impl ITable for TableV1 {
    fn name(&self) -> &'static [u8; 4] {
        b"maxp"
    }

    fn write(&self, writer: &mut impl bytes::BufMut) {
        let version = 0x00010000u32;
        writer.put_u32(version);
        writer.put_u16(self.n_glyphs);
        writer.put_u16(self.max_points);
        writer.put_u16(self.max_contours);
        writer.put_u16(self.max_composite_points);
        writer.put_u16(self.max_composite_contours);
        writer.put_u16(self.max_zones as u16);
        writer.put_u16(self.max_twilight_points);
        writer.put_u16(self.max_storage);
        writer.put_u16(self.max_func_defs);
        writer.put_u16(self.max_instruction_defs);
        writer.put_u16(self.max_stack_elements);
        writer.put_u16(self.max_size_of_instructions);
        writer.put_u16(self.max_component_elements);
        writer.put_u16(self.max_component_depth);
    }
}
