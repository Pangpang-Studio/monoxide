use std::sync::Arc;

use monoxide_script::{g2, prelude::*};

use crate::InputContext;
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

        let hook = {
            let rx = mid - sbl;
            let ry = mih;
            let hook_tip_width = 0.8;
            Hook::new((mid, mih), (rx, ry), ovs)
                .with_hook_tip_width(hook_tip_width)
                .with_hook_tip_r_factor(1. - stw * (1. - hook_tip_width) / rx)
                .stroked(stw)
        };

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
    type Outlines = [Arc<OutlineExpr>; 2];

    fn into_outlines(self) -> Self::Outlines {
        [self.hook.into_outline(), self.pipe.into_outline()]
    }
}

pub struct Hook {
    pub o_shape: OShape,
    pub hook_tip_heading: Option<Point2D>,
    pub hook_tip_width: Option<f64>,
    pub hook_tip_r_factor: Option<f64>,
}

impl Hook {
    pub fn new(center: impl Into<Point2D>, radii: impl Into<Point2D>, ovs: f64) -> Self {
        Self {
            o_shape: OShape::new(center, radii, ovs),
            hook_tip_heading: None,
            hook_tip_width: None,
            hook_tip_r_factor: None,
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

    pub fn with_hook_tip_r_factor(mut self, offset: impl Into<Option<f64>>) -> Self {
        self.hook_tip_r_factor = offset.into();
        self
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

        let r_factor = self.hook_tip_r_factor.unwrap_or(1.);
        let hook_tip_width = self.hook_tip_width.unwrap_or(1.);

        let y_hi = y + ry;
        let y_hi1 = y + ry * r_factor;

        SpiroBuilder::open()
            .insts([
                // Right side
                flat!(x + rx, 0.).aligned(Alignment::Right).width(1.1),
                curl!(x + rx, y + ry / 3.),
                // Top arc
                g2!(x + mid_curve_w * 0.9, y_hi - mid_curve_h / 2.).width(1.),
                g4!(x, y_hi + ovs).width(1.).heading(Dir::L),
                g2!(x - mid_curve_w * 0.9 * r_factor, y_hi1 - mid_curve_h / 2.)
                    .aligned(Alignment::Right),
                {
                    let mut tip =
                        g4!(x - rx * r_factor, y_hi1 - mid_curve_h * 1.4).width(hook_tip_width);
                    if let Some(heading) = self.hook_tip_heading {
                        tip = tip.heading(heading);
                    }
                    tip
                },
            ])
            .into_outline()
    }
}
