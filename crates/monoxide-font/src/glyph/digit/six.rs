use std::f64::consts::PI;

use monoxide_script::prelude::*;

use crate::{InputContext, glyph::digit::nine::NineShape, prelude::*};

pub fn six(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        sbl,
        sbr,
        stw,
        cap,
        mid,
        ovs,
        ..
    } = cx.settings().view();

    Glyph::builder()
        .outlines(
            NineShape::new(sbl..sbr, 0.0..cap, mid - stw / 2., ovs)
                .stroked(stw)
                .transformed(Affine2D::rotated_around((mid, cap / 2.), PI)),
        )
        .build()
}
