use monoxide_script::prelude::*;

use crate::{
    InputContext,
    font::{glyph::l::LShape, math::mix, settings::FontParamSettings, shape::Ring},
};

pub fn i(cx: &InputContext) -> Glyph {
    let settings = cx.settings();
    let_settings! { { sbl, sbr, stw, xh } = settings; }

    Glyph::builder()
        .outlines(LShape::new(sbl..sbr, 0.0..xh).stroked(stw))
        .outline(dot(settings))
        .build()
}

pub fn i_cap(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, sbr, stw, cap } = cx.settings(); }

    Glyph::builder()
        .outlines(
            LShape::new(sbl..sbr, 0.0..cap)
                .with_top_bar_scale((-1.)..1.)
                .stroked(stw),
        )
        .build()
}

pub fn dot(settings: &FontParamSettings) -> Ring {
    let_settings! { { cap, dtr, mid, sbl } = settings; }
    Ring::at((mix(mid, sbl, 0.97), 0.97 * cap), (dtr, dtr))
}
