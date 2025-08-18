use monoxide_script::{
    ast::Glyph,
    dsl::{BezierBuilder, IntoOutlineExt},
    line,
};

use super::InputContext;
use crate::font::shape::{Rect, Ring};

pub fn c(fcx: &InputContext) -> Glyph {
    Glyph::builder()
        .outline(BezierBuilder::closed((0.6, 0.)).insts([
            line!(0.8, 0.),
            line!(1., fcx.settings().width),
            line!(0.6, 0.),
        ]))
        .outline(Rect::new((0., 0.), (0.1, 0.4), 0.05))
        .outline(Ring::at((0.4, 0.2), (0.15, 0.1)).stroked(0.1))
        .build()
}
