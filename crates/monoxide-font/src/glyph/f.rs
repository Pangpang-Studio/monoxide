use std::sync::Arc;

use monoxide_script::prelude::*;

use crate::{InputContext, dir::Alignment, glyph::j::JShape, math::mix, prelude::*, shape::Rect};

pub fn f(cx: &InputContext) -> Glyph {
    Glyph::builder()
        .outlines(FShape::from_settings(&cx.settings))
        .build()
}

pub struct FShape {
    pub hook: Arc<OutlineExpr>,
    pub crossbar: Arc<OutlineExpr>,
    pub offset: Point2D,
}

impl FShape {
    pub fn from_settings(settings: &FontParamSettings) -> Self {
        let FontParamSettingsView {
            mid,
            mih,
            sbl,
            stw,
            xh,
            cap,
            ..
        } = settings.view();

        let hook = JShape::hook_raw(settings, cap)
            .transformed(Affine2D::mirrored_along((mid, mih), (0., 1.)));

        let crossbar = Rect::new((mix(sbl, mid, 0.7), xh), (2. * mid, xh))
            .aligned(Alignment::Left)
            .stroked(stw)
            .transformed(Affine2D::translated((0., -stw * 0.9)));

        Self {
            hook,
            crossbar,
            offset: (-stw, 0.).into(),
        }
    }
}

impl IntoOutlines for FShape {
    type Outlines = [Arc<OutlineExpr>; 2];

    fn into_outlines(self) -> Self::Outlines {
        [self.hook, self.crossbar].map(move |it| it.transformed(Affine2D::translated(self.offset)))
    }
}
