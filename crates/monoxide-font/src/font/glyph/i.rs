use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::{glyph::l::LShape, math::mix, shape::Ring};

pub fn i(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, sbr, stw, xh } = cx.settings(); }

    Glyph::builder()
        .outlines(LShape::new(sbl..sbr, 0.0..xh).stroked(stw))
        .outline(dot(cx))
        .build()
}

pub fn dot(cx: &InputContext) -> impl IntoOutline {
    let_settings! { { cap, dtr, mid, sbl } = cx.settings(); }
    Ring::at((mix(mid, sbl, 0.97), 0.97 * cap), (dtr, dtr))
}
