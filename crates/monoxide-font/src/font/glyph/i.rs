use monoxide_script::{
    ast::Glyph,
    corner,
    dsl::{IntoOutlineExt, SpiroBuilder},
    g4, let_settings,
};

use super::InputContext;
use crate::font::{
    math::mix,
    shape::{Rect, Ring},
};

pub fn i(fcx: &InputContext) -> Glyph {
    let_settings! { { cap, dtr, mid, sbl, sbr, stw, xh } = fcx.settings(); }

    let hstw = stw / 2.;

    Glyph::builder()
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
        .build()
}
