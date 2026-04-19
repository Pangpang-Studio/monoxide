//! This module contains the actual font definition.

mod dir;
mod glyph;
mod math;
mod settings;
mod shape;

use itertools::Itertools;
use monoxide_script::ast::FontContext;
use settings::FontParamSettings;

pub struct InputContext {
    pub settings: FontParamSettings,
}

impl InputContext {
    fn settings(&self) -> &FontParamSettings {
        &self.settings
    }
}

pub fn make_font() -> Result<FontContext, ()> {
    let cx = InputContext {
        settings: make_font_params(),
    };

    let tofu = glyph::tofu(&cx);
    let glyphs = GLYPH_FNS.iter().map(|(_, gl)| gl(&cx)).collect_vec();

    // This is the state
    let mut fcx = FontContext::new(cx.settings);
    fcx.set_tofu(tofu);
    for (&(ch, _), gl) in glyph::GLYPH_FNS.iter().zip(glyphs) {
        fcx.set_mapping(ch, gl);
    }
    Ok(fcx)
}

pub fn make_font_params() -> FontParamSettings {
    let width = 0.5;
    let cap_height = 0.7;
    let x_height = 0.75 * cap_height;

    FontParamSettings {
        width,
        cap_height,
        x_height,
        descender: -0.2,
        stroke_width: 0.144 * width,
        side_bearing: 0.125 * width,
        overshoot: 1. / 40.,
        dot_size: 0.25 * width,
    }
}
