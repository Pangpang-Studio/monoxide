use std::{ops::Range, sync::Arc};

use monoxide_script::prelude::*;

use crate::{
    InputContext,
    dir::{Alignment, Dir},
    math::mix,
    prelude::*,
    shape::Rect,
};

pub fn k(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        sbl,
        sbr,
        xh,
        mih,
        stw,
        cap,
        ..
    } = cx.settings().view();

    let pipe = Rect::new((sbl + stw / 4., 0.), (sbl + stw / 4., cap)).aligned(Alignment::Left);
    let chevron = Chevron::new(mix(sbr, sbl, 0.1) + stw..sbr, 0.0..xh);
    let bar = Rect::new((sbl + stw / 4., mih), chevron.corner() + (stw, 0.).into());

    Glyph::builder()
        .outlines([
            pipe.stroked(stw).into_outline(),
            // TODO: Find out how the scaling factors are determined.
            chevron.stroked(stw * 0.8),
            bar.stroked(stw * 0.9).into_outline(),
        ])
        .build()
}

#[derive(Clone, Debug)]
pub struct Chevron {
    pub xr: Range<f64>,
    pub yr: Range<f64>,
}

impl Chevron {
    pub fn new(xr: Range<f64>, yr: Range<f64>) -> Self {
        Self { xr, yr }
    }

    pub const fn corner(&self) -> Point2D {
        let Range { start: y0, end: y1 } = self.yr;
        Point2D::new(self.xr.start, y0.midpoint(y1))
    }
}

impl IntoOutline for Chevron {
    fn into_outline(self) -> Arc<OutlineExpr> {
        let Range { start: x0, end: x1 } = self.xr;
        let Range { start: y0, end: y1 } = self.yr;

        SpiroBuilder::open()
            .insts([
                corner!(x1, y0).aligned(Alignment::Right).heading(Dir::U),
                corner!(x0, y0.midpoint(y1)).aligned(Alignment::Left),
                corner!(x1, y1).aligned(Alignment::Right).heading(Dir::D),
            ])
            .build()
            .into_outline()
    }
}
