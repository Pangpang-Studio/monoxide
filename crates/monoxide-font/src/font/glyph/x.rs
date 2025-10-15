use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::dir::Dir;

pub fn x(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, sbr, stw, xh } = cx.settings(); }

    let slash = SpiroBuilder::open()
        .insts([
            g4!(sbl, 0.).heading(Dir::D).aligned(0.25),
            g4!(sbr, xh).heading(Dir::U).aligned(0.75),
        ])
        .into_outline();
    let backslash = SpiroBuilder::open()
        .insts([
            g4!(sbr, 0.).heading(Dir::D).aligned(0.75),
            g4!(sbl, xh).heading(Dir::U).aligned(0.25),
        ])
        .into_outline();

    Glyph::builder()
        // TODO: Find out how the scaling factor is determined.
        .outlines([slash, backslash].stroked(stw * 0.9))
        .build()
}
