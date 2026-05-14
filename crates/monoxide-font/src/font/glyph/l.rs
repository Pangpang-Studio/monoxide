use std::{ops::Range, sync::Arc};

use monoxide_script::prelude::*;

use crate::{
    InputContext,
    font::{dir::Alignment, glyph::c::CShape, shape::Rect},
};

pub fn l(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, sbr, stw, cap } = cx.settings(); }

    Glyph::builder()
        .outlines(LShape::new(sbl..sbr, 0.0..cap).stroked(stw))
        .build()
}

pub fn l_cap(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, sbr, stw, xh, cap, mid, mih, ovs } = cx.settings(); }

    let serif_l = CShape::new((mid, mih), (mid - sbl, mih), ovs).aperture_curve_h_lo();
    let serif = Rect::new((sbr, stw), (sbr, stw + serif_l)).aligned(Alignment::Right);

    let pipe = Rect::new((sbl, 0.), (sbl, cap)).aligned(Alignment::Left);
    let bar = Rect::new((sbl, 0.), (sbr, 0.)).aligned(Alignment::Right);

    Glyph::builder()
        .outlines([serif, pipe, bar].map(|it| it.stroked(stw).into_outline()))
        .build()
}

pub struct LShape {
    pub x_range: Range<f64>,
    pub y_range: Range<f64>,
    pub top_bar_scale: Range<f64>,
}

impl LShape {
    pub const DEFAULT_TOP_BAR_SCALE: Range<f64> = -0.85..0.;

    pub fn new(x_range: Range<f64>, y_range: Range<f64>) -> Self {
        Self {
            x_range,
            y_range,
            top_bar_scale: Self::DEFAULT_TOP_BAR_SCALE,
        }
    }

    pub fn with_top_bar_scale(mut self, top_bar_scale: impl Into<Option<Range<f64>>>) -> Self {
        self.top_bar_scale = top_bar_scale.into().unwrap_or(Self::DEFAULT_TOP_BAR_SCALE);
        self
    }
}

impl IntoOutlines for LShape {
    type Outlines = [Arc<OutlineExpr>; 3];

    fn into_outlines(self) -> Self::Outlines {
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

        let top_serif = Rect::new(
            (mid + top_bar_scale.start * hw, y_max),
            (mid + top_bar_scale.end * hw, y_max),
        )
        .aligned(Alignment::Left);

        let bottom_serif =
            Rect::new((mid - hw, y_min), (mid + hw, y_min)).aligned(Alignment::Right);

        [top_serif, pipe, bottom_serif].map(|it| it.into_outline())
    }
}
