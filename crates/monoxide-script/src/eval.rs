use std::{collections::BTreeMap, sync::Arc, time::SystemTime};

use monoxide_ttf::{
    hl,
    model::{
        FontFile, Outline, TrueTypeTables, cmap, fword, glyf, head, hhea, hmtx, name, os2, post,
        ufword,
    },
};

use crate::ast::{FontContext, OutlineExpr};

mod layout;
mod outline;
pub use layout::layout_glyphs;
pub use outline::*; // fixme: use selective imports

pub struct AuxiliarySettings {
    /// Points per em when converting floating-point point data into
    /// fixed-point ones. A common value is 2048.
    pub point_per_em: u16,
    pub font_name: String,
}

pub enum SerializedGlyphKind {
    /// A simple glyph represented as a number of outlines
    Simple(Vec<Arc<OutlineExpr>>),
    /// A compound glyph composed of multiple components, represented here as
    /// component indices.
    ///
    /// TODO: transformations of glyphs
    Compound(Vec<usize>),
}

pub struct SerializedGlyph {
    pub kind: SerializedGlyphKind,
    pub advance: Option<f64>,
}

#[derive(Debug, thiserror::Error)]
pub enum HighEvalError {
    #[error("Tofu glyph is unset")]
    TofuUnset,
}

/// A [`FontContext`] with only reachable glyphs and the layout determined.
///
/// The implementor should ensure that `glyph_list[0]` is the TOFU glyph.
pub struct SerializedFontContext {
    pub glyph_list: Vec<SerializedGlyph>,
    pub cmap: BTreeMap<char, usize>,
}

pub fn eval(cx: &FontContext, aux: &AuxiliarySettings) -> Result<FontFile, HighEvalError> {
    let scx = layout_glyphs(cx)?;
    if scx.glyph_list.len() == 1 {
        panic!("Windows font reader disallow single-glyph fonts")
    }
    let glyphs = eval_glyphs(aux, &scx);
    let res = create_tables(cx, &scx, aux, glyphs);
    Ok(res)
}

fn eval_glyphs(aux: &AuxiliarySettings, scx: &SerializedFontContext) -> Vec<glyf::Glyph> {
    let mut glyphs = vec![];
    for glyph in scx.glyph_list.iter() {
        match &glyph.kind {
            SerializedGlyphKind::Simple(outlines) => {
                let mut res_outlines = vec![];
                for it in outlines {
                    eval_outline(it, &mut res_outlines, &mut ()).expect("Eval error!");
                    // FIXME: handle errors
                }
                let quads = res_outlines
                    .into_iter()
                    .map(|x| monoxide_curves::convert::cube_to_quad(x, 0.00001))
                    .map(|x| x.cast(|x| x * (aux.point_per_em as f64)))
                    .map(|x| {
                        x.cast(|v| {
                            let x_fword = v.x as fword;
                            let y_fword = v.y as fword;
                            (x_fword, y_fword)
                        })
                    })
                    .collect::<Vec<_>>();
                let simple_glyph = hl::glyf::encode(&quads).unwrap();
                glyphs.push(glyf::Glyph::Simple(simple_glyph));
            }
            SerializedGlyphKind::Compound(..) => todo!("Compound is not supported"),
        }
    }
    glyphs
}

