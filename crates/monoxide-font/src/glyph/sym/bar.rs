use std::ops::Range;

use monoxide_script::prelude::*;

use crate::{InputContext, dir::Alignment, prelude::*, shape::Rect};

pub fn dash(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        sbl, sbr, stw, cap, ..
    } = cx.settings().view();

    Glyph::builder()
        .outline(hbar(sbl..sbr, cap / 2.).stroked(stw))
        .build()
}

pub fn underscore(cx: &InputContext) -> Glyph {
    let FontParamSettingsView { sbl, sbr, stw, .. } = cx.settings().view();

    Glyph::builder()
        .outline(hbar(sbl..sbr, 0.).stroked(stw).aligned(Alignment::Left))
        .build()
}

pub fn pipe(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        cap, dsc, mid, stw, ..
    } = cx.settings().view();

    Glyph::builder()
        .outline(vbar(mid, dsc + stw..cap + stw).stroked(stw))
        .build()
}

pub fn equal(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        sbl,
        mid,
        sbr,
        stw,
        cap,
        ..
    } = cx.settings().view();

    let xr = sbl..sbr;
    let y = cap / 2.;
    let dy = (mid - sbl) / 2.;

    Glyph::builder()
        .or_outlines([hbar(xr.clone(), y - dy), hbar(xr, y + dy)].map(|it| it.stroked(stw)))
        .build()
}

pub fn plus(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        sbl,
        mid,
        sbr,
        stw,
        cap,
        ..
    } = cx.settings().view();

    let y = cap / 2.;
    let r = mid - sbl;

    Glyph::builder()
        .or_outlines([hbar(sbl..sbr, y), vbar(mid, y - r..y + r)].map(|it| it.stroked(stw)))
        .build()
}

fn hbar(xr: Range<f64>, y: f64) -> Rect {
    Rect::new((xr.start, y), (xr.end, y))
}

fn vbar(x: f64, yr: Range<f64>) -> Rect {
    Rect::new((x, yr.start), (x, yr.end))
}
