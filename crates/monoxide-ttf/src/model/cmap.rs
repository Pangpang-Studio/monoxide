//! Model of the `cmap` table.
//!
//! The `cmap` table has a lot of different formats due to historical reasons.
//! This implementation, however, currently only support writing formats 4 and
//! 12, as they are the most commonly used today. Support for format 13 might be
//! added in the future.
//!
//! The following quote comes from Microsoft's OpenType specification:
//!
//! > Of the seven available formats, not all are commonly used today. Formats 4
//! > or 12 are appropriate for most new fonts, depending on the Unicode character
//! > repertoire supported. Format 14 is used in many applications for support of
//! > Unicode variation sequences. Some platforms also make use for format 13 for
//! > a last-resort fallback font. Other subtable formats are not recommended for
//! > use in new fonts. Application developers, however, should anticipate that
//! > any of the formats may be used in fonts.
//!
//! <https://learn.microsoft.com/zh-cn/typography/opentype/spec/cmap>

pub mod fmt12;
pub mod fmt4;

use super::encoding::{EncodingRecord, PlatformId};

/// Representing a subtable for a specific platform and encoding.
pub enum Subtable {
    Format4(fmt4::Table),
    Format12(fmt12::Table),
}

impl Subtable {
    pub fn byte_length(&self) -> usize {
        match self {
            Subtable::Format4(table) => table.byte_length(),
            Subtable::Format12(table) => table.byte_length(),
        }
    }

    pub fn write(&self, writer: &mut impl bytes::BufMut) {
        match self {
            Subtable::Format4(table) => table.write(writer),
            Subtable::Format12(table) => table.write(writer),
        }
    }
}

/// A runtime representation of the `cmap` table.
pub struct Table {
    /// The list of subtables that will be written in order.
    pub subtables: Vec<Subtable>,

    /// The mapping between encoding and their corresponding subtables
    /// indices in the `subtables` list.
    ///
    /// This mapping is used because multiple encodings can share the same
    /// subtable, e.g. Microsoft and Unicode both have UCS-2 tables and they
    /// can be shared.
    pub mapping: Vec<(EncodingRecord, usize)>,
}

use super::ITable;

impl Table {
    /// Converts a raw `cmap` table ([`raw::Table`]) to the concrete runtime
    /// representation ([`Table`]).
    ///
    /// This function will generate the correct format 4 and 12 subtables based
    /// on the input raw table. Subtable sharing is explicitly supported here.
    pub fn from_raw(mut raw: crate::hl::cmap::Table) -> Self {
        // Ensure that the character mappings within each subtable is sorted.
        for subtable in &mut raw.subtables {
            subtable.sort_by_key(|x| x.start_code);
        }

        // Freeze the raw table to prevent further modifications.
        let raw = raw;

        let mut subtables = Vec::new();
        let mut mapping = Vec::new();

        for &(encoding, tbl_index) in &raw.mapping {
            let tbl = &raw.subtables[tbl_index];

            let fmt4_tbls = encoding.to_encoding_records_fmt4();
            if !fmt4_tbls.is_empty() {
                let idx = subtables.len();
                let subtable = Subtable::Format4(fmt4::Table::from_raw(tbl));
                subtables.push(subtable);
                for fmt4_tbl in fmt4_tbls {
                    mapping.push((*fmt4_tbl, idx));
                }
            }

            let fmt12_tbls = encoding.to_encoding_records_fmt12();
            if !fmt12_tbls.is_empty() {
                let idx = subtables.len();
                let subtable = Subtable::Format12(fmt12::Table::from_raw(tbl));
                subtables.push(subtable);
                for fmt12_tbl in fmt12_tbls {
                    mapping.push((*fmt12_tbl, idx));
                }
            }
        }

        // The mapping should be sorted by platform ID and encoding ID,
        // as required by the OpenType specification.
        mapping.sort_by_key(|x| (x.0.platform_id as u16, x.0.encoding_id));

        Self { subtables, mapping }
    }
}

impl ITable for Table {
    fn name(&self) -> &'static [u8; 4] {
        b"cmap"
    }

    fn write(&self, writer: &mut impl bytes::BufMut) {
        // To write the header entries, we need to calculate the offset of each
        // subtable first.
        //
        // The table header is 4 bytes long:
        // - version: u16
        // - num_tables: u16
        let size_header = 4;
        // Each encoding record is 8 bytes long:
        // - platform_id: u16
        // - encoding_id: u16
        // - offset: u32
        let size_encoding_record = 8;
        let total_size_encoding_records = self.mapping.len() * size_encoding_record;
        let subtables_start_offset = size_header + total_size_encoding_records;
        // Now we can calculate the offset for each table.
        let mut subtable_offsets = Vec::new();
        let mut curr_offset = subtables_start_offset;
        for subtable in &self.subtables {
            subtable_offsets.push(curr_offset);
            curr_offset += subtable.byte_length();
        }

        // Write header
        writer.put_u16(0); // version
        writer.put_u16(self.mapping.len() as u16); // num_tables

        // Write encoding records
        for tbl in &self.mapping {
            let (encoding, idx) = tbl;
            writer.put_u16(encoding.platform_id as u16);
            writer.put_u16(encoding.encoding_id);
            writer.put_u32(subtable_offsets[*idx] as u32);
        }

        // Write subtables
        for subtable in &self.subtables {
            subtable.write(writer);
        }
    }
}
