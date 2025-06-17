//! This module contains the actual font definition.
//!
//! NOTE: This module must be named `mod.rs`, otherwise the "playground" example
//! crate will not be able to find it.

mod glyph;
mod math;
mod shape;

use monoxide_script::{FontParamSettings, ast::FontContext};

pub fn make_font() -> Result<FontContext, ()> {
    let width = 0.5;
    let x_height = 0.5;

    let settings = FontParamSettings {
        width,
        x_height,
        descender: -0.2,
        cap_height: 0.7,
        stroke_width: 0.144 * width,
        side_bearing: 0.15 * width,
        overshoot: x_height / 50.,
        dot_size: 0.25 * width,
    };

    let mut fcx = FontContext::new(settings);

    let glyphs = [
        ('c', fcx.add_glyph(glyph::c(&fcx).into())),
        ('i', fcx.add_glyph(glyph::i(&fcx).into())),
    ];

    for (ch, gl) in glyphs {
        fcx.assign_char(ch, gl);
    }
    Ok(fcx)
}
