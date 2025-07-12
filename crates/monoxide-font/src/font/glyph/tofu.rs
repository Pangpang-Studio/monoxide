use monoxide_script::{
    ast::Glyph,
    corner,
    dsl::{IntoOutlineExt, SpiroBuilder},
};

use super::InputContext;

pub fn tofu(fcx: &InputContext) -> Glyph {
    let h = fcx.settings().cap();
    let w = fcx.settings().wth();
    let stw = fcx.settings().stw();
    Glyph::builder()
        .outline(
            SpiroBuilder::closed()
                .insts([
                    corner!(stw, stw),
                    corner!(w - stw, stw),
                    corner!(w - stw, h - stw),
                    corner!(stw, h - stw),
                ])
                .stroked(fcx.settings().stw()),
        )
        .build()
}
