use monoxide_script::prelude::*;

use crate::{
    InputContext,
    font::{dir::Alignment, prelude::*},
};

pub fn tofu(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        cap, sbl, sbr, stw, ..
    } = cx.settings().view();

    Glyph::builder()
        .outline(
            SpiroBuilder::closed()
                .insts([
                    corner!(sbl, 0.).aligned(Alignment::Right),
                    corner!(sbr, 0.),
                    corner!(sbr, cap),
                    corner!(sbl, cap),
                ])
                .stroked(stw),
        )
        .build()
}
