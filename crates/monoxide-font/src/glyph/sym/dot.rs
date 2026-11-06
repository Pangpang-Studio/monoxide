use std::sync::Arc;

use monoxide_script::prelude::*;

use crate::{
    InputContext,
    dir::{Alignment, Dir},
    prelude::*,
    shape::{Rect, Ring},
};

pub fn dot(cx: &InputContext) -> Glyph {
    let FontParamSettingsView { mid, dtr, .. } = cx.settings.view();

    Glyph::builder()
        .outline(Ring::at((mid, dtr), (dtr, dtr)))
        .build()
}

pub fn colon(cx: &InputContext) -> Glyph {
    let FontParamSettingsView { mid, xh, dtr, .. } = cx.settings.view();

    Glyph::builder()
        .outlines([
            Ring::at((mid, xh - dtr), (dtr, dtr)),
            Ring::at((mid, dtr), (dtr, dtr)),
        ])
        .build()
}

pub fn comma(cx: &InputContext) -> Glyph {
    let FontParamSettingsView { mid, dtr, dsc, .. } = cx.settings.view();

    Glyph::builder()
        .outline(CommaShape::new(mid, dsc, dtr))
        .build()
}

struct CommaShape {
    mid: f64,
    dtr: f64,
    dsc: f64,
}

impl CommaShape {
    fn new(mid: f64, dsc: f64, dtr: f64) -> Self {
        Self { mid, dsc, dtr }
    }
}

impl IntoOutline for CommaShape {
    fn into_outline(self) -> Arc<OutlineExpr> {
        let Self { mid, dtr, dsc } = self;

        Ring::at((mid, dtr), (dtr, dtr)).or(SpiroBuilder::open()
            .insts([
                g4!(mid - dtr / 2., dsc).width(1.).aligned(Alignment::Right),
                g4!(mid + dtr / 2., dsc / 2.)
                    .width(0.9)
                    .aligned(Alignment::Right),
                g4!(mid + dtr, dtr)
                    .width(0.8)
                    .heading(Dir::D)
                    .aligned(Alignment::Right),
            ])
            .stroked(dtr))
    }
}

pub fn exclamation(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        mid, stw, dtr, cap, ..
    } = cx.settings.view();

    Glyph::builder()
        .outline(Ring::at((mid, dtr), (dtr, dtr)))
        .outline(Rect::new((mid, dtr * 3.5), (mid, cap)).stroked(stw))
        .build()
}
