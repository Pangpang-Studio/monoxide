use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::{
    dir::{Alignment, Dir},
    math::mix,
};

pub fn y(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, mid, sbr, stw, xh, dsc } = cx.settings(); }

    let aln = 0.2;
    let slash = SpiroBuilder::open().insts([
        g4!(mix(sbr, mid, dsc / xh), dsc).heading(Dir::D),
        g4!(mid, 0.).aligned(Alignment::Middle),
        g4!(sbr, xh).heading(Dir::U).aligned(1. - aln),
    ]);

    let backslash = SpiroBuilder::open().insts([
        g4!(mid, 0.).heading(Dir::D).aligned(Alignment::Middle),
        g4!(sbl, xh).heading(Dir::U).aligned(aln),
    ]);

    Glyph::builder()
        .outlines(
            [slash, backslash]
                .into_iter()
                .map(move |it| it.into_outline().stroked(stw)),
        )
        .build()
}
