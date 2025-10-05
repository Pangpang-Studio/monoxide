use monoxide_script::prelude::*;

use crate::font::{
    InputContext,
    dir::{Alignment, Dir},
    math::mix,
    shape::Rect,
};

pub fn a_cap(cx: &InputContext) -> Glyph {
    let_settings! {
        {
            lower_left,
            lower_right,
            upper_mid,
            lower_mid,
            stw,
        } = cx.settings();
    }

    let bar_height = 0.65;

    let left = SpiroBuilder::open()
        .insts([
            g4!(lower_left).heading(Dir::D),
            g4!(upper_mid).heading(Dir::U).aligned(Alignment::Middle),
        ])
        .stroked(stw);

    let right = left
        .clone()
        .transformed(Affine2D::mirrored_along(lower_mid, Point2D::unit_y()));

    let bar = Rect::new(
        mix(lower_left, upper_mid, bar_height),
        mix(lower_right, upper_mid, bar_height),
        stw,
    )
    .into_outline();

    Glyph::builder().outlines([left, bar, right]).build()
}
