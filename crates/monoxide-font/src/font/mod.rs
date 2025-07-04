//! This module contains the actual font definition.
//!
//! NOTE: This module must be named `mod.rs`, otherwise the "playground" example
//! crate will not be able to find it.

mod dir;
mod glyph;
mod math;
mod shape;

use monoxide_script::{FontParamSettings, ast::FontContext};

pub struct InputContext {
    pub settings: FontParamSettings,
}

impl InputContext {
    fn settings(&self) -> &FontParamSettings {
        &self.settings
    }
}

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
        overshoot: x_height / 40.,
        dot_size: 0.25 * width,
    };

    let cx = InputContext {
        settings: settings.clone(),
    };

    let glyphs = [
        ('c', glyph::c(&cx)),
        ('i', glyph::i(&cx)),
        ('n', glyph::n(&cx)),
        ('o', glyph::o(&cx)),
    ];
    let tofu = glyph::tofu(&cx);

    // This is the state
    let mut fcx = FontContext::new(settings);
    fcx.set_tofu(tofu);

    for (ch, gl) in glyphs {
        fcx.set_mapping(ch, gl);
    }
    Ok(fcx)
}
