use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::{
    dir::{Alignment, Dir},
    glyph::c::CShape,
    shape::Rect,
};

pub fn z(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, sbr, stw, xh, mid, mih, ovs } = cx.settings(); }

    let serif_l = CShape::new((mid, mih), (mid - sbl, mih), ovs).aperture_curve_h_lo();
    let serif = Rect::new((sbl, xh - serif_l - stw), (sbl, xh)).aligned(Alignment::Left);

    let aln = 0.;
    let slash = SpiroBuilder::open().insts([
        g4!(sbl, stw).heading(Dir::D).aligned(aln).width(0.9),
        g4!(sbr, xh - stw).heading(Dir::U).aligned(1. - aln),
    ]);

    let top_bar = Rect::new((sbl, xh), (sbr, xh)).aligned(Alignment::Left);
    let bottom_bar = Rect::new((sbl, 0.), (sbr, 0.)).aligned(Alignment::Right);

    Glyph::builder()
        .outlines(
            [
                serif.into_outline(),
                top_bar.into_outline(),
                slash.into_outline(),
                bottom_bar.into_outline(),
            ]
            .into_iter()
            .map(move |it| it.stroked(stw)),
        )
        .build()
}
