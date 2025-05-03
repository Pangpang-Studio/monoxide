use std::{collections::HashMap, time::SystemTime};

use monoxide_curves::{
    debug::CurveDebugger,
    point::Point2D,
    stroke::{StrokeResult, TangentOverride},
    CubicBezier,
};
use monoxide_ttf::{
    hl,
    model::{
        cmap, fword, glyf, head, hhea, hmtx, name, os2, post, ufword, FontFile, Outline,
        TrueTypeTables,
    },
};

use crate::{
    ast::{FontContext, OutlineExpr},
    trace::EvaluationTracer,
    FontParamSettings,
};

pub struct AuxiliarySettings {
    /// Points per em when converting floating-point point data into
    /// fixed-point ones. A common value is 2048.
    pub point_per_em: u16,

    pub font_name: String,

    pub settings: FontParamSettings,
}

pub fn eval(cx: &FontContext, aux: &AuxiliarySettings) -> monoxide_ttf::model::FontFile {
    // TODO: glyphs[0] must be tofu, we currently don't have anything to handle
    let mut glyphs = vec![];
    for glyph in cx.glyphs.iter() {
        match glyph {
            crate::ast::GlyphEntry::Simple(simple_glyph) => {
                let mut outlines = vec![];
                for it in &simple_glyph.outlines {
                    eval_outline(it, &mut outlines, &mut ());
                }
                let quads = outlines
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
            crate::ast::GlyphEntry::Compound(..) => todo!("Compound is not supported"),
        }
    }
    let glyf = glyf::Table { glyphs };
    let loca = hl::loca::glyf_to_loca(&glyf);
    let maxp = hl::maxp::glyf_to_maxp(&glyf);

    let mappings = cx
        .cmap
        .iter()
        .map(|(&k, &v)| hl::cmap::SeqMapping {
            start_code: k as u32,
            len: 1,
            glyph_id: v.0 as u32,
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
            advance_width: (aux.point_per_em as f64 * aux.settings.width) as ufword,
            left_side_bearing: 0,
        }],
        left_side_bearings: vec![0; cx.glyphs.len() - 1],
    };

    let head = head::Table {
        font_revision: 0,
        checksum_adjustment: 0,
        flags: head::HeaderFlags::empty(),
        units_per_em: 1024,
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

pub fn eval_outline<E: EvaluationTracer>(
    expr: &OutlineExpr,
    out: &mut Vec<CubicBezier<Point2D>>,
    dbg: &mut E,
) -> Result<(), EvalError<E::Id>> {
    let evaled = eval_outline_internal(expr, dbg)?;
    let id = match evaled.kind {
        EvalValueKind::Beziers(beziers) => {
            out.extend(beziers);
            evaled.id
        }
        EvalValueKind::Spiros(spiros) => {
            for spiro in spiros {
                let bez = monoxide_curves::convert::spiro_to_cube(&spiro.spiro);

                out.extend(bez);
            }
            let id = dbg.spiro_to_bezier(evaled.id);
            dbg.intermediate_output(id, &out);
            id
        }
    };

    dbg.set_output_id(id);
    Ok(())
}

/// A spiro curve used in evaluation context.
///
/// TODO: This should be the default type for spiro curves. Migrate later.
#[derive(Debug, Clone)]
pub struct EvalSpiro {
    pub spiro: Vec<monoxide_spiro::SpiroCp>,
    pub tangent_override: TangentOverride,
}

/// Represents an intermediate value during the evaluation of a glyph.
#[derive(Debug, Clone)]
pub enum EvalValueKind {
    Beziers(Vec<CubicBezier<Point2D>>),
    Spiros(Vec<EvalSpiro>),
}

#[derive(Debug, Clone)]
pub struct EvalValue<Id> {
    pub kind: EvalValueKind,
    pub id: Id,
}

impl<Id> EvalValue<Id> {
    pub fn bezier(bezier: CubicBezier<Point2D>, id: Id) -> Self {
        Self {
            kind: EvalValueKind::Beziers(vec![bezier]),
            id,
        }
    }

    pub fn spiro(spiro: EvalSpiro, id: Id) -> Self {
        Self {
            kind: EvalValueKind::Spiros(vec![spiro]),
            id,
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum EvalError<Id> {
    #[error("Stroking a bezier is not supported yet; at {0}")]
    StrokingABezier(Id),
}

fn eval_outline_internal<E: EvaluationTracer>(
    expr: &OutlineExpr,
    dbg: &mut E,
) -> Result<EvalValue<E::Id>, EvalError<E::Id>> {
    match expr {
        OutlineExpr::Bezier(cubic_bezier) => {
            let id = dbg.constructed_bezier(cubic_bezier);
            return Ok(EvalValue::bezier(cubic_bezier.clone(), id));
        }
        OutlineExpr::Spiro(spiro_cps, tangent_overrides) => {
            let id = dbg.constructed_spiro(spiro_cps);
            let spiro = EvalSpiro {
                spiro: spiro_cps.clone(),
                tangent_override: tangent_overrides.clone(),
            };
            return Ok(EvalValue::spiro(spiro, id));
        }
        OutlineExpr::Stroked(outline_expr, width) => {
            let evaled = eval_outline_internal(outline_expr, dbg)?;
            match evaled.kind {
                EvalValueKind::Beziers(_) => return Err(EvalError::StrokingABezier(evaled.id)),
                EvalValueKind::Spiros(eval_spiros) => {
                    eval_stroked(evaled.id, &eval_spiros, *width, dbg)
                }
            }
        }
    }
}

fn eval_stroked<E: EvaluationTracer>(
    evaled_id: E::Id,
    eval_spiros: &[EvalSpiro],
    width: f64,
    dbg: &mut E,
) -> Result<EvalValue<E::Id>, EvalError<E::Id>> {
    // For each of the spiro curves, stroke them. This may result in
    // more than one bezier per spiro.
    let id = dbg.stroked(evaled_id, width);

    let mut out_beziers = vec![];
    for spiro in eval_spiros {
        let oc = monoxide_curves::stroke::stroke_spiro(
            &spiro.spiro,
            width,
            &spiro.tangent_override,
            &mut dbg.curve_debugger(id),
        );
        match oc {
            StrokeResult::One(spiro_cps) => {
                let bez = monoxide_curves::convert::spiro_to_cube(&spiro_cps);
                out_beziers.extend_from_slice(&bez);
            }
            StrokeResult::Two(spiro_cps, spiro_cps1) => {
                let bez1 = monoxide_curves::convert::spiro_to_cube(&spiro_cps);
                let bez2 = monoxide_curves::convert::spiro_to_cube(&spiro_cps1);
                out_beziers.extend_from_slice(&bez1);
                out_beziers.extend_from_slice(&bez2);
            }
        }
    }

    dbg.intermediate_output(id, &out_beziers);

    let id1 = dbg.spiro_to_bezier(id);

    return Ok(EvalValue {
        kind: EvalValueKind::Beziers(out_beziers),
        id: id1,
    });
}
