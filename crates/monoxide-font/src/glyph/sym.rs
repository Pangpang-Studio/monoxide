mod bar;
mod bracket;
mod dot;
mod slash;
mod tick;
mod tofu;

use monoxide_script::prelude::*;

pub use self::{
    bar::{dash, equal, hash, pipe, plus, underscore},
    bracket::{
        curly_bracket_l, curly_bracket_r, round_bracket_l, round_bracket_r, square_bracket_l,
        square_bracket_r,
    },
    dot::{colon, comma, dot, exclamation, semicolon},
    slash::{backslash, slash},
    tick::{apostrophe, asterisk, quotes},
    tofu::tofu,
};
use crate::InputContext;

pub fn space(_cx: &InputContext) -> Glyph {
    Glyph::builder().build()
}
