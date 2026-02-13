use std::sync::Arc;

use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::{
    dir::Alignment, glyph::o::OShape, math::mix, settings::FontParamSettings, shape::Rect,
};

pub fn m(cx: &InputContext) -> Glyph {
    Glyph::builder()
        .outlines(MShape::from_settings(&cx.settings))
        .build()
}

struct MShape {
    pub hooks: [Arc<OutlineExpr>; 2],
    pub pipe: Rect,
}

impl MShape {
    pub fn from_settings(settings: &FontParamSettings) -> Self {
        let settings = FontParamSettings {
            side_bearing: settings.side_bearing / 1.5,
            ..*settings
        };
        let_settings! { { mid, mih, ovs, sbl, sbr, stw, xh } = settings; }

        let hook = Hook::new((mix(mid, sbr, 0.5), mih), ((mid - sbl) / 2., mih), ovs).stroked(stw);
        let hooks = [
            hook.clone(),
            hook.transformed(Affine2D::translated((sbl - mid + stw / 2., 0.).into())),
        ];
        let pipe = Rect::new((sbl, 0.), (sbl, xh), stw).aligned(Alignment::Left);

        Self { hooks, pipe }
    }
}

impl IntoOutlines for MShape {
    fn into_outlines(self) -> impl Iterator<Item = Arc<OutlineExpr>> {
        self.hooks.into_outlines().chain([self.pipe.into_outline()])
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
                flat!(x + rx, 0.).aligned(Alignment::Right).width(1.),
                curl!(x + rx, y + ry / 2.),
                // Top arc
                g4!(x + mid_curve_w, y_hi - mid_curve_h / 2.).width(1.),
                g4!(x, y_hi + ovs).width(0.9),
                g4!(x - mid_curve_w * 1.5, y_hi - mid_curve_h * 1.25).width(0.7),
            ])
            .into_outline()
    }
}
