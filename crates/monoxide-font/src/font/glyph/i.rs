use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::{glyph::l::LShape, math::mix, shape::Ring};

pub fn i(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, sbr, stw, xh } = cx.settings(); }

    let hstw = stw / 2.;

    Glyph::builder()
        .outlines(LShape::new(sbl..sbr, hstw..(xh - hstw)).stroked(stw))
        .outline(i_dot(cx))
        .build()
}

pub fn i_dot(cx: &InputContext) -> impl IntoOutline {
    let_settings! { { cap, dtr, mid, sbl } = cx.settings(); }
    Ring::at((mix(mid, sbl, 0.95), cap), (dtr, dtr))
}
