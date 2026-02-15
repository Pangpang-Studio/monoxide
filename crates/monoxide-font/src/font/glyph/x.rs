use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::dir::Dir;

pub fn x(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, sbr, stw, xh } = cx.settings(); }

    let aln = 0.2;
    let slash = SpiroBuilder::open().insts([
        g4!(sbl, 0.).heading(Dir::D).aligned(aln),
        g4!(sbr, xh).heading(Dir::U).aligned(1. - aln),
    ]);

    let backslash = SpiroBuilder::open().insts([
        g4!(sbr, 0.).heading(Dir::D).aligned(1. - aln),
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
