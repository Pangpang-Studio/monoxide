use monoxide_script::prelude::*;

use crate::{InputContext, dir::Alignment, glyph::n::NShape, prelude::*, shape::Rect};

pub fn h(cx: &InputContext) -> Glyph {
    let FontParamSettingsView { cap, .. } = cx.settings().view();

    Glyph::builder()
        .or_outlines(NShape::from_settings(cx.settings()).with_pipe_height(cap))
        .build()
}

pub fn h_cap(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        sbl, sbr, stw, cap, ..
    } = cx.settings().view();

    let left = Rect::new((sbl, 0.), (sbl, cap)).aligned(Alignment::Left);
    let right = Rect::new((sbr, 0.), (sbr, cap)).aligned(Alignment::Right);
    let bar = Rect::new((sbl, cap / 2.), (sbr, cap / 2.));

    Glyph::builder()
        .or_outlines([left, bar, right].map(|rect| rect.stroked(stw)))
        .build()
}
