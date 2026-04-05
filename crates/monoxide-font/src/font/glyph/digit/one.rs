use monoxide_script::prelude::*;

use crate::{
    InputContext,
    font::{
        dir::{Alignment, Dir},
        shape::Rect,
    },
};

pub fn one(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, sbr, stw, cap, mid, xh } = cx.settings(); }

    let pipe = Rect::new((mid, 0.), (mid, cap))
        .stroked(stw)
        .transformed(Affine2D::translated((stw / 2., 0.)));

    let slash = SpiroBuilder::open().insts([
        g4!(sbl, xh).aligned(Alignment::Middle),
        g4!(mid, cap).heading(Dir::R).aligned(Alignment::Left),
    ]);

    Glyph::builder()
        .outline(pipe)
        .outline(slash.stroked(stw))
        .build()
}
