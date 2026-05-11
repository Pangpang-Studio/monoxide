use monoxide_script::prelude::*;

use crate::{
    InputContext,
    font::{dir::Alignment, glyph::c::CShape, glyph::sym::SlashShape, shape::Rect},
};

pub fn z(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, sbr, stw, xh, mid, mih, ovs } = cx.settings(); }

    let serif_l = CShape::new((mid, mih), (mid - sbl, mih), ovs).aperture_curve_h_lo();
    let serif = Rect::new((sbl, xh - serif_l - stw), (sbl, xh)).aligned(Alignment::Left);

    let slash = SlashShape::new(sbl..sbr, stw..(xh - stw));

    let top_bar = Rect::new((sbl, xh), (sbr, xh)).aligned(Alignment::Left);
    let bottom_bar = Rect::new((sbl, 0.), (sbr, 0.)).aligned(Alignment::Right);

    Glyph::builder()
        .outlines([serif, top_bar, bottom_bar].map(move |it| it.into_outline().stroked(stw)))
        .outlines(slash.stroked(0.9 * stw))
        .build()
}
