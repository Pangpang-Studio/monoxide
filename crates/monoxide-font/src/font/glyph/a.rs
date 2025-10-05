use monoxide_script::prelude::*;

use crate::font::{
    InputContext,
    dir::{Alignment, Dir},
    math::mix,
};

pub fn a_cap(cx: &InputContext) -> Glyph {
    let_settings! {
        {
            lower_left,
            lower_right,
            upper_left,
            upper_right,
            upper_mid,
            lower_mid,
            stw,
        } = cx.settings();
    }

    let curl_height = 0.65;
    let curl_height1 = 0.6;
    let bar_height = 0.65;

    let lm1 = mix(lower_left, upper_left, curl_height);
    let lm1o = mix(lower_left, upper_left, curl_height1);
    let lm2 = mix(lm1o, upper_mid, 0.5);
    let left = SpiroBuilder::open()
        .insts([
            corner!(lower_left),
            curl!(lm1).align(Alignment::Left),
            flat!(lm2).align(Alignment::Middle),
            corner!(upper_mid).heading(Dir::U).align(Alignment::Middle),
        ])
        .stroked(stw);

    let right = left
        .clone()
        .transformed(Affine2D::mirrored_along(lower_mid, Point2D::unit_y()));

    let bar = SpiroBuilder::open()
        .insts([
            corner!(mix(lower_left, upper_left, bar_height)),
            corner!(mix(lower_right, upper_right, bar_height)),
        ])
        .stroked(stw);

    Glyph::builder().outlines([left, bar, right]).build()
}
