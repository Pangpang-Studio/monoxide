use monoxide_script::prelude::*;

use crate::{
    InputContext,
    dir::{Alignment, Dir},
    prelude::*,
    shape::Ring,
};

pub fn dot(cx: &InputContext) -> Glyph {
    let FontParamSettingsView { mid, dtr, .. } = cx.settings.view();

    Glyph::builder()
        .outline(Ring::at((mid, dtr), (dtr, dtr)))
        .build()
}

pub fn comma(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        mid, stw, dtr, dsc, ..
    } = cx.settings.view();

    Glyph::builder()
        .or_outlines([
            Ring::at((mid, dtr), (dtr, dtr)).into_outline(),
            SpiroBuilder::open()
                .insts([
                    g4!(mid - dtr / 2., dsc).width(1.).aligned(Alignment::Right),
                    g4!(mid + dtr / 2., dsc / 2.)
                        .width(0.9)
                        .aligned(Alignment::Right),
                    g4!(mid + dtr, dtr)
                        .width(0.8)
                        .heading(Dir::D)
                        .aligned(Alignment::Right),
                ])
                .stroked(stw)
                .into_outline(),
        ])
        .build()
}
