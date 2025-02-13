//! Platform and encoding definitions used in `cmap` and `name` tables.

/// The platform IDs for subtables of the `cmap` table.
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlatformId {
    Unicode = 0,
    Microsoft = 3,

    #[deprecated = "The use of 'Macintosh' platform ID is discouraged. It is not supported by Monoxide."]
    Macintosh = 1,
}

/// The encoding IDs for the Unicode platform ([`PlatformId::Unicode`]).
///
/// In practice, we should only use [`UnicodePlatformEncoding::V2Bmp`] and
/// [`UnicodePlatformEncoding::V2Full`] for modern fonts.
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnicodePlatformEncoding {
    /// Unicode 2.0 and later, BMP only
    V2Bmp = 3,
    /// Unicode 2.0 and later, full repertoire
    V2Full = 4,
    /// Unicode Variation Sequences, only for format 14
    VariationSequences = 5,
    /// Last Resort Font, only for format 13
    LastResort = 6,

    #[deprecated]
    V1_0 = 0,
    #[deprecated]
    V1_1 = 1,
    #[deprecated]
    Iso10416_1993 = 2,
}

/// The encoding IDs for the Microsoft platform ([`PlatformId::Microsoft`]).
///
/// In practice, we should only use [`WindowsPlatformEncoding::UnicodeUcs2`] and
/// [`WindowsPlatformEncoding::UnicodeUcs4`] for modern fonts.
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WindowsPlatformEncoding {
    Symbol = 0,
    UnicodeUcs2 = 1,
    ShiftJis = 2,
    Prc = 3,
    Big5 = 4,
    Wansung = 5,
    Johab = 6,
    UnicodeUcs4 = 10,
}

/// The language code to use when not using the Macintosh platform.
///
/// Since this implementation actually doesn't support the Macintosh platform
/// at all, this constant is used to fill in the language code field in all
/// `cmap` subtables.
pub const NON_MACINTOSH_LANG_CODE: u16 = 0;

/// Representing a platform and encoding pair.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncodingRecord {
    pub platform_id: PlatformId,
    pub encoding_id: u16,
}