fn create_tables(
    cx: &FontContext,
    scx: &SerializedFontContext,
    aux: &AuxiliarySettings,
    glyphs: Vec<glyf::Glyph>,
) -> FontFile {
    let glyf = glyf::Table { glyphs };
    let loca = hl::loca::glyf_to_loca(&glyf);
    let maxp = hl::maxp::glyf_to_maxp(&glyf);

    let mappings = scx
        .cmap
        .iter()
        .map(|(&k, &v)| hl::cmap::SeqMapping {
            start_code: k as u32,
            len: 1,
            glyph_id: v as u32,
        })
        .collect::<Vec<_>>();
    let cmap = hl::cmap::Table {
        subtables: vec![mappings],
        mapping: vec![(hl::cmap::Encoding::Unicode, 0)],
    };
    let cmap = cmap::Table::from_raw(cmap);

    let outline = Outline::TrueType(TrueTypeTables { glyf, loca, maxp });

    let hmtx = hmtx::Table {
        metrics: vec![hmtx::LongHorizontalMetric {
            advance_width: (aux.point_per_em as f64 * cx.settings.width) as ufword,
            left_side_bearing: 0,
        }],
        left_side_bearings: vec![0; scx.glyph_list.len() - 1],
    };

    let head = head::Table {
        font_revision: 0,
        checksum_adjustment: 0,
        flags: head::HeaderFlags::empty(),
        units_per_em: aux.point_per_em,
        created: SystemTime::now(),
        modified: SystemTime::now(),
        x_min: 0,
        y_min: 0,
        x_max: 1024,
        y_max: 1024,
        mac_style: head::MacStyle::REGULAR,
        lowest_rec_ppem: 72,
    };

    let hhea = hhea::Table {
        ascender: 1024,
        descender: -256,
        line_gap: 0,
        advance_width_max: 1024,
        min_left_side_bearing: 0,
        min_right_side_bearing: 0,
        x_max_extent: 1024,
        caret_slope_rise: 1,
        caret_slope_run: 0,
        caret_offset: 0,
        metric_data_format: 0,
        number_of_hmetrics: 2,
    };

    let name = name::Table {
        records: [(
            name::Lang::Microsoft(name::MSLangID::en_us),
            name::NameRecords {
                font_family_name: Some(aux.font_name.clone()),
                ..Default::default()
            },
        )]
        .into_iter()
        .collect(),
    };

    let os2 = os2::Table {
        x_avg_char_width: 1024,
        us_weight_class: 400,
        us_width_class: 5,
        usage_permission: os2::UsagePermissionKind::EditableEmbedding,
        fs_type: os2::FsTypeUpper::Nothing,
        y_subscript_x_size: 512,
        y_subscript_y_size: 512,
        y_subscript_x_offset: 0,
        y_subscript_y_offset: 128,
        y_superscript_x_size: 512,
        y_superscript_y_size: 512,
        y_superscript_x_offset: 0,
        y_superscript_y_offset: 768,
        y_strikeout_size: 50,
        y_strikeout_position: 258,
        s_family_class: os2::SFamilyClass::NoClassification,
        panose_classification: os2::PanroseClassification {
            family_type: 2,
            serif_style: 0,
            weight: 5,
            proportion: 3,
            contrast: 0,
            stroke_variation: 0,
            arm_style: 0,
            letterform: 0,
            midline: 0,
            x_height: 0,
        },
        unicode_range: os2::UnicodeRange::BasicLatin,
        ach_vend_id: *b"TEST",
        fs_selection: os2::FsSelectionKind::Regular,
        first_char_index: 0,
        last_char_index: 0xffff,
        s_typo_ascender: 1024,
        s_typo_descender: -256,
        s_typo_line_gap: 0,
        us_win_ascent: 1024,
        us_win_descent: 256,
        code_page_range: os2::CodePageRange::Latin1,
        sx_height: 768,
        s_cap_height: 1024,
        us_default_char: 65,
        us_break_char: 65,
        us_max_context: 0,
    };

    let post = post::TableV3 {
        italic_angle: monoxide_ttf::model::Fixed::from_num(0),
        underline_position: 0,
        underline_thickness: 0,
        is_fixed_pitch: false,
        min_mem_type42: 0,
        max_mem_type42: 0,
        min_mem_type1: 0,
        max_mem_type1: 0,
    };

    FontFile {
        head,
        hhea,
        hmtx,
        cmap,
        name,
        os2,
        post,
        outline,
        dsig: Some(Default::default()),
    }
}
