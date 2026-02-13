use std::{ops::Range, sync::Arc};

use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::{dir::Alignment, shape::Rect};

pub fn l(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, sbr, stw, cap } = cx.settings(); }

    Glyph::builder()
        .outlines(LShape::new(sbl..sbr, 0.0..cap).stroked(stw))
        .build()
}

pub struct LShape {
    pub x_range: Range<f64>,
    pub y_range: Range<f64>,
    pub top_bar_scale: f64,
}

impl LShape {
    pub const DEFAULT_TOP_BAR_SCALE: f64 = 0.85;

    pub fn new(x_range: Range<f64>, y_range: Range<f64>) -> Self {
        Self {
            x_range,
            y_range,
            top_bar_scale: Self::DEFAULT_TOP_BAR_SCALE,
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

        let pipe = Rect::new((mid, y_min), (mid, y_max));

        let top_serif =
            Rect::new((mid, y_max), (mid - top_bar_scale * hw, y_max)).aligned(Alignment::Right);

        let bottom_serif =
            Rect::new((mid - hw, y_min), (mid + hw, y_min)).aligned(Alignment::Right);

        [top_serif, pipe, bottom_serif]
            .into_iter()
            .map(|it| it.into_outline())
    }
}
