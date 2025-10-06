use monoxide_script::prelude::*;

use super::{InputContext, d::DShape};

pub fn p(cx: &InputContext) -> Glyph {
    let_settings! { { xh, mid, mih, dsc } = cx.settings; }
    Glyph::builder()
        .outlines(
            DShape::with_height(&cx.settings, xh - dsc).transformed(
                Affine2D::mirrored_along((mid, 0.).into(), Point2D::unit_y())
                    .mirror_along((0., mih).into(), Point2D::unit_x()),
            ),
        )
        .build()
}
