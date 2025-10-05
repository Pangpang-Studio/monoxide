use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::{dir::Alignment, glyph::o::OShape, shape::Rect};

pub fn d(cx: &InputContext) -> Glyph {
    let_settings! { { cap, mid, mih, ovs, sbl, sbr, stw } = cx.settings(); }

    Glyph::builder()
        .outline(OShape::new((mid, mih), (mid - sbl, mih), ovs).stroked(stw))
        .outline(Rect::new((sbr, 0.), (sbr, cap), stw).aligned(Alignment::Right))
        .build()
}
