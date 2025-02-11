use std::collections::HashMap;

use bytes::{BufMut, BytesMut};
use widestring::U16String;

use super::ITable;

/// The version of the name table. Only version 1 is supported by this library.
const NAME_TABLE_VERSION: u16 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum NameId {
    Copyright = 0,
    FontFamilyName = 1,
    FontSubfamilyName = 2,
    UniqueFontIdentifier = 3,
    FullFontName = 4,
    Version = 5,
    PostscriptName = 6,
    Trademark = 7,
    Manufacturer = 8,
    Designer = 9,
    Description = 10,
    VendorURL = 11,
    DesignerURL = 12,
    LicenseDescription = 13,
    LicenseInfoURL = 14,
    Reserved = 15,
    PreferredFamily = 16,
    PreferredSubfamily = 17,
    CompatibleFull = 18,
    SampleText = 19,
    PostscriptCIDFindfontName = 20,
    WWSFamilyName = 21,
    WWSSubfamilyName = 22,
    LightBackgroundPalette = 23,
    DarkBackgroundPalette = 24,
    VariationsPostscriptNamePrefix = 25,
}

#[derive(Debug, Clone)]
pub struct NameRecord {
    pub name_id: NameId,
    // Written as (len, string_start_offset), both uint16
    pub value: String,
}

/// The language tag, in BCP-47 format
#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Lang(pub String);

pub struct Table {
    pub records: HashMap<Lang, Vec<NameRecord>>,
}

struct EncodedNameRecord {
    platform_id: u16,
    encoding_id: u16,
    language_id: u16,
    name_id: u16,
    name_length: u16,
    name_offset: u16,
}

struct InsertedString {
    length: u16,
    offset: u16,
}

impl ITable for Table {
    fn name(&self) -> &'static [u8; 4] {
        b"name"
    }

    fn write(&self, writer: &mut impl bytes::BufMut) {
        let mut pool = BytesMut::new();
        let mut lang_tags = Vec::new();
        let mut name_records = Vec::new();
        let lang_tag_start = 0x8000;

        for (lang, recs) in &self.records {
            let lang_u16 = U16String::from_str(&lang.0);
            let lang_start = pool.len();
            let lang_len = lang_u16.as_slice().len() * 2;
            for ch in lang_u16.as_slice() {
                pool.put_u16(*ch);
            }
            let tag_idx = lang_tags.len();
            lang_tags.push(InsertedString {
                length: lang_len as u16,
                offset: lang_start as u16,
            });
            let lang_tag = lang_tag_start + tag_idx;

            for rec in recs {
                let rec_u16 = U16String::from_str(&rec.value);
                let rec_start = pool.len();
                let rec_len = rec_u16.as_vec().len() * 2;
                for ch in rec_u16.as_vec() {
                    pool.put_u16(*ch);
                }

                name_records.push(EncodedNameRecord {
                    platform_id: 0, // Unicode
                    encoding_id: 4, // Unicode Full
                    language_id: lang_tag as u16,
                    name_id: rec.name_id as u16,
                    name_length: rec_len as u16,
                    name_offset: rec_start as u16,
                })
            }
        }

        name_records.sort_by_key(|x| (x.platform_id, x.encoding_id, x.language_id, x.name_id));

        let storage_offset = 4 * 2 // version, len, startOffset, langTagsLen
            + (2 * 2) * lang_tags.len() // lang tag entries { len, offset }
            + (6 * 2) * name_records.len(); // name record entries

        // Actual encoding
        writer.put_u16(NAME_TABLE_VERSION); // version
        writer.put_u16(name_records.len() as u16);
        writer.put_u16(storage_offset as u16);
        for rec in name_records {
            writer.put_u16(rec.platform_id);
            writer.put_u16(rec.encoding_id);
            writer.put_u16(rec.language_id);
            writer.put_u16(rec.name_id);
            writer.put_u16(rec.name_length);
            writer.put_u16(rec.name_offset);
        }
        writer.put_u16(lang_tags.len() as u16);
        for tag in lang_tags {
            writer.put_u16(tag.length);
            writer.put_u16(tag.offset);
        }

        // Storage area
        writer.put_slice(&pool);
    }
}
