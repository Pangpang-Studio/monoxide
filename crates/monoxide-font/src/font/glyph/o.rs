use std::sync::Arc;

use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::dir::Alignment;

pub fn o(cx: &InputContext) -> Glyph {
    let_settings! { { mid, mih, ovs, sbl, stw } = cx.settings(); }

    Glyph::builder()
        .outline(OShape::new((mid, mih), (mid - sbl, mih), ovs).stroked(stw))
        .build()
}

pub fn o_cap(cx: &InputContext) -> Glyph {
    let_settings! { { mid, cap, ovs, ovh, sbl, stw } = cx.settings(); }

    Glyph::builder()
        .outline(
            OShape::new((mid, cap / 2.), (mid - sbl, cap / 2.), ovs)
                .with_ovh(ovh)
                .stroked(stw),
        )
        .build()
}

pub struct OShape {
    pub center: Point2D,
    pub radii: Point2D,
    pub ovs: f64,
    pub ovh: f64,
}

impl OShape {
    pub fn new(center: impl Into<Point2D>, radii: impl Into<Point2D>, ovs: f64) -> Self {
        Self {
            center: center.into(),
            radii: radii.into(),
            ovs,
            ovh: Default::default(),
        }
    }

    pub fn with_ovh(mut self, ovh: impl Into<Option<f64>>) -> Self {
        self.ovh = ovh.into().unwrap_or_default();
        self
    }

    pub const fn mid_curve_w(&self) -> f64 {
        0.85 * self.radii.x
    }

    pub const fn mid_curve_h(&self) -> f64 {
        (5. / 16.) * self.radii.y
    }

    pub const fn end_curve_h(&self) -> f64 {
        (15. / 16.) * self.radii.y
    }
}

impl IntoOutline for OShape {
    fn into_outline(self) -> Arc<OutlineExpr> {
        let Self {
            center: Point2D { x, y },
            radii: Point2D { x: rx, y: ry },
            ovs,
            ovh,
        } = self;

        let mid_curve_w = self.mid_curve_w();
        let mid_curve_h = self.mid_curve_h();
        let end_curve_h = self.end_curve_h();

        let left = x - rx - ovh;
        let right = x + rx + ovh;
        let y_hi = y + ry;
        let y_lo = y - ry;

        SpiroBuilder::closed()
            .insts([
                // Top arc
                g4!(x + mid_curve_w, y_hi - mid_curve_h),
                g4!(x, y_hi + ovs),
                g4!(x - mid_curve_w, y_hi - mid_curve_h),
                // Left side
                flat!(left, y_hi - end_curve_h),
                curl!(left, y_lo + end_curve_h),
                // Bottom arc
                g4!(x - mid_curve_w, y_lo + mid_curve_h).aligned(Alignment::Right),
                g4!(x, y_lo - ovs),
                g4!(x + mid_curve_w, y_lo + mid_curve_h),
                // Right side
                flat!(right, y_lo + end_curve_h),
                curl!(right, y_hi - end_curve_h),
            ])
            .into_outline()
    }
}
