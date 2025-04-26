mod lang_id_ms;

use std::collections::HashMap;

use bytes::{BufMut, BytesMut};
use widestring::U16String;

use super::{
    encoding::{PlatformId, UnicodePlatformEncoding},
    ITable,
};

pub use lang_id_ms::MSLangID;

/// The version of the name table. Only version 1 is supported by this library.
const NAME_TABLE_VERSION: u16 = 0;

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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NameRecords {
    pub copyright: Option<String>,
    pub font_family_name: Option<String>,
    pub font_subfamily_name: Option<String>,
    pub unique_font_identifier: Option<String>,
    pub full_font_name: Option<String>,
    pub version: Option<String>,
    pub postscript_name: Option<String>,
    pub trademark: Option<String>,
    pub manufacturer: Option<String>,
    pub designer: Option<String>,
    pub description: Option<String>,
    pub vendor_url: Option<String>,
    pub designer_url: Option<String>,
    pub license_description: Option<String>,
    pub license_info_url: Option<String>,
    pub reserved: Option<String>,
    pub preferred_family: Option<String>,
    pub preferred_subfamily: Option<String>,
    pub compatible_full: Option<String>,
    pub sample_text: Option<String>,
    pub postscript_cid_findfont_name: Option<String>,
    pub wws_family_name: Option<String>,
    pub wws_subfamily_name: Option<String>,
    pub light_background_palette: Option<String>,
    pub dark_background_palette: Option<String>,
    pub variations_postscript_name_prefix: Option<String>,
}

macro_rules! opt_to_record {
    ($self:expr, $records:expr, $field_name:ident, $name_id_name:ident) => {
        if let Some(value) = &$self.$field_name {
            $records.push(NameRecord {
                name_id: NameId::$name_id_name,
                value: value.clone(),
            });
        }
    };
}

impl NameRecords {
    fn to_records(&self) -> Vec<NameRecord> {
        let mut records = Vec::new();
        opt_to_record!(self, records, copyright, Copyright);
        opt_to_record!(self, records, font_family_name, FontFamilyName);
        opt_to_record!(self, records, font_subfamily_name, FontSubfamilyName);
        opt_to_record!(self, records, unique_font_identifier, UniqueFontIdentifier);
        opt_to_record!(self, records, full_font_name, FullFontName);
        opt_to_record!(self, records, version, Version);
        opt_to_record!(self, records, postscript_name, PostscriptName);
        opt_to_record!(self, records, trademark, Trademark);
        opt_to_record!(self, records, manufacturer, Manufacturer);
        opt_to_record!(self, records, designer, Designer);
        opt_to_record!(self, records, description, Description);
        opt_to_record!(self, records, vendor_url, VendorURL);
        opt_to_record!(self, records, designer_url, DesignerURL);
        opt_to_record!(self, records, license_description, LicenseDescription);
        opt_to_record!(self, records, license_info_url, LicenseInfoURL);
        opt_to_record!(self, records, reserved, Reserved);
        opt_to_record!(self, records, preferred_family, PreferredFamily);
        opt_to_record!(self, records, preferred_subfamily, PreferredSubfamily);
        opt_to_record!(self, records, compatible_full, CompatibleFull);
        opt_to_record!(self, records, sample_text, SampleText);
        opt_to_record!(
            self,
            records,
            postscript_cid_findfont_name,
            PostscriptCIDFindfontName
        );
        opt_to_record!(self, records, wws_family_name, WWSFamilyName);
        opt_to_record!(self, records, wws_subfamily_name, WWSSubfamilyName);
        opt_to_record!(
            self,
            records,
            light_background_palette,
            LightBackgroundPalette
        );
        opt_to_record!(
            self,
            records,
            dark_background_palette,
            DarkBackgroundPalette
        );
        opt_to_record!(
            self,
            records,
            variations_postscript_name_prefix,
            VariationsPostscriptNamePrefix
        );
        records
    }
}

#[derive(Debug, Clone)]
pub struct NameRecord {
    pub name_id: NameId,
    // Written as (len, string_start_offset), both uint16
    pub value: String,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum Lang {
    /// Used by Unicode platform.
    ///
    /// There is no platform-specific encoding for the Unicode platform.
    /// As version-1 of the name table is not widely supported, the alternative
    /// format of using BCP-47 language tags with Unicode platform is not
    /// implemented in this library.
    ///
    /// Or actually, it has been implemented before, but it was rejected by the
    /// majority of commonly-used parsers, so it was removed.
    Unicode,

    /// Used by Microsoft platform.
    ///
    /// Please refer to the [`MSLangID`] enum for the list of language IDs.
    Microsoft(MSLangID),
}

pub struct Table {
    // version: u16 = 0
    pub records: HashMap<Lang, NameRecords>,
}

struct EncodedNameRecord {
    platform_id: u16,
    encoding_id: u16,
    language_id: u16,
    name_id: u16,
    name_length: u16,
    name_offset: u16,
}

impl ITable for Table {
    fn name(&self) -> &'static [u8; 4] {
        b"name"
    }

    fn write(&self, writer: &mut impl bytes::BufMut) {
        let mut pool = BytesMut::new();
        let mut name_records = Vec::new();

        for (lang, recs) in &self.records {
            let (platform_id, encoding_id, language_id) = match lang {
                Lang::Unicode => (
                    PlatformId::Unicode as u16,
                    UnicodePlatformEncoding::V2Full as u16,
                    0,
                ),
                Lang::Microsoft(mslang_id) => (PlatformId::Microsoft as u16, 1, *mslang_id as u16),
            };

            for rec in recs.to_records() {
                let rec_u16 = U16String::from_str(&rec.value);
                let rec_start = pool.len();
                let rec_len = rec_u16.as_vec().len() * 2;
                for ch in rec_u16.as_vec() {
                    pool.put_u16(*ch);
                }

                name_records.push(EncodedNameRecord {
                    platform_id,
                    encoding_id,
                    language_id,
                    name_id: rec.name_id as u16,
                    name_length: rec_len as u16,
                    name_offset: rec_start as u16,
                })
            }
        }

        name_records.sort_by_key(|x| (x.platform_id, x.encoding_id, x.language_id, x.name_id));

        let storage_offset = 3 * 2 // version, len, startOffset
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

        // Storage area
        writer.put_slice(&pool);
    }
}
