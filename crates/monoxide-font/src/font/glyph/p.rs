use std::{f64::consts::PI, sync::Arc};

use monoxide_script::prelude::*;

use crate::font::{
    InputContext,
    dir::Alignment,
    glyph::{
        d::DShape,
        o::{IOShape, OCapShape},
    },
    settings::FontParamSettings,
    shape::Rect,
};

pub fn p(cx: &InputContext) -> Glyph {
    let_settings! { { xh, mid, mih, dsc } = cx.settings; }
    Glyph::builder()
        .outlines(
            DShape::from_settings(&cx.settings)
                .with_height(xh - dsc)
                .transformed(Affine2D::rotated_around((mid, mih), PI)),
        )
        .build()
}

pub fn p_cap(cx: &InputContext) -> Glyph {
    Glyph::builder()
        .outlines(PCapShape::from_settings(&cx.settings))
        .build()
}

pub struct PCapShape {
    pub bowl: CapBowl,
    pub pipe: Rect,
}

impl PCapShape {
    pub const DEFAULT_BOWL_H_FACTOR: f64 = 0.575;

    pub fn from_settings(settings: &FontParamSettings) -> Self {
        let_settings! { { sbl, mid, sbr, stw, cap } = settings; }

        let bowl_h = cap * Self::DEFAULT_BOWL_H_FACTOR;
        let bowl = CapBowl::new((mid, cap - bowl_h / 2.), (mid - sbl, bowl_h / 2.));

        let pipe = Rect::new((sbl, 0.), (sbl, cap))
            .aligned(Alignment::Left)
            .stroked(stw);

        Self { bowl, pipe }
    }
}

impl IntoOutlines for PCapShape {
    type Outlines = [Arc<OutlineExpr>; 2];

    fn into_outlines(self) -> Self::Outlines {
        let pipe = self.pipe;
        let stw = pipe.width.unwrap_or_default();
        [self.bowl.stroked(stw), pipe.into_outline()]
    }
}

pub struct CapBowl {
    pub o_shape: OCapShape,
    pub mid_curve_h_factor: f64,
    pub end_curve_h_factor: f64,
}

impl CapBowl {
    pub const DEFAULT_END_CURVE_H_FACTOR: f64 = 1.2;
    pub const DEFAULT_MID_CURVE_H_FACTOR: f64 = 1.7;

    pub fn new(center: impl Into<Point2D>, radii: impl Into<Point2D>) -> Self {
        Self {
            o_shape: OCapShape::new(center, radii, 0.),
            mid_curve_h_factor: Self::DEFAULT_MID_CURVE_H_FACTOR,
            end_curve_h_factor: Self::DEFAULT_END_CURVE_H_FACTOR,
        }
    }

    pub fn with_mid_curve_h_factor(mut self, factor: impl Into<Option<f64>>) -> Self {
        self.mid_curve_h_factor = factor.into().unwrap_or(Self::DEFAULT_MID_CURVE_H_FACTOR);
        self
    }

    pub fn with_end_curve_h_factor(mut self, factor: impl Into<Option<f64>>) -> Self {
        self.end_curve_h_factor = factor.into().unwrap_or(Self::DEFAULT_END_CURVE_H_FACTOR);
        self
    }

    pub fn mid_curve_h(&self) -> f64 {
        self.o_shape.mid_curve_h() * self.mid_curve_h_factor
    }

    pub fn mid_curve_w(&self) -> f64 {
        self.o_shape.mid_curve_w()
    }

    pub fn end_curve_h(&self) -> f64 {
        self.o_shape.end_curve_h() * self.end_curve_h_factor
    }

    pub fn end_curve_w(&self) -> f64 {
        self.o_shape.radii().x / 3.
    }
}

impl IntoOutline for CapBowl {
    fn into_outline(self) -> Arc<OutlineExpr> {
        let o_shape = &self.o_shape;
        let Point2D { x, y } = o_shape.center();
        let Point2D { x: rx, y: ry } = o_shape.radii();

        let mid_curve_w = self.mid_curve_w();
        let mid_curve_h = self.mid_curve_h();
        let end_curve_w = self.end_curve_w();
        let end_curve_h = self.end_curve_h();

        let y_hi = y + ry;
        let y_lo = y - ry;

        SpiroBuilder::open()
            .insts([
                // Top arc
                flat!(x - rx, y_hi),
                curl!(x - end_curve_w, y_hi),
                g4!(x + mid_curve_w, y_hi - mid_curve_h).width(1.),
                // Right side
                flat!(x + rx, y_hi - end_curve_h),
                curl!(x + rx, y_lo + end_curve_h),
                // Bottom arc
                g4!(x + mid_curve_w, y_lo + mid_curve_h)
                    .aligned(Alignment::Left)
                    .width(1.),
                flat!(x - end_curve_w, y_lo),
                curl!(x - rx, y_lo),
            ])
            .into_outline()
    }
}
