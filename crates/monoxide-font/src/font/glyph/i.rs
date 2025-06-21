use monoxide_script::{
    ast::SimpleGlyph,
    corner,
    dsl::{IntoOutlineExt, SpiroBuilder},
    g4, let_settings,
};

use crate::{
    InputContext,
    font::{
        math::mix,
        shape::{Rect, Ring},
    },
};

pub fn i(fcx: &InputContext) -> SimpleGlyph {
    let_settings! { { cap, dtr, mid, sbl, sbr, stw, xh } = fcx.settings(); }

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
