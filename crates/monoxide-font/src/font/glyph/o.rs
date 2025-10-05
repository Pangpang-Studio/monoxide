use std::sync::Arc;

use monoxide_script::prelude::*;

use super::InputContext;

pub fn o(cx: &InputContext) -> Glyph {
    let_settings! { { mid, mih, ovs, sbl, stw } = cx.settings(); }

    let hstw = stw / 2.;
    Glyph::builder()
        .outline(OShape::new((mid, mih), (mid - sbl - hstw, mih - hstw), ovs).stroked(stw))
        .build()
}

pub struct OShape {
    pub center: Point2D,
    pub radii: Point2D,
    pub ovs: f64,
}

impl OShape {
    pub fn new(center: impl Into<Point2D>, radii: impl Into<Point2D>, ovs: f64) -> Self {
        Self {
            center: center.into(),
            radii: radii.into(),
            ovs,
        }
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
        } = self;

        let mid_curve_w = self.mid_curve_w();
        let mid_curve_h = self.mid_curve_h();
        let end_curve_h = self.end_curve_h();

        let y_hi = y + ry;
        let y_lo = y - ry;

        SpiroBuilder::closed()
            .insts([
                // Bottom arc
                g4!(x - mid_curve_w, y_lo + mid_curve_h),
                g4!(x, y_lo - ovs),
                g4!(x + mid_curve_w, y_lo + mid_curve_h),
                // Right side
                flat!(x + rx, y_lo + end_curve_h),
                curl!(x + rx, y_hi - end_curve_h),
                // Top arc
                g4!(x + mid_curve_w, y_hi - mid_curve_h),
                g4!(x, y_hi + ovs),
                g4!(x - mid_curve_w, y_hi - mid_curve_h),
                // Left side
                flat!(x - rx, y_hi - end_curve_h),
                curl!(x - rx, y_lo + end_curve_h),
            ])
            .into_outline()
    }
}
