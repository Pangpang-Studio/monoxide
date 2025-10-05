use monoxide_script::prelude::*;

use super::{InputContext, d::DShape};

pub fn b(cx: &InputContext) -> Glyph {
    Glyph::builder()
        .outlines(
            DShape::new(&cx.settings).transformed(Affine2D::mirrored_along(
                cx.settings.lower_mid(),
                Point2D::unit_y(),
            )),
        )
        .build()
}
