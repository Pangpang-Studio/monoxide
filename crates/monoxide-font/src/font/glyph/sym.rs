mod tofu;

use monoxide_script::prelude::*;

pub use self::tofu::tofu;
use crate::InputContext;

pub fn space(_cx: &InputContext) -> Glyph {
    Glyph::builder().build()
}
