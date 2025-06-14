use std::sync::Arc;

use monoxide_script::{
    FontParamSettings,
    ast::{FontContext, SimpleGlyph},
    dsl::{BezierBuilder, BezierInst},
};

pub fn make_font() -> Result<FontContext, ()> {
    let width = 0.5;
    let x_height = 0.5;

    let settings = FontParamSettings {
        width,
        x_height,
        descender: -0.2,
        cap_height: 0.7,
        side_bearing: 0.15 * width,
        overshoot: x_height / 50.,
    };

    let mut fcx = FontContext::new(settings);

    let glyphs = [(
        'c',
        fcx.add_glyph(
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
            .into(),
        ),
    )];

    for (ch, gl) in glyphs {
        fcx.assign_char(ch, gl);
    }
    Ok(fcx)
}
