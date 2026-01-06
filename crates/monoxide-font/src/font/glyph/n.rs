use std::sync::Arc;

use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::{
    dir::{Alignment, Dir},
    glyph::o::OShape,
    settings::FontParamSettings,
    shape::Rect,
};

pub fn n(cx: &InputContext) -> Glyph {
    Glyph::builder()
        .outlines(NShape::from_settings(&cx.settings))
        .build()
}

pub struct NShape {
    pub hook: Arc<OutlineExpr>,
    pub pipe: Rect,
}

impl NShape {
    pub fn from_settings(settings: &FontParamSettings) -> Self {
        let_settings! { { mid, mih, ovs, sbl, stw, xh } = settings; }

        let hook = Hook::new((mid, mih), (mid - sbl, mih), ovs).stroked(stw);
        let pipe = Rect::new((sbl, 0.), (sbl, xh), stw).aligned(Alignment::Left);

        Self { hook, pipe }
    }

    pub fn with_height(mut self, height: f64) -> Self {
        self.pipe.end.y = height;
        self
    }
}

impl IntoOutlines for NShape {
    fn into_outlines(self) -> impl Iterator<Item = std::sync::Arc<OutlineExpr>> {
        [self.hook.into_outline(), self.pipe.into_outline()].into_iter()
    }
}

pub struct Hook {
    pub o_shape: OShape,
}

impl Hook {
    pub fn new(center: impl Into<Point2D>, radii: impl Into<Point2D>, ovs: f64) -> Self {
        Self {
            o_shape: OShape::new(center, radii, ovs),
        }
    }
}

impl IntoOutline for Hook {
    fn into_outline(self) -> std::sync::Arc<OutlineExpr> {
        let o_shape @ OShape {
            center: Point2D { x, y },
            radii: Point2D { x: rx, y: ry },
            ovs,
        } = self.o_shape;

        let mid_curve_w = o_shape.mid_curve_w();
        let mid_curve_h = o_shape.mid_curve_h();

        let y_hi = y + ry;

        SpiroBuilder::open()
            .insts([
                // Right side
                flat!(x + rx, 0.).aligned(Alignment::Right).width(1.1),
                curl!(x + rx, y + ry / 3.),
                // Top arc
                g4!(x + mid_curve_w, y_hi - mid_curve_h / 2.).width(1.),
                g4!(x, y_hi + ovs).width(0.9),
                g4!(x - mid_curve_w, y_hi - mid_curve_h * 1.25)
                    .heading(Dir::L)
                    .width(0.7),
            ])
            .into_outline()
    }
}
