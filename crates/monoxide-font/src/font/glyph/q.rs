use monoxide_script::prelude::*;

use super::{InputContext, d::DShape};

pub fn q(cx: &InputContext) -> Glyph {
    let_settings! { { xh, mih, dsc } = cx.settings; }
    Glyph::builder()
        .outlines(
            DShape::from_settings(&cx.settings)
                .with_height(xh - dsc)
                .transformed(Affine2D::mirrored_along((0., mih), (1., 0.))),
        )
        .build()
}
