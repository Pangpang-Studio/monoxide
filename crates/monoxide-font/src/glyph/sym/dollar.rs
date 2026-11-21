use monoxide_script::prelude::*;

use super::tick::Tick;
use crate::{InputContext, glyph::s::SShape, prelude::*};

pub fn dollar(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        mid,
        sbl,
        dsc,
        cap,
        stw,
        ovs,
        ..
    } = cx.settings().view();

    let tick_hi = {
        let top = (mid, cap - dsc / 2.);
        let bot = Point2D::from(top) - (0., 0.35);
        Tick::new(top, bot).stroked(stw).into_outline()
    };
    let tick_lo = tick_hi
        .clone()
        .transformed(Affine2D::reflect((mid, cap / 2.), (1., 0.)));

    Glyph::builder()
        .or_outlines([
            tick_hi,
            SShape::new((mid, cap / 2.), (mid - sbl, cap * 0.45), ovs).stroked(stw),
            tick_lo,
        ])
        .build()
}
