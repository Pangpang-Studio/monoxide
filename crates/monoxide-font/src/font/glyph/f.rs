use std::sync::Arc;

use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::{
    dir::{Alignment, Dir},
    glyph::{n::Hook, o::OShape},
    settings::FontParamSettings,
    shape::Rect,
};

pub fn f(cx: &InputContext) -> Glyph {
    Glyph::builder()
        .outlines(FShape::from_settings(&cx.settings))
        .build()
}

pub struct FShape {
    pub hook: Arc<OutlineExpr>,
    pub pipe: Rect,
}

impl FShape {
    pub fn from_settings(settings: &FontParamSettings) -> Self {
        let_settings! { { cap, mid, mih, ovs, sbl, sbr, stw, xh } = settings; }

        let hook = Hook::new((mid, mih + cap - xh), (mid - sbl, mih), ovs)
            .stroked(stw)
            .transformed(Affine2D::mirrored_along(
                (mid, 0.).into(),
                Point2D::unit_y(),
            ));
        let pipe = Rect::new((0., mih), (mid, mih), stw);

        Self { hook, pipe }
    }
}

impl IntoOutlines for FShape {
    fn into_outlines(self) -> impl Iterator<Item = std::sync::Arc<OutlineExpr>> {
        [self.hook.into_outline(), self.pipe.into_outline()].into_iter()
    }
}
