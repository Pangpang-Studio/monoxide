use std::sync::Arc;

use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::{dir::Alignment, glyph::o::OShape, settings::FontParamSettings, shape::Rect};

pub fn d(cx: &InputContext) -> Glyph {
    Glyph::builder()
        .outlines(DShape::from_settings(&cx.settings))
        .build()
}

pub struct DShape {
    bowl: Arc<OutlineExpr>,
    pipe: Rect,
}

impl DShape {
    pub fn from_settings(settings: &FontParamSettings) -> Self {
        let_settings! { { cap, mid, mih, ovs, sbl, sbr, stw } = settings; }

        let bowl = Bowl::new((mid, mih), (mid - sbl, mih), ovs)
            .stroked(stw)
            .into_outline();
        let pipe = Rect::new((sbr, 0.), (sbr, cap), stw).aligned(Alignment::Right);

        Self { bowl, pipe }
    }

    pub fn with_height(mut self, height: f64) -> Self {
        self.pipe.end.y = height;
        self
    }
}

impl IntoOutlines for DShape {
    fn into_outlines(self) -> impl Iterator<Item = std::sync::Arc<OutlineExpr>> {
        [self.bowl, self.pipe.into_outline()].into_iter()
    }
}

struct Bowl {
    pub o_shape: OShape,
}

impl Bowl {
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
}

impl IntoOutline for Bowl {
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

        SpiroBuilder::closed()
            .insts([
                // Left side
                flat!(x - rx, y_hi - end_curve_h),
                curl!(x - rx, y_lo + end_curve_h),
                // Bottom arc
                g4!(x - mid_curve_w, y_lo + mid_curve_h)
                    .aligned(Alignment::Right)
                    .width(1.),
                g4!(x, y_lo - ovs),
                g4!(x + mid_curve_w, y_lo + mid_curve_h * 1.25).width(0.9),
                // Right side
                g4!(x + rx, y_hi - end_curve_h).width(0.9),
                // Top arc
                g4!(x + mid_curve_w, y_hi - mid_curve_h).width(1.),
                g4!(x, y_hi + ovs),
                g4!(x - mid_curve_w, y_hi - mid_curve_h),
            ])
            .into_outline()
    }
}
