use monoxide_script::{
    ast::{FontContext, SimpleGlyph},
    dsl::BezierBuilder,
    line,
};

use crate::font::{
    math::crange,
    shape::{rect, ring},
};

pub fn c(fcx: &FontContext) -> SimpleGlyph {
    SimpleGlyph::new([
        BezierBuilder::new(true, (0.6, 0.))
            .extend([
                line!(0.8, 0.),
                line!(1., fcx.settings.width),
                line!(0.6, 0.),
            ])
            .build(),
        rect((0., 0.), (0.1, 0.4), 0.05),
        ring(crange(0.4, 0.15), crange(0.2, 0.1)).stroked(0.1),
    ])
}
