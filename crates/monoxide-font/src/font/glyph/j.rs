use std::sync::Arc;

use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::{
    dir::Alignment,
    glyph::{i::dot, l::LShape, n::Hook},
    math::mix,
    settings::FontParamSettings,
    shape::{Rect, Ring},
};

pub fn j(cx: &InputContext) -> Glyph {
    Glyph::builder()
        .outlines(JShape::from_settings(&cx.settings))
        .build()
}

pub struct JShape {
    pub hook: Arc<OutlineExpr>,
    pub top_serif: Rect,
    pub dot: Ring,
    pub offset: Point2D,
}

impl JShape {
    pub const HOOK_TIP_HEADING: Point2D = Point2D::new(-1., -2.);

    pub fn from_settings(settings: &FontParamSettings) -> Self {
        let_settings! { { mid, mih, sbl, stw, xh } = settings; }

        let hook = Self::hook_raw(settings, None)
            .transformed(Affine2D::mirrored_along((0., xh / 2.), (1., 0.)));

        let top_serif = Rect::new(
            (mid, xh),
            (mid - LShape::DEFAULT_TOP_BAR_SCALE * (mih - sbl), xh),
        )
        .aligned(Alignment::Right)
        .stroked(stw);

        Self {
            hook,
            top_serif,
            dot: dot(settings),
            offset: (stw, 0.).into(),
        }
    }

    /// Returns a stroked `j` hook without transformation and the hook's reference height. The
    /// `y_hi` parameter can be used to override the highest point of the hook.
    pub fn hook_raw(
        settings: &FontParamSettings,
        y_hi: impl Into<Option<f64>>,
    ) -> Arc<OutlineExpr> {
        let_settings! { { cap, mid, mih, ovs, sbl, sbr, stw, xh } = settings; }

        let y = cap / 2.;
        let ry = mih;
        let y_lo = y_hi.into().map_or(0., |y_hi| y + ry - y_hi);
        let rx = (mid - sbl) * 0.9;

        // TODO: Consider using the boundary expression instead of the center-radii expression
        // here?
        Hook::new((mid, y - y_lo), (rx, ry), ovs)
            .with_hook_tip_heading(Self::HOOK_TIP_HEADING)
            .stroked(1.05 * stw)
            .transformed(Affine2D::translated((-rx + stw / 2., 0.)))
    }
}

impl IntoOutlines for JShape {
    fn into_outlines(self) -> impl Iterator<Item = Arc<OutlineExpr>> {
        [
            self.hook.into_outline(),
            self.top_serif.into_outline(),
            self.dot.into_outline(),
        ]
        .into_iter()
        .map(move |it| it.transformed(Affine2D::translated(self.offset)))
    }
}

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

        let hook = Hook::new((mid, cap - mih), (1.05 * (mid - sbl), mih), ovs)
            .with_hook_tip_heading(JShape::HOOK_TIP_HEADING)
            .stroked(1.1 * stw)
            .transformed(Affine2D::mirrored_along((0., cap / 2.), (1., 0.)));

        let pipe = Rect::new((mix(sbl, mid, 0.3), cap), (sbr, cap))
            .aligned(Alignment::Left)
            .stroked(stw);

        Self {
            hook,
            pipe,
            offset: (-stw / 2., 0.).into(),
        }
    }
}

impl IntoOutlines for JCapShape {
    fn into_outlines(self) -> impl Iterator<Item = Arc<OutlineExpr>> {
        [self.hook.into_outline(), self.pipe.into_outline()]
            .into_iter()
            .map(move |it| it.transformed(Affine2D::translated(self.offset)))
    }
}
