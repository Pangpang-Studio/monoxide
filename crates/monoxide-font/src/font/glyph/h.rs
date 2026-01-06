use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::glyph::n::NShape;

pub fn h(cx: &InputContext) -> Glyph {
    let_settings! { { cap } = cx.settings(); }

    Glyph::builder()
        .outlines(NShape::from_settings(cx.settings()).with_pipe_height(cap))
        .build()
}
