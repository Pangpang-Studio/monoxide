use monoxide_script::prelude::*;

use crate::{InputContext, prelude::*};

pub fn apostrophe(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        mid,
        xh,
        dtr,
        stw,
        cap,
        ..
    } = cx.settings().view();

    Glyph::builder()
        .outline(
            SpiroBuilder::open()
                .insts([
                    flat!(mid, cap + dtr).width(1.2),
                    curl!(mid, xh - stw).width(1.),
                ])
                .stroked(stw),
        )
        .build()
}
