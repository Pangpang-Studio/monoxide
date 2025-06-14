use std::sync::Arc;

use monoxide_script::{
    ast::{FontContext, SimpleGlyph},
    dsl::{BezierBuilder, BezierInst},
};

pub fn c(fcx: &FontContext) -> SimpleGlyph {
    SimpleGlyph::new(
        [BezierBuilder::new(true, (0.3, 0.))
            .extend([
                BezierInst::line(0.6, 0.),
                BezierInst::line(1., fcx.settings.width),
                BezierInst::line(0.3, 0.),
            ])
            .build()]
        .map(Arc::new),
    )
}
