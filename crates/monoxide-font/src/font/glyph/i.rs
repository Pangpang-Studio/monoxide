use monoxide_script::{
    ast::Glyph,
    dsl::{IntoOutline, IntoOutlinesExt},
    let_settings,
};

use super::InputContext;
use crate::font::{glyph::l::LShape, math::mix, shape::Ring};

pub fn i(fcx: &InputContext) -> Glyph {
    let_settings! { { sbl, sbr, stw, xh } = fcx.settings(); }

    let hstw = stw / 2.;

    Glyph::builder()
        .outlines(LShape::new(sbl..sbr, hstw..(xh - hstw)).stroked(stw))
        .outline(i_dot(fcx))
        .build()
}

pub fn i_dot(fcx: &InputContext) -> impl IntoOutline {
    let_settings! { { cap, dtr, mid, sbl } = fcx.settings(); }
    Ring::at((mix(mid, sbl, 0.95), cap), (dtr, dtr))
}
