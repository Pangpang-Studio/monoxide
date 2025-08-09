use std::{ops::Range, sync::Arc};

use monoxide_script::{
    ast::{Glyph, OutlineExpr},
    corner,
    dsl::{IntoOutline, IntoOutlines, IntoOutlinesExt, SpiroBuilder},
    g4, let_settings,
};

use super::InputContext;
use crate::font::{math::mix, shape::Ring};

pub fn i(fcx: &InputContext) -> Glyph {
    let_settings! { { sbl, sbr, stw, xh } = fcx.settings(); }

    let hstw = stw / 2.;

    Glyph::builder()
        .outlines(LShape::new(sbl..sbr, hstw..(xh - hstw), 0.85).stroked(stw))
        .outline(i_dot(fcx))
        .build()
}

pub fn i_dot(fcx: &InputContext) -> impl IntoOutline {
    let_settings! { { cap, dtr, mid, sbl } = fcx.settings(); }
    Ring::at((mix(mid, sbl, 0.95), cap), (dtr, dtr))
}

pub struct LShape {
    pub x_range: Range<f64>,
    pub y_range: Range<f64>,
    pub top_bar_scale: f64,
}

impl LShape {
    pub fn new(x_range: Range<f64>, y_range: Range<f64>, top_bar_scale: f64) -> Self {
        Self {
            x_range,
            y_range,
            top_bar_scale,
        }
    }
}

impl IntoOutlines for LShape {
    fn into_outlines(self) -> impl Iterator<Item = Arc<OutlineExpr>> {
        let Self {
            x_range: Range {
                start: x_min,
                end: x_max,
            },
            y_range: Range {
                start: y_min,
                end: y_max,
            },
            top_bar_scale,
        } = self;

        let mid = (x_min + x_max) / 2.;
        let hw = (x_max - x_min) / 2.;

        vec![
            SpiroBuilder::open()
                .insts([
                    g4!(mid, y_min),
                    corner!(mid, y_max),
                    g4!(mid - top_bar_scale * hw, y_max),
                ])
                .into_outline(),
            SpiroBuilder::open()
                .insts([g4!(mid - hw, y_min), g4!(mid + hw, y_min)])
                .into_outline(),
        ]
        .into_iter()
    }
}
