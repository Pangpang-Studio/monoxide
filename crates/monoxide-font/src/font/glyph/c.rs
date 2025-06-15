use monoxide_script::{
    ast::{FontContext, SimpleGlyph},
    dsl::BezierBuilder,
    line,
};

pub fn c(fcx: &FontContext) -> SimpleGlyph {
    SimpleGlyph::new([BezierBuilder::new(true, (0.3, 0.))
        .extend([
            line!(0.6, 0.),
            line!(1., fcx.settings.width),
            line!(0.3, 0.),
        ])
        .build()])
}
