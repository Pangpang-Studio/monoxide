use monoxide_script::{
    ast::Glyph,
    corner,
    dsl::{IntoOutlineExt, SpiroBuilder},
    let_settings,
};

use super::InputContext;

pub fn tofu(cx: &InputContext) -> Glyph {
    let_settings! { { cap: h, wth: w, stw } = cx.settings(); }

    Glyph::builder()
        .outline(
            SpiroBuilder::closed()
                .insts([
                    corner!(stw, stw),
                    corner!(w - stw, stw),
                    corner!(w - stw, h - stw),
                    corner!(stw, h - stw),
                ])
                .stroked(stw),
        )
        .build()
}
