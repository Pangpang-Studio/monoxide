//! A higher-level representation of the character mapping within the `cmap` table.
//!
//! Code creating `cmap` table records should use types in this module, and then
//! convert it to various other formats using their respective modules, instead
//! of trying to instantiate the other formats directly.
use crate::model::encoding::{
    EncodingRecord, PlatformId, UnicodePlatformEncoding, WindowsPlatformEncoding,
};

/// A run of characters to be mapped sequentially to glyphs. A list of them
/// should be sorted by `start_code`.
pub struct SeqMapping {
    pub start_code: u32,
    pub len: u32,
    pub glyph_id: u32,
}

/// A subtable for a specific platform and encoding is encoded as a list of
/// character mappings.
pub type SubTable = Vec<SeqMapping>;

/// A high-level representation of the encoding of a subtable.
/// This encoding will be mapped in a one-to-many relationship with the
/// actual OpenType platform and encoding IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    /// Unicode scalars.
    ///
    /// This will map to both Unicode UCS-2 and UCS-4 encodings of Unicode and
    /// Microsoft platforms.
    Unicode,

    /// Unicode variation sequences. Unsupported for now.
    UnicodeVariations,

    /// Windows Symbol encoding.
    Symbol,
    /// Windows Shift-JIS encoding.
    ShiftJis,
    /// Windows PRC encoding.
    Prc,
    /// Windows Big5 encoding.
    Big5,
    /// Windows Wansung encoding.
    Wansung,
    /// Windows Johab encoding.
    Johab,
}

impl Encoding {
    /// The encoding record for this encoding, that should be used in the
    /// format 4 subtable.
    pub fn to_encoding_records_fmt4(&self) -> &'static [EncodingRecord] {
        match self {
            Encoding::Unicode => &[
                EncodingRecord {
                    platform_id: PlatformId::Unicode,
                    encoding_id: UnicodePlatformEncoding::V2Bmp as u16,
                },
                EncodingRecord {
                    platform_id: PlatformId::Microsoft,
                    encoding_id: WindowsPlatformEncoding::UnicodeUcs2 as u16,
                },
            ],
            Encoding::UnicodeVariations => &[],
            Encoding::Symbol => &[EncodingRecord {
                platform_id: PlatformId::Microsoft,
                encoding_id: WindowsPlatformEncoding::Symbol as u16,
            }],
            Encoding::ShiftJis => &[EncodingRecord {
                platform_id: PlatformId::Microsoft,
                encoding_id: WindowsPlatformEncoding::ShiftJis as u16,
            }],
            Encoding::Prc => &[EncodingRecord {
                platform_id: PlatformId::Microsoft,
                encoding_id: WindowsPlatformEncoding::Prc as u16,
            }],
            Encoding::Big5 => &[EncodingRecord {
                platform_id: PlatformId::Microsoft,
                encoding_id: WindowsPlatformEncoding::Big5 as u16,
            }],
            Encoding::Wansung => &[EncodingRecord {
                platform_id: PlatformId::Microsoft,
                encoding_id: WindowsPlatformEncoding::Wansung as u16,
            }],
            Encoding::Johab => &[EncodingRecord {
                platform_id: PlatformId::Microsoft,
                encoding_id: WindowsPlatformEncoding::Johab as u16,
            }],
        }
    }

    pub fn to_encoding_records_fmt12(&self) -> &'static [EncodingRecord] {
        match self {
            Encoding::Unicode => &[
                EncodingRecord {
                    platform_id: PlatformId::Unicode,
                    encoding_id: UnicodePlatformEncoding::V2Full as u16,
                },
                EncodingRecord {
                    platform_id: PlatformId::Microsoft,
                    encoding_id: WindowsPlatformEncoding::UnicodeUcs4 as u16,
                },
            ],
            Encoding::UnicodeVariations => &[],
            Encoding::Symbol => &[],
            Encoding::ShiftJis => &[],
            Encoding::Prc => &[],
            Encoding::Big5 => &[],
            Encoding::Wansung => &[],
            Encoding::Johab => &[],
        }
    }
}

/// A full-fledged `cmap` table is a list of subtables, each for a specific
/// platform and encoding.
pub struct Table {
    pub subtables: Vec<SubTable>,

    /// The mapping between encoding and their corresponding subtables
    /// indices in the `subtables` list.
    ///
    /// This mapping is used because multiple encodings can share the same
    /// subtable, e.g. Microsoft and Unicode both have UCS-2 tables and they
    /// can be shared.
    pub mapping: Vec<(Encoding, usize)>,
}
