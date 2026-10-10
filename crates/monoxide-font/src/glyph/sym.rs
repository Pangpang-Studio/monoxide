mod dash;
mod dot;
mod slash;
mod tick;
mod tofu;

use monoxide_script::prelude::*;

pub use self::{
    dash::{dash, underscore},
    dot::dot,
    slash::{backslash, slash},
    tick::apostrophe,
    tofu::tofu,
};
use crate::InputContext;

pub fn space(_cx: &InputContext) -> Glyph {
    Glyph::builder().build()
}
