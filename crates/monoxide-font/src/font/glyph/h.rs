use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::{dir::Alignment, glyph::n::Hook, shape::Rect};

pub fn h(cx: &InputContext) -> Glyph {
    let_settings! { { mid, mih, ovs, sbl, stw, cap} = cx.settings(); }

    Glyph::builder()
        .outline(Rect::new((sbl, 0.), (sbl, cap), stw).aligned(Alignment::Left))
        .outline(Hook::new((mid, mih), (mid - sbl, mih), ovs).stroked(stw))
        .build()
}
