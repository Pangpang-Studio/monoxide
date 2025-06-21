use monoxide_script::{
    ast::SimpleGlyph,
    corner,
    dsl::{IntoOutlineExt, SpiroBuilder},
};

use crate::InputContext;

pub fn tofu(fcx: &InputContext) -> SimpleGlyph {
    let h = fcx.settings().cap();
    let w = fcx.settings().wth();
    let stw = fcx.settings().stw();
    SimpleGlyph::new().outline(
        SpiroBuilder::closed()
            .insts([
                corner!(stw, stw),
                corner!(w - stw, stw),
                corner!(w - stw, h - stw),
                corner!(stw, h - stw),
            ])
            .stroked(fcx.settings().stw()),
    )
}
