use std::sync::Arc;

use monoxide_script::{g2, prelude::*};

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

        let hook = Hook::new((mid, mih), (mid - sbl, mih), ovs)
            .with_hook_tip_width(0.8)
            .stroked(stw);

        let pipe = Rect::new((sbl, 0.), (sbl, xh))
            .aligned(Alignment::Left)
            .stroked(stw);

        Self { hook, pipe }
    }

    pub fn with_pipe_height(mut self, height: f64) -> Self {
        self.pipe.end.y = height;
        self
    }
}

impl IntoOutlines for NShape {
    fn into_outlines(self) -> impl Iterator<Item = Arc<OutlineExpr>> {
        [self.hook.into_outline(), self.pipe.into_outline()].into_iter()
    }
}

pub struct Hook {
    pub o_shape: OShape,
    pub hook_tip_heading: Option<Point2D>,
    pub hook_tip_width: Option<f64>,
}

impl Hook {
    pub fn new(center: impl Into<Point2D>, radii: impl Into<Point2D>, ovs: f64) -> Self {
        Self {
            o_shape: OShape::new(center, radii, ovs),
            hook_tip_heading: None,
            hook_tip_width: None,
        }
    }

    pub fn with_hook_tip_heading(mut self, heading: impl Into<Option<Point2D>>) -> Self {
        self.hook_tip_heading = heading.into();
        self
    }

    pub fn with_hook_tip_width(mut self, width: impl Into<Option<f64>>) -> Self {
        self.hook_tip_width = width.into();
        self
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
                g2!(x + mid_curve_w, y_hi - mid_curve_h / 2.).width(1.),
                g4!(x, y_hi + ovs).width(1.).heading(Dir::L),
                g2!(x - mid_curve_w * 0.7, y_hi - mid_curve_h * 0.2).aligned(Alignment::Right),
                {
                    let mut tip = g4!(x - rx, y_hi - mid_curve_h * 1.5)
                        .width(self.hook_tip_width.unwrap_or(1.));
                    if let Some(heading) = self.hook_tip_heading {
                        tip = tip.heading(heading);
                    }
                    tip
                },
            ])
            .into_outline()
    }
}
