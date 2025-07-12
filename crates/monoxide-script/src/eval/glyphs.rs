//! Evaluate glyphs

use std::sync::Arc;

use monoxide_curves::{point::Point2D, xform::Affine2D};
use monoxide_ttf::{
    hl,
    model::{
        f2dot14, fword,
        glyf::{self, GlyphCommon, compound::Scale},
    },
};
use petgraph::visit::DfsPostOrder;

use crate::{
    ast::OutlineExpr,
    eval::{
        AuxiliarySettings, SerializedComponent, SerializedFontContext, SerializedGlyphKind,
        eval_outline,
    },
};

pub fn eval_glyphs(aux: &AuxiliarySettings, scx: &SerializedFontContext) -> Vec<glyf::Glyph> {
    // Evaluate in post-order, so we calculate the dimensions of compound glyphs
    // after all of its components.
    let mut glyphs = Vec::new();
    glyphs.resize_with(scx.glyph_list.len(), || None);

    let mut iter = DfsPostOrder::empty(&scx.glyph_map);
    // Add all nodes to the to-be-iterated list
    iter.stack.extend((0..glyphs.len()).rev());
    // RUN!
    while let Some(ix) = iter.next(&scx.glyph_map) {
        let glyph = &scx.glyph_list[ix];
        match &glyph.kind {
            SerializedGlyphKind::Simple(outlines) => {
                let simple_glyph = eval_simple_glyph(aux, outlines);
                glyphs[ix] = Some(glyf::Glyph::Simple(simple_glyph));
            }
            SerializedGlyphKind::Compound(comps) => {
                let glyph = eval_compound_glyph(aux, &glyphs, comps);
                glyphs[ix] = Some(glyf::Glyph::Compound(glyph));
            }
        }
    }

    glyphs
        .into_iter()
        .enumerate()
        .map(|(ix, val)| val.unwrap_or_else(|| panic!("Glyph at index {ix} is not set")))
        .collect()
}

fn eval_simple_glyph(
    aux: &AuxiliarySettings,
    outlines: &[Arc<OutlineExpr>],
) -> glyf::simple::SimpleGlyph {
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

    hl::glyf::encode(&quads).unwrap()
}

fn eval_compound_glyph(
    aux: &AuxiliarySettings,
    glyphs: &[Option<glyf::Glyph>],
    comps: &[SerializedComponent],
) -> glyf::compound::CompoundGlyph {
    let bb = comps
        .iter()
        .map(|comp| {
            let bb = glyphs[comp.index]
                .as_ref()
                .expect("Children glyphs should be evaluated before compound glyph evaluate")
                .common();
            affine_bb(aux, &comp.xform, bb)
        })
        .reduce(|bb1, bb2| GlyphCommon {
            x_min: bb1.x_min.min(bb2.x_min),
            y_min: bb1.y_min.min(bb2.y_min),
            x_max: bb1.x_max.max(bb2.x_max),
            y_max: bb1.y_max.max(bb2.y_max),
        })
        .expect("Compound glyph should have at lease one component");

    let components = comps
        .iter()
        .map(|c| {
            let trans = c.xform.translation();
            let args = glyf::compound::Args::Offset {
                x: (trans.x * aux.point_per_em as f64) as fword,
                y: (trans.y * aux.point_per_em as f64) as fword,
            };
            let scale = if c.xform.scale_is_identity() {
                Scale::One
            } else if let Some(scale) = c.xform.scale_is_uniform() {
                Scale::Simple(f2dot14::from_num(scale))
            } else if let Some((x, y)) = c.xform.mat_is_only_scale() {
                Scale::XY {
                    x: f2dot14::from_num(x),
                    y: f2dot14::from_num(y),
                }
            } else {
                let mat = c.xform.matrix(); // column major
                Scale::TwoByTwo {
                    xx: f2dot14::from_num(mat[0].x),
                    yx: f2dot14::from_num(mat[1].x),
                    xy: f2dot14::from_num(mat[0].y),
                    yy: f2dot14::from_num(mat[1].y),
                }
            };
            glyf::compound::Component {
                flags: glyf::compound::ComponentFlags::empty(),
                glyph_index: c.index as u16,
                args,
                scale,
            }
        })
        .collect();

    glyf::compound::CompoundGlyph {
        common: bb,
        components,
        instructions: vec![],
    }
}

/// Affine transform a bounding box
fn affine_bb(aux: &AuxiliarySettings, aff: &Affine2D<Point2D>, bb: &GlyphCommon) -> GlyphCommon {
    // One day we will have a bounding box on data before transform...
    // But now let's hack this through by translating the points back to f64 and
    // calculate the result bounding box
    let aff = Affine2D::make(aff.translation() * aux.point_per_em as f64, aff.matrix());
    let points = [
        cvt_pt(bb.x_min, bb.y_min),
        cvt_pt(bb.x_max, bb.y_min),
        cvt_pt(bb.x_max, bb.y_max),
        cvt_pt(bb.x_min, bb.y_max),
    ];
    let xformed_points = points.map(|pt| aff.apply(&pt));

    let x_max = xformed_points.iter().map(|pt| pt.x as fword).max().unwrap();
    let x_min = xformed_points.iter().map(|pt| pt.x as fword).min().unwrap();
    let y_max = xformed_points.iter().map(|pt| pt.y as fword).max().unwrap();
    let y_min = xformed_points.iter().map(|pt| pt.y as fword).min().unwrap();

    GlyphCommon {
        x_min,
        y_min,
        x_max,
        y_max,
    }
}

fn cvt_pt(x: fword, y: fword) -> Point2D {
    Point2D::new(x as f64, y as f64)
}
