use super::ITable;

#[derive(Clone, Copy, Debug)]
#[repr(u16)]
pub enum MaxZonesKind {
    DoesNotUseTwilightZone = 1,
    UsesTwilightZone = 2,
}

#[derive(Debug)]
pub struct Table {
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

impl ITable for Table {
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
