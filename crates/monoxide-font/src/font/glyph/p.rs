use monoxide_script::prelude::*;

use super::{InputContext, d::DShape};

pub fn p(cx: &InputContext) -> Glyph {
    let_settings! { { xh, mid, mih, dsc } = cx.settings; }
    Glyph::builder()
        .outlines(
            DShape::from_settings(&cx.settings)
                .with_height(xh - dsc)
                .transformed(
                    Affine2D::mirrored_along((mid, 0.), (0., 1.)).mirror_along((0., mih), (1., 0.)),
                ),
        )
        .build()
}
