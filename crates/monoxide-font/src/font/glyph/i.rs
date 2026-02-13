use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::{glyph::l::LShape, math::mix, settings::FontParamSettings, shape::Ring};

pub fn i(cx: &InputContext) -> Glyph {
    let settings = cx.settings();
    let_settings! { { sbl, sbr, stw, xh } = settings; }

    Glyph::builder()
        .outlines(LShape::new(sbl..sbr, 0.0..xh).stroked(stw))
        .outline(dot(settings))
        .build()
}

pub fn dot(settings: &FontParamSettings) -> Ring {
    let_settings! { { cap, dtr, mid, sbl } = settings; }
    Ring::at((mix(mid, sbl, 0.97), 0.97 * cap), (dtr, dtr))
}
