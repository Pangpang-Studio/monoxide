use std::sync::Arc;

use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::{
    dir::{Alignment, Dir},
    glyph::o::OShape,
    settings::FontParamSettings,
    shape::Rect,
};

pub fn r(cx: &InputContext) -> Glyph {
    Glyph::builder()
        .outlines(RShape::from_settings(&cx.settings))
        .build()
}

pub struct RShape {
    pub hook: Arc<OutlineExpr>,
    pub pipe: Rect,
    pub offset: Point2D,
}

impl RShape {
    pub fn from_settings(settings: &FontParamSettings) -> Self {
        let_settings! { { mid, mih, ovs, sbl, stw, xh } = settings; }

        let hook = Hook::new((mid, mih), (mid - sbl, mih), ovs).stroked(stw);

        let pipe = Rect::new((sbl, 0.), (sbl, xh))
            .aligned(Alignment::Left)
            .stroked(stw);

        Self {
            hook,
            pipe,
            offset: (stw / 2., 0.).into(),
        }
    }
}

impl IntoOutlines for RShape {
    fn into_outlines(self) -> impl Iterator<Item = Arc<OutlineExpr>> {
        [self.hook.into_outline(), self.pipe.into_outline()]
            .into_iter()
            .map(move |it| it.transformed(Affine2D::translated(self.offset)))
    }
}

pub struct Hook {
    pub o_shape: OShape,
    pub hook_tip_heading: Point2D,
}

impl Hook {
    pub fn new(center: impl Into<Point2D>, radii: impl Into<Point2D>, ovs: f64) -> Self {
        Self {
            o_shape: OShape::new(center, radii, ovs),
            hook_tip_heading: Dir::L.into(),
        }
    }
}

impl IntoOutline for Hook {
    fn into_outline(self) -> Arc<OutlineExpr> {
        let o_shape @ OShape {
            center: Point2D { x, y },
            radii: Point2D { y: ry, .. },
            ovs,
        } = self.o_shape;

        let mid_curve_w = o_shape.mid_curve_w();
        let mid_curve_h = o_shape.mid_curve_h();

        let y_hi = y + ry;

        SpiroBuilder::open()
            .insts([
                g4!(x + mid_curve_w + ovs, y_hi)
                    .aligned(Alignment::Right)
                    .heading(self.hook_tip_heading)
                    .width(1.1),
                g4!(x + mid_curve_w / 1.5, y_hi + ovs),
                g4!(x - mid_curve_w / 2., y_hi - mid_curve_h * 0.75),
                g4!(x - mid_curve_w, y_hi - mid_curve_h * 2.)
                    .heading(self.hook_tip_heading)
                    .width(1.),
            ])
            .into_outline()
    }
}
