use monoxide_script::{
    ast::Glyph,
    corner,
    dsl::{IntoOutlineExt, SpiroBuilder},
    let_settings,
};

use crate::{InputContext, font::dir::Alignment};

pub fn space(_cx: &InputContext) -> Glyph {
    Glyph::builder().build()
}

pub fn tofu(cx: &InputContext) -> Glyph {
    let_settings! { { cap, sbl, sbr, stw } = cx.settings(); }

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
