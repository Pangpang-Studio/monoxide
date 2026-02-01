use std::sync::Arc;

use monoxide_script::prelude::*;

use super::{InputContext, o::OShape};
use crate::font::{dir::Alignment, math::mix};

pub fn c(cx: &InputContext) -> Glyph {
    let_settings! { { mid, mih, ovs, sbl, stw } = cx.settings(); }
    Glyph::builder()
        .outline(CShape::new((mid, mih), (mid - sbl, mih), ovs).stroked(stw))
        .build()
}

pub struct CShape {
    pub o_shape: OShape,
}

impl CShape {
    pub fn new(center: impl Into<Point2D>, radii: impl Into<Point2D>, ovs: f64) -> Self {
        Self {
            o_shape: OShape::new(center, radii, ovs),
        }
    }

    pub fn mid_curve_h(&self) -> f64 {
        self.o_shape.mid_curve_h()
    }

    pub fn mid_curve_w(&self) -> f64 {
        self.o_shape.mid_curve_w()
    }

    pub fn end_curve_h(&self) -> f64 {
        self.o_shape.end_curve_h()
    }

    pub fn aperture_curve_h_hi(&self) -> f64 {
        mix(self.mid_curve_h(), self.end_curve_h(), 0.2)
    }

    pub fn aperture_curve_h_lo(&self) -> f64 {
        self.aperture_curve_h_hi() / 5.
    }
}

impl IntoOutline for CShape {
    fn into_outline(self) -> Arc<OutlineExpr> {
        let OShape {
            center: Point2D { x, y },
            radii: Point2D { x: rx, y: ry },
            ovs,
        } = self.o_shape;

        let mid_curve_w = self.mid_curve_w();
        let mid_curve_h = self.mid_curve_h();
        let end_curve_h = self.end_curve_h();

        let y_hi = y + ry;
        let y_lo = y - ry;

        SpiroBuilder::open()
            .insts([
                // Right side (upper)
                curl!(x + rx, y_hi - self.aperture_curve_h_hi()),
                // Top arc
                corner!(x + rx, y_hi - mid_curve_h),
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
