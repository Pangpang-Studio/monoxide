use std::sync::Arc;

use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::{
    dir::Alignment, glyph::j::JShape, math::mix, settings::FontParamSettings, shape::Rect,
};

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
        let_settings! { { mid, mih, sbl, sbr, stw, xh, cap } = settings; }

        let hook = {
            // TODO: The height of the `hook_raw` is (cap + xh) / 2. Combined with this gives the
            // full height. Consider doing the actual calculation.
            let base_y = (cap - xh) / 2.;
            JShape::hook_raw(settings, -base_y)
                .transformed(Affine2D::mirrored_along((mid, mih), (0., 1.)).translate((0., base_y)))
        };

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
    fn into_outlines(self) -> impl Iterator<Item = Arc<OutlineExpr>> {
        [self.hook, self.crossbar]
            .into_iter()
            .map(move |it| it.transformed(Affine2D::translated(self.offset)))
    }
}
