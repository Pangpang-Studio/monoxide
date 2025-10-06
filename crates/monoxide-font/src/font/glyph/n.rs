use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::{
    dir::{Alignment, Dir},
    glyph::o::OShape,
    shape::Rect,
};

pub fn n(cx: &InputContext) -> Glyph {
    let_settings! { { mid, mih, ovs, sbl, stw, xh } = cx.settings(); }

    Glyph::builder()
        .outline(Rect::new((sbl, 0.), (sbl, xh), stw).aligned(Alignment::Left))
        .outline(Hook::new((mid, mih), (mid - sbl, mih), ovs).stroked(stw))
        .build()
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
                flat!(x + rx, 0.).aligned(Alignment::Right),
                curl!(x + rx, y + ry / 3.),
                // Top arc
                g4!(x + mid_curve_w, y_hi - mid_curve_h / 2.),
                g4!(x, y_hi + ovs).width(1.),
                g4!(x - mid_curve_w, y_hi - mid_curve_h * 1.1)
                    .heading(Dir::L)
                    .width(0.5),
            ])
            .into_outline()
    }
}
