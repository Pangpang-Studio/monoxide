use std::sync::Arc;

use monoxide_script::prelude::*;

use crate::{
    InputContext,
    dir::Alignment,
    glyph::{
        n::NShape,
        o::{IOShape, OCapShape},
    },
    prelude::*,
};

pub fn u(cx: &InputContext) -> Glyph {
    let FontParamSettingsView { mid, mih, .. } = cx.settings().view();

    Glyph::builder()
        .outlines(NShape::from_settings(cx.settings()).transformed(
            Affine2D::mirrored_along((mid, 0.), (0., 1.)).mirror_along((0., mih), (1., 0.)),
        ))
        .build()
}

pub fn u_cap(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        mid,
        cap,
        ovs,
        ovh,
        sbl,
        stw,
        ..
    } = cx.settings().view();

    Glyph::builder()
        .outline(
            UCapShape::new((mid, cap / 2.), (mid - sbl, cap / 2.), ovs)
                .with_ovh(ovh)
                .stroked(stw),
        )
        .build()
}
pub struct UCapShape {
    pub o_shape: OCapShape,
}

impl UCapShape {
    pub fn new(center: impl Into<Point2D>, radii: impl Into<Point2D>, ovs: f64) -> Self {
        Self {
            o_shape: OCapShape::new(center, radii, ovs),
        }
    }

    pub fn with_ovh(mut self, ovh: impl Into<Option<f64>>) -> Self {
        self.o_shape = self.o_shape.with_ovh(ovh);
        self
    }
}

impl IntoOutline for UCapShape {
    fn into_outline(self) -> Arc<OutlineExpr> {
        let o_shape = self.o_shape;

        let Point2D { x, y } = o_shape.center();
        let Point2D { y: ry, .. } = o_shape.radii();
        let ovs = o_shape.ovs();

        let mid_curve_w = o_shape.mid_curve_w();
        let mid_curve_h = o_shape.mid_curve_h();
        let end_curve_h = o_shape.end_curve_h();

        let left = o_shape.left();
        let right = o_shape.right();
        let y_hi = y + ry;
        let y_lo = y - ry;

        SpiroBuilder::open()
            .insts([
                // Left side
                flat!(left, y_hi),
                curl!(left, y_lo + end_curve_h),
                // Bottom arc
                g4!(x - mid_curve_w, y_lo + mid_curve_h).aligned(Alignment::Right),
                g4!(x, y_lo - ovs),
                g4!(x + mid_curve_w, y_lo + mid_curve_h),
                // Right side
                flat!(right, y_lo + end_curve_h),
                curl!(right, y_hi),
            ])
            .into_outline()
    }
}
