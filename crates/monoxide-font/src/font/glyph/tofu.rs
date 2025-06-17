use monoxide_script::{
    ast::{FontContext, SimpleGlyph},
    corner,
    dsl::{IntoOutlineExt, SpiroBuilder},
};

pub fn tofu(fcx: &FontContext) -> SimpleGlyph {
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
