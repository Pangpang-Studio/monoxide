use bitflags::bitflags;
use bytes::BufMut;

use super::{fword, ufword, ITable};

#[repr(u16)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WeightClass {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Regular = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
}

#[repr(u16)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WidthClass {
    UltraCondensed = 1,
    ExtraCondensed = 2,
    Condensed = 3,
    SemiCondensed = 4,
    Normal = 5,
    SemiExpanded = 6,
    Expanded = 7,
    ExtraExpanded = 8,
    UltraExpanded = 9,
}

#[repr(u16)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UsagePermissionKind {
    InstallableEmbedding = 0,
    RestrictedLicenseEmbedding = 2,
    PreviewPrintEmbedding = 4,
    EditableEmbedding = 8,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FsTypeUpper(u16);

bitflags::bitflags! {
    impl FsTypeUpper: u16 {
        const Nothing = 0x0000;
        const NoSubsetting = 0x0100;
        const BitmapEmbeddingOnly = 0x0200;
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BareFamilyClass {
    NoClassification = 0,
    OldStyleSerifs = 1,
    TransitionalSerifs = 2,
    ModernSerifs = 3,
    ClarendonSerifs = 4,
    SlabSerifs = 5,
    FreeformSerifs = 7,
    SansSerif = 8,
    Ornamentals = 9,
    Scripts = 10,
    Symbolic = 12,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OldStyleSerifKind {
    NoClassification = 0,
    IBMRoundedLegibility = 1,
    Garalde = 2,
    Venetian = 3,
    ModifiedVenetian = 4,
    DutchModern = 5,
    DutchTraditional = 6,
    Contemporary = 7,
    Calligraphic = 8,
    Miscellaneous = 15,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TransitionalSerifKind {
    NoClassification = 0,
    DirectLine = 1,
    Script = 2,
    Miscellaneous = 15,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ModernSerifKind {
    NoClassification = 0,
    Italian = 1,
    Script = 2,
    Miscellaneous = 15,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ClarendonSerifKind {
    NoClassification = 0,
    Clarendon = 1,
    Modern = 2,
    Traditional = 3,
    Newspaper = 4,
    StubSerif = 5,
    Monotone = 6,
    Typewriter = 7,
    Miscellaneous = 15,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SlabSerifKind {
    NoClassification = 0,
    Monotone = 1,
    Humanist = 2,
    Geometric = 3,
    Swiss = 4,
    Typewriter = 5,
    Miscellaneous = 15,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FreeformSerifKind {
    NoClassification = 0,
    Modern = 1,
    Miscellaneous = 15,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SansSerifKind {
    NoClassification = 0,
    IBMNeoGrotesqueGothic = 1,
    Humanist = 2,
    LowXRoundGeometric = 3,
    HighXRoundGeometric = 4,
    NeoGrotesqueGothic = 5,
    ModifiedNeoGrotesqueGothic = 6,
    TypewriterGothic = 9,
    Matrix = 10,
    Miscellaneous = 15,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OrnamentalKind {
    NoClassification = 0,
    Engraver = 1,
    BlackLetter = 2,
    Decorative = 3,
    ThreeDimensional = 4,
    Miscellaneous = 15,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScriptKind {
    NoClassification = 0,
    Uncial = 1,
    BrushJoined = 2,
    FormalJoined = 3,
    MonotoneJoined = 4,
    Calligraphic = 5,
    BrushUnjoined = 6,
    FormalUnjoined = 7,
    MonotoneUnjoined = 8,
    Miscellaneous = 15,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SymbolicKind {
    NoClassification = 0,
    MixedSerif = 3,
    OldstyleSerif = 6,
    NeoGrotesqueSansSerif = 7,
    Miscellaneous = 15,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SFamilyClass {
    NoClassification,
    OldStyleSerifs(OldStyleSerifKind),
    TransitionalSerifs(TransitionalSerifKind),
    ModernSerifs(ModernSerifKind),
    ClarendonSerifs(ClarendonSerifKind),
    SlabSerifs(SlabSerifKind),
    FreeformSerifs(FreeformSerifKind),
    SansSerif(SansSerifKind),
    Ornamentals(OrnamentalKind),
    Scripts(ScriptKind),
    Symbolic(SymbolicKind),
}

impl SFamilyClass {
    pub fn to_int(&self) -> u16 {
        match self {
            SFamilyClass::NoClassification => 0,
            SFamilyClass::OldStyleSerifs(kind) => 1 << 8 | *kind as u16,
            SFamilyClass::TransitionalSerifs(kind) => 2 << 8 | *kind as u16,
            SFamilyClass::ModernSerifs(kind) => 3 << 8 | *kind as u16,
            SFamilyClass::ClarendonSerifs(kind) => 4 << 8 | *kind as u16,
            SFamilyClass::SlabSerifs(kind) => 5 << 8 | *kind as u16,
            SFamilyClass::FreeformSerifs(kind) => 7 << 8 | *kind as u16,
            SFamilyClass::SansSerif(kind) => 8 << 8 | *kind as u16,
            SFamilyClass::Ornamentals(kind) => 9 << 8 | *kind as u16,
            SFamilyClass::Scripts(kind) => 10 << 8 | *kind as u16,
            SFamilyClass::Symbolic(kind) => 12 << 8 | *kind as u16,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PanroseClassification {
    pub family_type: u8,
    pub serif_style: u8,
    pub weight: u8,
    pub proportion: u8,
    pub contrast: u8,
    pub stroke_variation: u8,
    pub arm_style: u8,
    pub letterform: u8,
    pub midline: u8,
    pub x_height: u8,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FsSelectionKind(u16);
bitflags::bitflags! {
    impl FsSelectionKind: u16 {
        const Italic = 0x0001;
        const Underscore = 0x0002;
        const Negative = 0x0004;
        const Outlined = 0x0008;
        const Strikeout = 0x0010;
        const Bold = 0x0020;
        const Regular = 0x0040;
        const UseTypoMetrics = 0x0080;
        const WWS = 0x0100;
        const Oblique = 0x0200;
    }
}

macro_rules! unicode_range {
    (
        struct $name:ident
        {$(
            $(#[$attr:meta])*
            $range_name:ident ($bit:expr, $range:pat)
        ),*$(,)?}
    ) => {
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub struct $name(u128);

        bitflags::bitflags! {
            impl $name: u128 {
                $(
                    $(#[$attr])*
                    const $range_name = 1 << $bit;
                )*
            }
        }

        impl $name {
            /// Set the bit for the given character. Might simultaneously set multiple bits.
            pub fn add_bits_from_range(mut self, ch: char) -> Self {
                let ch = ch as u32;
                $(if matches!(ch, $range) { self.0 |= 1 << $bit; })*
                self
            }
        }
    };
}

unicode_range! {
    struct UnicodeRange {
        BasicLatin(0, 0x0000..=0x007F),
        Latin1Supplement(1, 0x0080..=0x00FF),
        LatinExtendedA(2, 0x0100..=0x017F),
        LatinExtendedB(3, 0x0180..=0x024F),
        IPAExtensions(4, 0x0250..=0x02AF),
        PhoneticExtensions(5, 0x1D00..=0x1D7F),
        PhoneticExtensionsSupplement(6, 0x1D80..=0x1DBF),
        SpacingModifierLetters(7, 0x02B0..=0x02FF),
        ModifierToneLetters(8, 0xA700..=0xA71F),
        CombiningDiacriticalMarks(9, 0x0300..=0x036F),
        CombiningDiacriticalMarksSupplement(10, 0x1DC0..=0x1DFF),
        GreekAndCoptic(11, 0x0370..=0x03FF),
        Coptic(12, 0x2C80..=0x2CFF),
        Cyrillic(13, 0x0400..=0x04FF),
        CyrillicSupplement(14, 0x0500..=0x052F),
        CyrillicExtendedA(15, 0x2DE0..=0x2DFF),
        CyrillicExtendedB(16, 0xA640..=0xA69F),
        Armenian(17, 0x0530..=0x058F),
        Hebrew(18, 0x0590..=0x05FF),
        Vai(19, 0xA500..=0xA63F),
        Arabic(20, 0x0600..=0x06FF),
        ArabicSupplement(21, 0x0750..=0x077F),
        NKo(22, 0x07C0..=0x07FF),
        Kannada(22, 0x0C80..=0x0CFF),
        Malayalam(23, 0x0D00..=0x0D7F),
        Thai(24, 0x0E00..=0x0E7F),
        Lao(25, 0x0E80..=0x0EFF),
        Georgian(26, 0x10A0..=0x10FF),
        GeorgianSupplement(26, 0x2D00..=0x2D2F),
        Balinese(27, 0x1B00..=0x1B7F),
        HangulJamo(28, 0x1100..=0x11FF),
        LatinExtendedAdditional(29, 0x1E00..=0x1EFF),
        LatinExtendedC(29, 0x2C60..=0x2C7F),
        LatinExtendedD(29, 0xA720..=0xA7FF),
        GreekExtended(30, 0x1F00..=0x1FFF),
        GeneralPunctuation(31, 0x2000..=0x206F),
        SupplementalPunctuation(31, 0x2E00..=0x2E7F),
        SuperscriptsAndSubscripts(32, 0x2070..=0x209F),
        CurrencySymbols(33, 0x20A0..=0x20CF),
        CombiningDiacriticalMarksForSymbols(34, 0x20D0..=0x20FF),
        LetterlikeSymbols(35, 0x2100..=0x214F),
        NumberForms(36, 0x2150..=0x218F),
        Arrows(37, 0x2190..=0x21FF | 0x27F0..=0x27FF | 0x2900..=0x297F),
        MiscellaneousSymbolsAndArrows(37, 0x2B00..=0x2BFF),
        MathematicalOperators(38, 0x2200..=0x22FF | 0x2A00..=0x2AFF | 0x27C0..=0x27EF | 0x2980..=0x29FF),
        MiscellaneousTechnical(39, 0x2300..=0x23FF),
        ControlPictures(40, 0x2400..=0x243F),
        OpticalCharacterRecognition(41, 0x2440..=0x245F),
        EnclosedAlphanumerics(42, 0x2460..=0x24FF),
        BoxDrawing(43, 0x2500..=0x257F),
        BlockElements(44, 0x2580..=0x259F),
        GeometricShapes(45, 0x25A0..=0x25FF),
        MiscellaneousSymbols(46, 0x2600..=0x26FF),
        Dingbats(47, 0x2700..=0x27BF),
        CJKSymbolsAndPunctuation(48, 0x3000..=0x303F),
        Hiragana(49, 0x3040..=0x309F),
        Katakana(50, 0x30A0..=0x30FF),
        KatakanaPhoneticExtensions(50, 0x31F0..=0x31FF),
        Bopomofo(51, 0x3100..=0x312F),
        BopomofoExtended(51, 0x31A0..=0x31BF),
        HangulCompatibilityJamo(52, 0x3130..=0x318F),
        Phagspa(53, 0xA840..=0xA87F),
        EnclosedCJKLettersAndMonths(54, 0x3200..=0x32FF),
        CJKCompatibility(55, 0x3300..=0x33FF),
        HangulSyllables(56, 0xAC00..=0xD7AF),
        NonPlane0(57, 0x10000..=0x10FFFF),
        Phoenician(58, 0x10900..=0x1091F),
        CJKUnifiedIdeographs(59, 0x4E00..=0x9FFF),
        CJKRadicalsSupplement(59, 0x2E80..=0x2EFF),
        KangxiRadicals(59, 0x2F00..=0x2FDF),
        IdeographicDescriptionCharacters(59, 0x2FF0..=0x2FFF),
        CJKUnifiedIdeographsExtensionA(59, 0x3400..=0x4DBF),
        CJKUnifiedIdeographsExtensionB(59, 0x20000..=0x2A6DF),
        Kanbun(59, 0x3190..=0x319F),
        PrivateUseAreaPlane0(60, 0xE000..=0xF8FF),
        CJKStrokes(61, 0x31C0..=0x31EF),
        CJKCompatibilityIdeographs(61, 0xF900..=0xFAFF),
        CJKCompatibilityIdeographsSupplement(61, 0x2F800..=0x2FA1F),
        AlphabeticPresentationForms(62, 0xFB00..=0xFB4F),
        ArabicPresentationFormsA(63, 0xFB50..=0xFDFF),
        CombiningHalfMarks(64, 0xFE20..=0xFE2F),
        VerticalForms(65, 0xFE10..=0xFE1F),
        CJKCompatibilityForms(65, 0xFE30..=0xFE4F),
        SmallFormVariants(66, 0xFE50..=0xFE6F),
        ArabicPresentationFormsB(67, 0xFE70..=0xFEFF),
        HalfwidthAndFullwidthForms(68, 0xFF00..=0xFFEF),
        Specials(69, 0xFFF0..=0xFFFF),
        Tibetan(70, 0x0F00..=0x0FFF),
        Syriac(71, 0x0700..=0x074F),
        Thaana(72, 0x0780..=0x07BF),
        Sinhala(73, 0x0D80..=0x0DFF),
        Myanmar(74, 0x1000..=0x109F),
        Ethiopic(75, 0x1200..=0x137F),
        EthiopicSupplement(75, 0x1380..=0x139F),
        EthiopicExtended(75, 0x2D80..=0x2DDF),
        Cherokee(76, 0x13A0..=0x13FF),
        UnifiedCanadianAboriginalSyllabics(77, 0x1400..=0x167F),
        Ogham(78, 0x1680..=0x169F),
        Runic(79, 0x16A0..=0x16FF),
        Khmer(80, 0x1780..=0x17FF),
        KhmerSymbols(80, 0x19E0..=0x19FF),
        Mongolian(81, 0x1800..=0x18AF),
        BraillePatterns(82, 0x2800..=0x28FF),
        YiSyllables(83, 0xA000..=0xA48F),
        YiRadicals(83, 0xA490..=0xA4CF),
        Tagalog(84, 0x1700..=0x171F),
        Hanunoo(84, 0x1720..=0x173F),
        Buhid(84, 0x1740..=0x175F),
        Tagbanwa(84, 0x1760..=0x177F),
        OldItalic(85, 0x10300..=0x1032F),
        Gothic(86, 0x10330..=0x1034F),
        Deseret(87, 0x10400..=0x1044F),
        ByzantineMusicalSymbols(88, 0x1D000..=0x1D0FF),
        MusicalSymbols(88, 0x1D100..=0x1D1FF),
        AncientGreekMusicalNotation(88, 0x1D200..=0x1D24F),
        MathematicalAlphanumericSymbols(89, 0x1D400..=0x1D7FF),
        PrivateUsePlane15(90, 0xF0000..=0xFFFFD),
        PrivateUsePlane16(90, 0x100000..=0x10FFFD),
        VariationSelectors(91, 0xFE00..=0xFE0F),
        VariationSelectorsSupplement(91, 0xE0100..=0xE01EF),
        Tags(92, 0xE0000..=0xE007F),
        Limbu(93, 0x1900..=0x194F),
        TaiLe(94, 0x1950..=0x197F),
        NewTaiLue(95, 0x1980..=0x19DF),
        Buginese(96, 0x1A00..=0x1A1F),
        Glagolitic(97, 0x2C00..=0x2C5F),
        Tifinagh(98, 0x2D30..=0x2D7F),
        YijingHexagramSymbols(99, 0x4DC0..=0x4DFF),
        SylotiNagri(100, 0xA800..=0xA82F),
        LinearBSyllabary(101, 0x10000..=0x1007F),
        LinearBIdeograms(101, 0x10080..=0x100FF),
        AegeanNumbers(101, 0x10100..=0x1013F),
        AncientGreekNumbers(102, 0x10140..=0x1018F),
        Ugaritic(103, 0x10380..=0x1039F),
        OldPersian(104, 0x103A0..=0x103DF),
        Shavian(105, 0x10450..=0x1047F),
        Osmanya(106, 0x10480..=0x104AF),
        CypriotSyllabary(107, 0x10800..=0x1083F),
        Kharoshthi(108, 0x10A00..=0x10A5F),
        TaiXuanJingSymbols(109, 0x1D300..=0x1D35F),
        Cuneiform(110, 0x12000..=0x123FF),
        CuneiformNumbersAndPunctuation(110, 0x12400..=0x1247F),
        CountingRodNumerals(111, 0x1D360..=0x1D37F),
        Sundanese(112, 0x1B80..=0x1BBF),
        Lepcha(113, 0x1C00..=0x1C4F),
        OlChiki(114, 0x1C50..=0x1C7F),
        Saurashtra(115, 0xA880..=0xA8DF),
        KayahLi(116, 0xA900..=0xA92F),
        Rejang(117, 0xA930..=0xA95F),
        Cham(118, 0xAA00..=0xAA5F),
        AncientSymbols(119, 0x10190..=0x101CF),
        PhaistosDisc(120, 0x101D0..=0x101FF),
        Carian(121, 0x102A0..=0x102DF),
        Lycian(121, 0x10280..=0x1029F),
        Lydian(121, 0x10920..=0x1093F),
        DominoTiles(122, 0x1F030..=0x1F09F),
        MahjongTiles(122, 0x1F000..=0x1F02F),
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CodePageRange(u64);
bitflags! {
    impl CodePageRange: u64 {
        const Latin1 = 1 << 0;
        const Latin2 = 1 << 1;
        const Cyrillic = 1 << 2;
        const Greek = 1 << 3;
        const Turkish = 1 << 4;
        const Hebrew = 1 << 5;
        const Arabic = 1 << 6;
        const WindowsBaltic = 1 << 7;
        const Vietnamese = 1 << 8;
        const Thai = 1 << 16;
        const JISJapan = 1 << 17;
        const ChineseSimplified = 1 << 18;
        const KoreanWansung = 1 << 19;
        const ChineseTraditional = 1 << 20;
        const KoreanJohab = 1 << 21;
        const MacintoshCharacterSet = 1 << 29;
        const OEMCharacterSet = 1 << 30;
        const SymbolCharacterSet = 1 << 31;
        const IBMGreek = 1 << 48;
        const MSDOSRussian = 1 << 49;
        const MSDOSNordic = 1 << 50;
        const Arabic864 = 1 << 51;
        const MSDOSCanadianFrench = 1 << 52;
        const Hebrew862 = 1 << 53;
        const MSDOSIcelandic = 1 << 54;
        const MSDOSPortuguese = 1 << 55;
        const IBMTurkish = 1 << 56;
        const IBMCyrillic = 1 << 57;
        const Latin2_852 = 1 << 58;
        const MSDOSBaltic = 1 << 59;
        const GreekFormer437G = 1 << 60;
        const ArabicASMO708 = 1 << 61;
        const WELatin1 = 1 << 62;
        const US = 1 << 63;
    }
}

pub struct Table {
    // pub version: u16, = 4
    pub x_avg_char_width: fword,
    pub us_weight_class: u16,
    pub us_width_class: u16,

    // The following two fields should be bitwise ORed together forming a single 16-bit value
    pub usage_permission: UsagePermissionKind,
    pub fs_type: FsTypeUpper,

    pub y_subscript_x_size: fword,
    pub y_subscript_y_size: fword,
    pub y_subscript_x_offset: fword,
    pub y_subscript_y_offset: fword,
    pub y_superscript_x_size: fword,
    pub y_superscript_y_size: fword,
    pub y_superscript_x_offset: fword,
    pub y_superscript_y_offset: fword,
    pub y_strikeout_size: fword,
    pub y_strikeout_position: fword,

    // https://learn.microsoft.com/zh-cn/typography/opentype/spec/ibmfc
    pub s_family_class: SFamilyClass,
    pub panose_classification: PanroseClassification,

    // https://learn.microsoft.com/zh-cn/typography/opentype/spec/os2#ur
    pub unicode_range: UnicodeRange,

    pub ach_vend_id: [u8; 4], // set to 4 spaces
    pub fs_selection: FsSelectionKind,

    pub first_char_index: u16, // =0x0020
    pub last_char_index: u16,  // =0xffff

    pub s_typo_ascender: fword,
    pub s_typo_descender: fword,
    pub s_typo_line_gap: fword,
    pub us_win_ascent: ufword,
    pub us_win_descent: ufword,

    pub code_page_range: CodePageRange,

    pub sx_height: fword,
    pub s_cap_height: fword,

    pub us_default_char: u16, // = 0
    pub us_break_char: u16,   // = 0x0020
    pub us_max_context: u16,  // max kerning/ligature range
}

impl ITable for Table {
    fn name(&self) -> &'static [u8; 4] {
        b"OS/2"
    }

    fn write(&self, writer: &mut impl BufMut) {
        // writer.put_u16(self.version);
        let version = 4;
        writer.put_u16(version);
        writer.put_i16(self.x_avg_char_width);
        writer.put_u16(self.us_weight_class);
        writer.put_u16(self.us_width_class);
        writer.put_u16(self.usage_permission as u16 | self.fs_type.bits());
        writer.put_i16(self.y_subscript_x_size);
        writer.put_i16(self.y_subscript_y_size);
        writer.put_i16(self.y_subscript_x_offset);
        writer.put_i16(self.y_subscript_y_offset);
        writer.put_i16(self.y_superscript_x_size);
        writer.put_i16(self.y_superscript_y_size);
        writer.put_i16(self.y_superscript_x_offset);
        writer.put_i16(self.y_superscript_y_offset);
        writer.put_i16(self.y_strikeout_size);
        writer.put_i16(self.y_strikeout_position);
        writer.put_u16(self.s_family_class.to_int());
        writer.put_u8(self.panose_classification.family_type);
        writer.put_u8(self.panose_classification.serif_style);
        writer.put_u8(self.panose_classification.weight);
        writer.put_u8(self.panose_classification.proportion);
        writer.put_u8(self.panose_classification.contrast);
        writer.put_u8(self.panose_classification.stroke_variation);
        writer.put_u8(self.panose_classification.arm_style);
        writer.put_u8(self.panose_classification.letterform);
        writer.put_u8(self.panose_classification.midline);
        writer.put_u8(self.panose_classification.x_height);
        let unicode_range_1 = self.unicode_range.0 as u32;
        let unicode_range_2 = (self.unicode_range.0 >> 32) as u32;
        let unicode_range_3 = (self.unicode_range.0 >> 64) as u32;
        let unicode_range_4 = (self.unicode_range.0 >> 96) as u32;
        writer.put_u32(unicode_range_1);
        writer.put_u32(unicode_range_2);
        writer.put_u32(unicode_range_3);
        writer.put_u32(unicode_range_4);
        writer.put_u32(u32::from_be_bytes(self.ach_vend_id));
        writer.put_u16(self.fs_selection.bits());
        writer.put_u16(self.first_char_index);
        writer.put_u16(self.last_char_index);
        writer.put_i16(self.s_typo_ascender);
        writer.put_i16(self.s_typo_descender);
        writer.put_i16(self.s_typo_line_gap);
        writer.put_u16(self.us_win_ascent);
        writer.put_u16(self.us_win_descent);
        let code_page_range_1 = self.code_page_range.0 as u32;
        let code_page_range_2 = (self.code_page_range.0 >> 32) as u32;
        writer.put_u32(code_page_range_1);
        writer.put_u32(code_page_range_2);
        writer.put_i16(self.sx_height);
        writer.put_i16(self.s_cap_height);
        writer.put_u16(self.us_default_char);
        writer.put_u16(self.us_break_char);
        writer.put_u16(self.us_max_context);
    }
}
