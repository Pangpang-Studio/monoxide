use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::{dir::Alignment, glyph::o::OShape, settings::FontParamSettings, shape::Rect};

pub fn d(cx: &InputContext) -> Glyph {
    Glyph::builder().outlines(DShape::new(&cx.settings)).build()
}

pub struct DShape<'a> {
    pub settings: &'a FontParamSettings,
    pub height: Option<f64>,
}

impl<'a> DShape<'a> {
    pub fn new(settings: &'a FontParamSettings) -> Self {
        Self::with_height(settings, None)
    }

    pub fn with_height(settings: &'a FontParamSettings, height: impl Into<Option<f64>>) -> Self {
        Self {
            settings,
            height: height.into(),
        }
    }
}

impl IntoOutlines for DShape<'_> {
    fn into_outlines(self) -> impl Iterator<Item = std::sync::Arc<OutlineExpr>> {
        let_settings! { { cap, mid, mih, ovs, sbl, sbr, stw } = self.settings; }

        let height = self.height.unwrap_or(cap);
        let bowl = OShape::new((mid, mih), (mid - sbl, mih), ovs)
            .stroked(stw)
            .into_outline();
        let pipe = Rect::new((sbr, 0.), (sbr, height), stw)
            .aligned(Alignment::Right)
            .into_outline();
        [bowl, pipe].into_iter()
    }
}
