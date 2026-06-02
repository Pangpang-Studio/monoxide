use std::sync::Arc;

use monoxide_script::prelude::*;

use crate::{
    InputContext,
    dir::Alignment,
    glyph::{
        o::{IOShape, OShape},
        p::{CapBowl, PCapShape},
    },
    prelude::*,
    shape::Rect,
};

pub fn d(cx: &InputContext) -> Glyph {
    Glyph::builder()
        .outlines(DShape::from_settings(&cx.settings))
        .build()
}

pub fn d_cap(cx: &InputContext) -> Glyph {
    let settings = &cx.settings;
    let FontParamSettingsView { sbl, mid, cap, .. } = settings.view();

    let mut p_cap_shape = PCapShape::from_settings(settings);
    p_cap_shape.bowl = CapBowl::new((mid, cap / 2.), (mid - sbl, cap / 2.))
        .with_mid_curve_h_factor(1.2)
        .with_end_curve_h_factor(1.0);

    Glyph::builder().outlines(p_cap_shape).build()
}

pub struct DShape {
    bowl: Arc<OutlineExpr>,
    pipe: Rect,
}

impl DShape {
    pub fn from_settings(settings: &FontParamSettings) -> Self {
        let FontParamSettingsView {
            cap,
            mid,
            mih,
            ovs,
            sbl,
            sbr,
            stw,
            ..
        } = settings.view();

        let bowl = Bowl::new((mid - stw / 4., mih), (mid - sbl - stw / 4., mih), ovs)
            .stroked(stw)
            .into_outline();
        let pipe = Rect::new((sbr, 0.), (sbr, cap))
            .aligned(Alignment::Right)
            .stroked(stw);

        Self { bowl, pipe }
    }

    pub fn with_height(mut self, height: f64) -> Self {
        self.pipe.end.y = height;
        self
    }
}

impl IntoOutlines for DShape {
    type Outlines = [Arc<OutlineExpr>; 2];

    fn into_outlines(self) -> Self::Outlines {
        [self.bowl, self.pipe.into_outline()]
    }
}

pub struct Bowl {
    pub o_shape: OShape,
}

impl Bowl {
    pub fn new(center: impl Into<Point2D>, radii: impl Into<Point2D>, ovs: f64) -> Self {
        Self {
            o_shape: OShape::new(center, radii, ovs),
        }
    }

    pub fn mid_curve_h(&self) -> f64 {
        self.o_shape.mid_curve_h()
    }

    pub fn mid_curve_w(&self) -> f64 {
        self.o_shape.mid_curve_w()
    }

    pub fn end_curve_h(&self) -> f64 {
        self.o_shape.end_curve_h()
    }
}

impl IntoOutline for Bowl {
    fn into_outline(self) -> Arc<OutlineExpr> {
        let o_shape = &self.o_shape;
        let Point2D { x, y } = o_shape.center();
        let Point2D { x: rx, y: ry } = o_shape.radii();
        let ovs = o_shape.ovs();

        let mid_curve_w = self.mid_curve_w();
        let mid_curve_h = self.mid_curve_h();
        let end_curve_h = self.end_curve_h();

        let y_hi = y + ry;
        let y_lo = y - ry;

        SpiroBuilder::open()
            .insts([
                // Top arc
                g4!(x + rx, y_hi - mid_curve_h * 2.).width(0.4),
                g4!(x, y_hi + ovs).width(0.9),
                g4!(x - mid_curve_w, y_hi - mid_curve_h).width(1.),
                // Left side
                flat!(x - rx, y_hi - end_curve_h),
                curl!(x - rx, y_lo + end_curve_h),
                // Bottom arc
                g4!(x - mid_curve_w, y_lo + mid_curve_h)
                    .aligned(Alignment::Right)
                    .width(1.),
                g4!(x, y_lo - ovs).width(0.9),
                g4!(x + rx, y_lo + mid_curve_h * 2.).width(0.4),
            ])
            .into_outline()
    }
}
