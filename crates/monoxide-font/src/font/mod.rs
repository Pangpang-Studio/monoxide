//! This module contains the actual font definition.
//!
//! NOTE: This module must be named `mod.rs`, otherwise the "playground" example
//! crate will not be able to find it.

mod dir;
mod glyph;
mod math;
mod settings;
mod shape;

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
    let width = 0.5;
    let cap_height = 0.7;
    let x_height = 0.75 * cap_height;

    let settings = FontParamSettings {
        width,
        cap_height,
        x_height,
        descender: -0.2,
        stroke_width: 0.144 * width,
        side_bearing: 0.125 * width,
        overshoot: x_height / 40.,
        dot_size: 0.25 * width,
    };

    let cx = InputContext { settings };

    let glyphs = [
        (' ', glyph::space(&cx)),
        ('A', glyph::a::a_cap(&cx)),
        ('a', glyph::a(&cx)),
        ('b', glyph::b(&cx)),
        ('c', glyph::c(&cx)),
        ('d', glyph::d(&cx)),
        ('e', glyph::e(&cx)),
        ('f', glyph::f(&cx)),
        ('h', glyph::h(&cx)),
        ('i', glyph::i(&cx)),
        ('J', glyph::j::j_cap(&cx)),
        ('j', glyph::j(&cx)),
        ('k', glyph::k(&cx)),
        ('l', glyph::l(&cx)),
        ('m', glyph::m(&cx)),
        ('n', glyph::n(&cx)),
        ('o', glyph::o(&cx)),
        ('p', glyph::p(&cx)),
        ('q', glyph::q(&cx)),
        ('r', glyph::r(&cx)),
        ('t', glyph::t(&cx)),
        ('u', glyph::u(&cx)),
        ('v', glyph::v(&cx)),
        ('x', glyph::x(&cx)),
        ('y', glyph::y(&cx)),
        ('z', glyph::z(&cx)),
    ];
    let tofu = glyph::tofu(&cx);

    // This is the state
    let mut fcx = FontContext::new(cx.settings);
    fcx.set_tofu(tofu);
    for (ch, gl) in glyphs {
        fcx.set_mapping(ch, gl);
    }
    Ok(fcx)
}
