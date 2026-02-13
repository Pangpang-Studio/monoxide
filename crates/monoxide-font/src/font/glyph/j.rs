use std::sync::Arc;

use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::{
    dir::{Alignment, Dir},
    glyph::n::Hook,
    settings::FontParamSettings,
    shape::Rect,
};

pub fn j_cap(cx: &InputContext) -> Glyph {
    Glyph::builder()
        .outlines(JCapShape::from_settings(&cx.settings))
        .build()
}

pub struct JCapShape {
    pub hook: Arc<OutlineExpr>,
    pub pipe: Rect,
    pub offset: Point2D,
}

impl JCapShape {
    pub fn from_settings(settings: &FontParamSettings) -> Self {
        let_settings! { { cap, mid, mih, ovs, sbl, sbr, stw, xh } = settings; }

        let hook = Hook::new((mid, mih + cap - xh), (1.05 * (mid - sbl), mih), ovs)
            .with_hook_tip_heading(Dir::L)
            .stroked(1.1 * stw)
            .transformed(Affine2D::mirrored_along(
                (0., cap / 2.).into(),
                Point2D::unit_x(),
            ));

        let pipe = Rect::new((sbl.midpoint(mid), cap), (sbr, cap), stw).aligned(Alignment::Left);

        Self {
            hook,
            pipe,
            offset: (-stw / 2., 0.).into(),
        }
    }
}

impl IntoOutlines for JCapShape {
    fn into_outlines(self) -> impl Iterator<Item = std::sync::Arc<OutlineExpr>> {
        [self.hook.into_outline(), self.pipe.into_outline()]
            .into_iter()
            .map(move |it| it.transformed(Affine2D::translated(self.offset)))
    }
}
