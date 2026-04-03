use std::sync::Arc;

use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::{
    dir::{Alignment, Dir},
    glyph::{d::Bowl, j::JShape, o::OShape},
    settings::FontParamSettings,
};

pub fn g(cx: &InputContext) -> Glyph {
    Glyph::builder()
        .outlines(GShape::from_settings(&cx.settings))
        .build()
}

pub struct GShape {
    bowl: Arc<OutlineExpr>,
    hook: Arc<OutlineExpr>,
}

impl GShape {
    pub fn from_settings(settings: &FontParamSettings) -> Self {
        let_settings! { { cap, mid, mih, ovs, sbl, sbr, stw, xh, dsc } = settings; }

        let bowl = Bowl::new((mid, mih), (mid - sbl, mih), ovs)
            .stroked(stw)
            .into_outline();
        let hook = Hook::new((mid, mih - dsc), (mid - sbl, mih), ovs)
            .stroked(1.05 * stw)
            .transformed(Affine2D::mirrored_along((0., xh / 2.), (1., 0.)));

        Self { bowl, hook }
    }
}

impl IntoOutlines for GShape {
    type Outlines = [Arc<OutlineExpr>; 2];

    fn into_outlines(self) -> Self::Outlines {
        [self.bowl.into_outline(), self.hook.into_outline()]
    }
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
                g4!(x + mid_curve_w * 0.9, y_hi - mid_curve_h / 2.).width(1.),
                g4!(x, y_hi + ovs).width(0.9).heading(Dir::L),
                g4!(x - mid_curve_w * 0.9, y_hi - mid_curve_h / 2.)
                    .width(1.)
                    .aligned(Alignment::Right),
                g4!(x - rx, y_hi - mid_curve_h * 1.2)
                    .width(1.)
                    .heading(JShape::HOOK_TIP_HEADING),
            ])
            .into_outline()
    }
}
