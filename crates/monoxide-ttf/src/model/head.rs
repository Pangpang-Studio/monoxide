//! Implementation of the OpenType `head` table.
//!
//! <https://learn.microsoft.com/zh-cn/typography/opentype/spec/head>
use std::time;

use bitflags::bitflags;

use super::ITable;

#[derive(Debug, Clone)]
pub struct HeaderFlags(u16);
bitflags! {
    impl HeaderFlags: u16 {
        const NOTHING = 0x0000;
        const BASELINE_Y0 = 0x0001;
        const LEFT_SIDEBEARING_X0 = 0x0002;
        const INSTRUCTIONS_DEPEND_ON_POINT_SIZE = 0x0004;
        const FORCE_PPEM_TO_INT = 0x0008;
        const INSTRUCTIONS_ALTER_ADVANCE_WIDTH = 0x0010;
        const LOSSLESS_FONT_DATA = 0x0800;
        const FONT_CONVERTED = 0x1000;
        const OPTIMIZED_FOR_CLEARTYPE = 0x2000;
        const LAST_RESORT_FONT = 0x4000;
    }
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum IndexToLocFormat {
    Short = 0,
    Long = 1,
}

#[derive(Debug, Clone)]
pub struct MacStyle(u16);
bitflags! {
    impl MacStyle: u16 {
        const REGULAR = 0x0000;
        const BOLD = 0x0001;
        const ITALIC = 0x0002;
        const UNDERLINE = 0x0004;
        const OUTLINE = 0x0008;
        const SHADOW = 0x0010;
        const CONDENSED = 0x0020;
        const EXTENDED = 0x0040;
    }
}

#[derive(Debug, Clone)]
pub struct Table {
    // pub major_version: u16, = 1
    // pub minor_version: u16, = 0
    pub font_revision: u32,
    /// Set to 0, will be rewritten in later stages
    pub checksum_adjustment: u32,
    // pub magic_number: u32,
    pub flags: HeaderFlags,
    pub units_per_em: u16,
    pub created: time::SystemTime,
    pub modified: time::SystemTime,
    pub x_min: i16,
    pub y_min: i16,
    pub x_max: i16,
    pub y_max: i16,
    pub mac_style: MacStyle,
    pub lowest_rec_ppem: u16,
    // pub font_direction_hint: i16, = 2
    // pub index_to_loc_format: IndexToLocFormat,
    // pub glyph_data_format: i16, = 0
}

const HEAD_MAJOR: u16 = 1;
const HEAD_MINOR: u16 = 0;
const MAGIC_NUMBER: u32 = 0x5F0F3CF5;

impl ITable for Table {
    fn name(&self) -> &'static [u8; 4] {
        b"head"
    }

    fn write(&self, writer: &mut impl bytes::BufMut) {
        let time_offset_base: time::SystemTime =
            time::UNIX_EPOCH - time::Duration::from_secs(2082844800);

        writer.put_u16(HEAD_MAJOR);
        writer.put_u16(HEAD_MINOR);
        writer.put_u32(self.font_revision);
        writer.put_u32(self.checksum_adjustment);
        // writer.put_u32(self.magic_number);
        writer.put_u32(MAGIC_NUMBER);
        writer.put_u16(self.flags.bits());
        writer.put_u16(self.units_per_em);
        // 	Number of seconds since 12:00 midnight that started January 1st, 1904, in
        // GMT/UTC time zone.
        writer.put_u64(
            self.created
                .duration_since(time_offset_base)
                .unwrap()
                .as_secs(),
        );
        writer.put_u64(
            self.modified
                .duration_since(time_offset_base)
                .unwrap()
                .as_secs(),
        );
        writer.put_i16(self.x_min);
        writer.put_i16(self.y_min);
        writer.put_i16(self.x_max);
        writer.put_i16(self.y_max);
        writer.put_u16(self.mac_style.bits());
        writer.put_u16(self.lowest_rec_ppem);
        // writer.put_i16(self.font_direction_hint);
        writer.put_i16(2);
        writer.put_u16(IndexToLocFormat::Long as u16);
        // writer.put_i16(self.glyph_data_format);
        writer.put_i16(0);
    }
}
