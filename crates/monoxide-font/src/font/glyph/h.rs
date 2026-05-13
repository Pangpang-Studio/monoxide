use monoxide_script::prelude::*;

use crate::{
    InputContext,
    font::{dir::Alignment, glyph::n::NShape, shape::Rect},
};

pub fn h(cx: &InputContext) -> Glyph {
    let_settings! { { cap } = cx.settings(); }

    Glyph::builder()
        .outlines(NShape::from_settings(cx.settings()).with_pipe_height(cap))
        .build()
}

pub fn h_cap(cx: &InputContext) -> Glyph {
    let_settings! { { lower_left, lower_right, upper_left, upper_right, stw, cap } = cx.settings(); }

    let left = Rect::new(lower_left, upper_left).aligned(Alignment::Left);
    let right = Rect::new(lower_right, upper_right).aligned(Alignment::Right);
    let bar = Rect::new((lower_left.x, cap / 2.), (lower_right.x, cap / 2.));

    Glyph::builder()
        .outlines([left, right, bar].map(|rect| rect.stroked(stw).into_outline()))
        .build()
}
