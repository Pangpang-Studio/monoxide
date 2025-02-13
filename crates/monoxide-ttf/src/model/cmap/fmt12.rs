use bytes::BufMut;

use crate::{hl::cmap as hl, model::encoding::NON_MACINTOSH_LANG_CODE};

#[derive(Debug)]
pub struct SequentialMapGroup {
    pub start_code: u32,
    pub end_code: u32,
    pub start_glyph_id: u32,
}

#[derive(Debug)]
pub struct Table {
    // format: u16 = 12,
    // reserved: u16 = 0,
    // byte_length_including_header: u32,
    pub language: u32,
    // count: u32,
    pub groups: Vec<SequentialMapGroup>,
}

impl Table {
    pub fn byte_length(&self) -> usize {
        // format + reserved + byte_length_including_header + language + count
        let header_size = 2 + 2 + 4 + 4 + 4;
        // start_code + end_code + start_glyph_id
        let size_of_group = 3 * 4;
        header_size + self.groups.len() * size_of_group
    }

    pub fn write(&self, writer: &mut impl BufMut) {
        let size_of_table = self.byte_length() as u32;

        writer.put_u16(12); // format
        writer.put_u16(0); // reserved
        writer.put_u32(size_of_table);
        writer.put_u32(self.language);
        writer.put_u32(self.groups.len() as u32);
        for group in &self.groups {
            writer.put_u32(group.start_code);
            writer.put_u32(group.end_code);
            writer.put_u32(group.start_glyph_id);
        }
    }

    pub fn from_raw(raw: &hl::SubTable) -> Self {
        // The mapping is very straightforward, as the raw format is already
        // mimicking the format 12.
        let groups = raw
            .iter()
            .map(|mapping| SequentialMapGroup {
                start_code: mapping.start_code,
                end_code: mapping.start_code + mapping.len - 1,
                start_glyph_id: mapping.glyph_id,
            })
            .collect();
        let language = NON_MACINTOSH_LANG_CODE as u32;

        Table { language, groups }
    }
}
