use super::ITable;

/// `loca` table, long format.
pub struct Table {
    /// The starting offset within the `glyf` table of each glyph
    pub offsets: Vec<u32>,
}

impl ITable for Table {
    fn name(&self) -> &'static [u8; 4] {
        b"loca"
    }

    fn write(&self, writer: &mut impl bytes::BufMut) {
        for &x in &self.offsets {
            writer.put_u32(x);
        }
    }
}
