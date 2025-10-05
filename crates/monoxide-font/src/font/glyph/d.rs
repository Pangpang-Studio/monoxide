use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::{glyph::o::OShape, shape::Rect};

pub fn d(cx: &InputContext) -> Glyph {
    let_settings! { { cap, mid, mih, ovs, sbl, sbr, stw } = cx.settings(); }

    let hstw = stw / 2.;

    Glyph::builder()
        .outline(OShape::new((mid, mih), (mid - sbl, mih), ovs).stroked(stw))
        .outline(Rect::new((sbr - hstw, 0.), (sbr - hstw, cap), stw))
        .build()
}
