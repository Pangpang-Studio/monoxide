use std::{ops::Range, sync::Arc};

use monoxide_script::prelude::*;

use crate::{
    InputContext,
    font::{
        dir::Alignment,
        glyph::c::CShape,
        shape::{Rect, Slash},
    },
};

pub fn z(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, sbr, xh, ovs, stw } = cx.settings(); }

    Glyph::builder()
        .outlines(ZShape::new(sbl..sbr, 0.0..xh, ovs, stw))
        .build()
}

pub fn z_cap(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, sbr, cap, ovs, stw } = cx.settings(); }

    Glyph::builder()
        .outlines(ZShape::new(sbl..sbr, 0.0..cap, ovs, stw))
        .build()
}

pub struct ZShape {
    pub c_shape: CShape,
    pub stw: Option<f64>,
}

impl ZShape {
    pub fn new(xr: Range<f64>, yr: Range<f64>, ovs: f64, stw: impl Into<Option<f64>>) -> Self {
        let (x, y) = (xr.start.midpoint(xr.end), yr.start.midpoint(yr.end));
        let (rx, ry) = (x - xr.start, y - yr.start);
        Self {
            c_shape: CShape::new((x, y), (rx, ry), ovs),
            stw: stw.into(),
        }
    }

    pub fn left(&self) -> f64 {
        self.c_shape.left()
    }

    pub fn right(&self) -> f64 {
        self.c_shape.right()
    }
}

impl IntoOutlines for ZShape {
    type Outlines = [Arc<OutlineExpr>; 4];

    fn into_outlines(self) -> Self::Outlines {
        let (sbl, sbr) = (self.left(), self.right());

        let c_shape = &self.c_shape;
        let serif_l = c_shape.aperture_curve_h_lo();

        let o_shape = &c_shape.o_shape;
        let (bot, top) = (
            o_shape.center.y - o_shape.radii.y,
            o_shape.center.y + o_shape.radii.y,
        );

        let stw = self.stw.unwrap_or_default();

        let serif = Rect::new((sbl, top - serif_l - stw), (sbl, top)).aligned(Alignment::Left);
        let slash = Slash::new(sbl..sbr, (bot + stw)..(top - stw));
        let top_bar = Rect::new((sbl, top), (sbr, top)).aligned(Alignment::Left);
        let bottom_bar = Rect::new((sbl, bot), (sbr, bot)).aligned(Alignment::Right);

        if self.stw.is_none() {
            return [
                serif.into_outline(),
                top_bar.into_outline(),
                bottom_bar.into_outline(),
                slash.into_outline(),
            ];
        };

        [
            serif.into_outline().stroked(stw),
            top_bar.into_outline().stroked(stw),
            bottom_bar.into_outline().stroked(stw),
            slash.into_outline().stroked(0.9 * stw),
        ]
    }
}
