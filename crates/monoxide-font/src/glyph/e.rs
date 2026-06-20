use std::{ops::Range, sync::Arc};

use monoxide_script::prelude::*;

use crate::{
    InputContext,
    dir::{Alignment, Dir},
    glyph::{c::CShape, f::FCapShape, o::IOShape},
    prelude::*,
    shape::Rect,
};

pub fn e(cx: &InputContext) -> Glyph {
    Glyph::builder()
        .outlines(EShape::from_settings(cx.settings()))
        .build()
}

pub fn e_cap(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        stw, sbl, sbr, cap, ..
    } = cx.settings().view();

    Glyph::builder()
        .outlines(ECapShape::new(sbl + stw / 2.0..sbr, 0.0..cap).stroked(stw))
        .build()
}

struct EShape {
    pub bowl: Arc<OutlineExpr>,
    pub bar: Rect,
}

impl EShape {
    pub fn from_settings(settings: &FontParamSettings) -> Self {
        let FontParamSettingsView {
            mid,
            mih,
            ovs,
            sbl,
            sbr,
            stw,
            ..
        } = settings.view();

        let bowl = Bowl::new((mid, mih), (mid - sbl, mih), ovs).stroked(stw);
        let bar = Rect::new((sbl, mih), (sbr, mih)).stroked(stw);

        Self { bowl, bar }
    }
}

impl IntoOutlines for EShape {
    type Outlines = [Arc<OutlineExpr>; 2];

    fn into_outlines(self) -> Self::Outlines {
        [self.bowl.into_outline(), self.bar.into_outline()]
    }
}

struct Bowl {
    pub c_shape: CShape,
}

impl Bowl {
    pub fn new(center: impl Into<Point2D>, radii: impl Into<Point2D>, ovs: f64) -> Self {
        Self {
            c_shape: CShape::new(center, radii, ovs),
        }
    }

    pub fn mid_curve_h(&self) -> f64 {
        self.c_shape.mid_curve_h()
    }

    pub fn mid_curve_w(&self) -> f64 {
        self.c_shape.mid_curve_w()
    }

    pub fn end_curve_h(&self) -> f64 {
        self.c_shape.end_curve_h()
    }

    pub fn aperture_curve_h_lo(&self) -> f64 {
        self.c_shape.aperture_curve_h_lo()
    }
}

impl IntoOutline for Bowl {
    fn into_outline(self) -> Arc<OutlineExpr> {
        let o_shape = &self.c_shape.o_shape;
        let Point2D { x, y } = o_shape.center();
        let Point2D { x: rx, y: ry } = o_shape.radii();
        let ovs = o_shape.ovs();

        let mid_curve_w = self.mid_curve_w();
        let mid_curve_h = self.mid_curve_h();
        let end_curve_h = self.end_curve_h();

        let y_hi = y + ry;
        let y_lo = y - ry;

        let rx1 = 0.9 * rx;

        SpiroBuilder::open()
            .insts([
                // Right side (upper)
                flat!(x + rx, y).heading(Dir::D),
                curl!(x + rx, y_hi - end_curve_h),
                // Top arc
                g4!(x + rx1, y_hi - mid_curve_h),
                g4!(x, y_hi + ovs),
                g4!(x - mid_curve_w, y_hi - mid_curve_h),
                // Left side
                flat!(x - rx, y_hi - end_curve_h),
                curl!(x - rx, y_lo + end_curve_h),
                // Bottom arc
                g4!(x - mid_curve_w, y_lo + mid_curve_h).aligned(Alignment::Right),
                g4!(x, y_lo - ovs),
                // One control point omitted.
                // Right side (lower)
                flat!(x + rx, y_lo + self.aperture_curve_h_lo()),
            ])
            .into_outline()
    }
}

pub struct ECapShape {
    pub f_shape: FCapShape,
}

impl ECapShape {
    pub fn new(xr: Range<f64>, yr: Range<f64>) -> Self {
        Self {
            f_shape: FCapShape::new(xr, yr),
        }
    }
}

impl IntoOutlines for ECapShape {
    type Outlines = [Arc<OutlineExpr>; 4];

    fn into_outlines(self) -> Self::Outlines {
        let top = &self.f_shape.top;
        let x0 = top.start.x;
        let x1 = top.end.x;

        let bot = Rect::new((x0, 0.), (x1, 0.))
            .aligned(Alignment::Right)
            .into_outline();

        let [pipe, top, crossbar] = self.f_shape.into_outlines();
        [pipe, top, crossbar, bot]
    }
}
