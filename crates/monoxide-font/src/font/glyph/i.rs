use monoxide_script::{
    ast::{FontContext, SimpleGlyph},
    corner,
    dsl::{IntoOutlineExt, SpiroBuilder},
    g4,
};

use crate::font::{
    math::mix,
    shape::{Rect, Ring},
};

pub fn i(fcx: &FontContext) -> SimpleGlyph {
    let s = fcx.settings();

    let cap = s.cap();
    let mid = s.mid();
    let sbl = s.sbl();
    let sbr = s.sbr();
    let stw = s.stw();
    let xh = s.xh();
    let dtr = s.dtr();

    let hstw = stw / 2.;

    SimpleGlyph::new()
        .outline(
            SpiroBuilder::open()
                .insts([
                    g4!(mid, stw),
                    corner!(mid, xh - hstw),
                    g4!(mix(mid, sbl, 0.175), xh - hstw),
                ])
                .stroked(stw),
        )
        .outline(Rect::new((sbl, hstw), (sbr, hstw), stw))
        .outline(Ring::at((mid, cap - dtr), (dtr, dtr)))
}
