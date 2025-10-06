use std::sync::Arc;

use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::{dir::Alignment, glyph::o::OShape, settings::FontParamSettings, shape::Rect};

pub fn d(cx: &InputContext) -> Glyph {
    Glyph::builder()
        .outlines(DShape::from_settings(&cx.settings))
        .build()
}

pub struct DShape {
    bowl: Arc<OutlineExpr>,
    pipe: Rect,
}

impl DShape {
    pub fn from_settings(settings: &FontParamSettings) -> Self {
        let_settings! { { cap, mid, mih, ovs, sbl, sbr, stw } = settings; }

        // TODO: Stroke should narrow at lower join of the bowl.
        let bowl = OShape::new((mid, mih), (mid - sbl, mih), ovs)
            .stroked(stw)
            .into_outline();
        let pipe = Rect::new((sbr, 0.), (sbr, cap), stw).aligned(Alignment::Right);

        Self { bowl, pipe }
    }

    pub fn with_height(mut self, height: f64) -> Self {
        self.pipe.end.y = height;
        self
    }
}

impl IntoOutlines for DShape {
    fn into_outlines(self) -> impl Iterator<Item = std::sync::Arc<OutlineExpr>> {
        [self.bowl, self.pipe.into_outline()].into_iter()
    }
}
