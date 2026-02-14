use monoxide_script::prelude::*;

use super::{InputContext, n::NShape};

pub fn u(cx: &InputContext) -> Glyph {
    let_settings! { { mid, mih } = cx.settings(); }

    Glyph::builder()
        .outlines(NShape::from_settings(cx.settings()).transformed(
            Affine2D::mirrored_along((mid, 0.), (0., 1.)).mirror_along((0., mih), (1., 0.)),
        ))
        .build()
}
