use super::{fword, ufword, ITable};
use bytes::BufMut;

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
    pub b_family_type: u8,
    pub b_serif_style: u8,
    pub b_weight: u8,
    pub b_proportion: u8,
    pub b_contrast: u8,
    pub b_stroke_variation: u8,
    pub b_arm_style: u8,
    pub b_letterform: u8,
    pub b_midline: u8,
    pub b_x_height: u8,
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

pub struct Table {
    pub version: u16,
    pub x_avg_char_width: fword,
    pub us_weight_class: u16,

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
    pub unicode_range1: u32,
    pub unicode_range2: u32,
    pub unicode_range3: u32,
    pub unicode_range4: u32,

    pub ach_vend_id: [u8; 4], // set to 4 spaces
    pub fs_selection: FsSelectionKind,

    pub first_char_index: u16, // =0x0020
    pub last_char_index: u16,  // =0xffff

    pub s_typo_ascender: fword,
    pub s_typo_descender: fword,
    pub s_typo_line_gap: fword,
    pub us_win_ascent: ufword,
    pub us_win_descent: ufword,

    pub ul_code_page_range1: u32,
    pub ul_code_page_range2: u32,

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
        writer.put_u16(self.version);
        writer.put_i16(self.x_avg_char_width);
        writer.put_u16(self.us_weight_class);
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
        writer.put_u8(self.panose_classification.b_family_type);
        writer.put_u8(self.panose_classification.b_serif_style);
        writer.put_u8(self.panose_classification.b_weight);
        writer.put_u8(self.panose_classification.b_proportion);
        writer.put_u8(self.panose_classification.b_contrast);
        writer.put_u8(self.panose_classification.b_stroke_variation);
        writer.put_u8(self.panose_classification.b_arm_style);
        writer.put_u8(self.panose_classification.b_letterform);
        writer.put_u8(self.panose_classification.b_midline);
        writer.put_u8(self.panose_classification.b_x_height);
        writer.put_u32(self.unicode_range1);
        writer.put_u32(self.unicode_range2);
        writer.put_u32(self.unicode_range3);
        writer.put_u32(self.unicode_range4);
        writer.put_u32(u32::from_be_bytes(self.ach_vend_id));
        writer.put_u16(self.fs_selection.bits());
        writer.put_u16(self.first_char_index);
        writer.put_u16(self.last_char_index);
        writer.put_i16(self.s_typo_ascender);
        writer.put_i16(self.s_typo_descender);
        writer.put_i16(self.s_typo_line_gap);
        writer.put_u16(self.us_win_ascent);
        writer.put_u16(self.us_win_descent);
        writer.put_u32(self.ul_code_page_range1);
        writer.put_u32(self.ul_code_page_range2);
        writer.put_i16(self.sx_height);
        writer.put_i16(self.s_cap_height);
        writer.put_u16(self.us_default_char);
        writer.put_u16(self.us_break_char);
        writer.put_u16(self.us_max_context);
    }
}
