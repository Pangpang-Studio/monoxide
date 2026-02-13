use std::sync::Arc;

use monoxide_script::prelude::*;

use super::{InputContext, c::CShape, o::OShape};
use crate::font::{
    dir::{Alignment, Dir},
    settings::FontParamSettings,
    shape::Rect,
};

pub fn e(cx: &InputContext) -> Glyph {
    Glyph::builder()
        .outlines(EShape::from_settings(cx.settings()))
        .build()
}

struct EShape {
    pub bowl: Arc<OutlineExpr>,
    pub bar: Rect,
}

impl EShape {
    pub fn from_settings(settings: &FontParamSettings) -> Self {
        let_settings! { { mid, mih, ovs, sbl, sbr, stw } = settings; }

        let bowl = Bowl::new((mid, mih), (mid - sbl, mih), ovs).stroked(stw);
        let bar = Rect::new((sbl, mih), (sbr, mih)).stroked(stw);

        Self { bowl, bar }
    }
}

impl IntoOutlines for EShape {
    fn into_outlines(self) -> impl Iterator<Item = Arc<OutlineExpr>> {
        [self.bowl.into_outline(), self.bar.into_outline()].into_iter()
    }
}

struct Bowl {
    pub c_shape: CShape,
}

impl Bowl {
    pub fn new(center: impl Into<Point2D>, radii: impl Into<Point2D>, ovs: f64) -> Self {
        Self {
            c_shape: CShape::new(center, radii, ovs),
        }
    }

    pub fn mid_curve_h(&self) -> f64 {
        self.c_shape.mid_curve_h()
    }

    pub fn mid_curve_w(&self) -> f64 {
        self.c_shape.mid_curve_w()
    }

    pub fn end_curve_h(&self) -> f64 {
        self.c_shape.end_curve_h()
    }

    pub fn aperture_curve_h_lo(&self) -> f64 {
        self.c_shape.aperture_curve_h_lo()
    }
}

impl IntoOutline for Bowl {
    fn into_outline(self) -> Arc<OutlineExpr> {
        let OShape {
            center: Point2D { x, y },
            radii: Point2D { x: rx, y: ry },
            ovs,
        } = self.c_shape.o_shape;

        let mid_curve_w = self.mid_curve_w();
        let mid_curve_h = self.mid_curve_h();
        let end_curve_h = self.end_curve_h();

        let y_hi = y + ry;
        let y_lo = y - ry;

        let rx1 = 0.9 * rx;

        SpiroBuilder::open()
            .insts([
                // Right side (upper)
                flat!(x + rx, y).heading(Dir::D),
                curl!(x + rx, y_hi - end_curve_h),
                // Top arc
                g4!(x + rx1, y_hi - mid_curve_h),
                g4!(x, y_hi + ovs),
                g4!(x - mid_curve_w, y_hi - mid_curve_h),
                // Left side
                flat!(x - rx, y_hi - end_curve_h),
                curl!(x - rx, y_lo + end_curve_h),
                // Bottom arc
                g4!(x - mid_curve_w, y_lo + mid_curve_h).aligned(Alignment::Right),
                g4!(x, y_lo - ovs),
                // One control point omitted.
                // Right side (lower)
                flat!(x + rx, y_lo + self.aperture_curve_h_lo()),
            ])
            .into_outline()
    }
}
