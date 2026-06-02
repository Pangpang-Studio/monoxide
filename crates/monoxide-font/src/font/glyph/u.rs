use monoxide_script::prelude::*;

use super::{InputContext, n::NShape};
use crate::font::prelude::*;

pub fn u(cx: &InputContext) -> Glyph {
    let FontParamSettingsView { mid, mih, .. } = cx.settings().view();

    // TODO: Redesign `u` to use a different hook.
    Glyph::builder()
        .outlines(NShape::from_settings(cx.settings()).transformed(
            Affine2D::mirrored_along((mid, 0.), (0., 1.)).mirror_along((0., mih), (1., 0.)),
        ))
        .build()
}
