mod bar;
mod dot;
mod slash;
mod tick;
mod tofu;

use monoxide_script::prelude::*;

pub use self::{
    bar::{dash, equal, pipe, plus, underscore},
    dot::{comma, dot, exclamation},
    slash::{backslash, slash},
    tick::{apostrophe, asterisk, quotes},
    tofu::tofu,
};
use crate::InputContext;

pub fn space(_cx: &InputContext) -> Glyph {
    Glyph::builder().build()
}
