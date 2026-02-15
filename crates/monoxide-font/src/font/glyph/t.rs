use std::{f64::consts::PI, sync::Arc};

use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::{
    dir::Alignment, glyph::j::JShape, math::mix, settings::FontParamSettings, shape::Rect,
};

pub fn t(cx: &InputContext) -> Glyph {
    Glyph::builder()
        .outlines(TShape::from_settings(&cx.settings))
        .build()
}

pub struct TShape {
    pub hook: Arc<OutlineExpr>,
    pub crossbar: Rect,
    pub offset: Point2D,
}

impl TShape {
    pub fn from_settings(settings: &FontParamSettings) -> Self {
        let_settings! { { mid, mih, sbl, sbr, stw, xh, cap } = settings; }

        let hook = JShape::hook_raw(settings, cap)
            .transformed(Affine2D::rotated_around((mid, cap / 2.), PI));

        let crossbar = Rect::new((mix(sbl, mid, 0.8), xh), (2. * mid, xh))
            .aligned(Alignment::Left)
            .stroked(stw);

        Self {
            hook,
            crossbar,
            offset: (-stw, 0.).into(),
        }
    }
}

impl IntoOutlines for TShape {
    fn into_outlines(self) -> impl Iterator<Item = Arc<OutlineExpr>> {
        [self.hook.into_outline(), self.crossbar.into_outline()]
            .into_iter()
            .map(move |it| it.transformed(Affine2D::translated(self.offset)))
    }
}
