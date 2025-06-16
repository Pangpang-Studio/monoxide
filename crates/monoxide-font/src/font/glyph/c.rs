use monoxide_script::{
    ast::{FontContext, SimpleGlyph},
    dsl::{BezierBuilder, IntoOutlineExt},
    line,
};

use crate::font::shape::{Rect, Ring};

pub fn c(fcx: &FontContext) -> SimpleGlyph {
    SimpleGlyph::new()
        .outline(BezierBuilder::open((0.6, 0.)).insts([
            line!(0.8, 0.),
            line!(1., fcx.settings.width),
            line!(0.6, 0.),
        ]))
        .outline(Rect::new((0., 0.), (0.1, 0.4), 0.05))
        .outline(Ring::at((0.4, 0.2), (0.15, 0.1)).stroked(0.1))
}
