use std::ops::Range;

use monoxide_script::prelude::*;

use crate::{InputContext, dir::Alignment, prelude::*, shape::Rect};

pub fn dash(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        sbl, sbr, stw, cap, ..
    } = cx.settings().view();

    Glyph::builder()
        .outline(bar(sbl..sbr, cap / 2.).stroked(stw))
        .build()
}

pub fn underscore(cx: &InputContext) -> Glyph {
    let FontParamSettingsView { sbl, sbr, stw, .. } = cx.settings().view();

    Glyph::builder()
        .outline(bar(sbl..sbr, 0.).stroked(stw).aligned(Alignment::Left))
        .build()
}

fn bar(xr: Range<f64>, y: f64) -> Rect {
    Rect::new((xr.start, y), (xr.end, y))
}
