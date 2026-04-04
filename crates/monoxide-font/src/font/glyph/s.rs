use std::sync::Arc;

use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::{
    dir::{Alignment, Dir},
    glyph::{c::CShape, o::OShape},
};

pub fn s(cx: &InputContext) -> Glyph {
    let_settings! { { mid, mih, ovs, sbl, stw } = cx.settings(); }

    Glyph::builder()
        .outline(Hook::new((mid, mih), (mid - sbl, mih), ovs).stroked(stw))
        .build()
}

struct Hook {
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
    fn into_outline(self) -> Arc<OutlineExpr> {
        let o_shape @ OShape {
            center: Point2D { x, y },
            radii: Point2D { x: rx, y: ry },
            ovs,
            ..
        } = self.o_shape;

        let mid_curve_w = o_shape.mid_curve_w();
        let mid_curve_h = o_shape.mid_curve_h();

        let c_shape = CShape { o_shape };
        let aperture_curve_h_hi = c_shape.aperture_curve_h_hi();

        let y_hi = y + ry;
        let y_lo = y - ry;

        SpiroBuilder::open()
            .insts([
                // Top arc
                g4!(x + rx * 0.9, y_hi - aperture_curve_h_hi * 0.75),
                corner!(x + rx * 0.9, y_hi - mid_curve_h * 0.75),
                g4!(x, y_hi + ovs),
                g4!(x - mid_curve_w, y_hi - mid_curve_h * 0.8).aligned(Alignment::Right),
                // Midpoint
                g4!(x, y).aligned(Alignment::Middle),
                // Bottom arc
                g4!(x + mid_curve_w, y_lo + mid_curve_h * 0.8).aligned(Alignment::Left),
                g4!(x, y_lo - ovs),
                g4!(x - mid_curve_w, y_lo + mid_curve_h * 0.75),
                g4!(x - rx, y_lo + mid_curve_h * 1.75).heading(Dir::U),
            ])
            .into_outline()
    }
}
