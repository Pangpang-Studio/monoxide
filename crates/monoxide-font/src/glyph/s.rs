use std::sync::Arc;

use monoxide_script::prelude::*;

use crate::{
    InputContext,
    dir::{Alignment, Dir},
    glyph::{
        c::CShape,
        o::{IOShape, OCapShape, OShape},
    },
    prelude::*,
};

pub fn s(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        mid,
        mih,
        ovs,
        sbl,
        stw,
        ..
    } = cx.settings().view();

    Glyph::builder()
        .outline(HookedSShape::new((mid, mih), (mid - sbl, mih), ovs).stroked(stw))
        .build()
}

pub fn s_cap(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        mid,
        ovs,
        sbl,
        stw,
        cap,
        ..
    } = cx.settings().view();

    Glyph::builder()
        .outline(SShape::new((mid, cap / 2.), (mid - sbl, cap / 2.), ovs).stroked(stw))
        .build()
}

struct HookedSShape {
    pub o_shape: OShape,
}

impl HookedSShape {
    pub fn new(center: impl Into<Point2D>, radii: impl Into<Point2D>, ovs: f64) -> Self {
        Self {
            o_shape: OShape::new(center, radii, ovs),
        }
    }
}

impl IntoOutline for HookedSShape {
    fn into_outline(self) -> Arc<OutlineExpr> {
        let o_shape = self.o_shape;
        let Point2D { x, y } = o_shape.center();
        let Point2D { x: rx, y: ry } = o_shape.radii();
        let ovs = o_shape.ovs();

        let mid_curve_w = o_shape.mid_curve_w();
        let mid_curve_h = o_shape.mid_curve_h();

        let c_shape = CShape { o_shape };
        let aperture_curve_h_hi = c_shape.aperture_curve_h_hi();

        let y_hi = y + ry;
        let y_lo = y - ry;

        SpiroBuilder::open()
            .insts([
                // Top arc
                g4!(x + rx * 0.9, y_hi - aperture_curve_h_hi * 0.75),
                corner!(x + rx * 0.9, y_hi - mid_curve_h * 0.75),
                g4!(x, y_hi + ovs).width(1.).heading(Dir::L),
                g4!(x - mid_curve_w, y_hi - mid_curve_h * 0.8)
                    .width(0.98)
                    .aligned(Alignment::Right),
                // Midpoint
                g4!(x, y).width(0.95).aligned(Alignment::Middle),
                // Bottom arc
                g4!(x + mid_curve_w, y_lo + mid_curve_h * 0.8)
                    .width(0.98)
                    .aligned(Alignment::Left),
                g4!(x, y_lo - ovs).width(1.).heading(Dir::L),
                g4!(x - mid_curve_w, y_lo + mid_curve_h * 0.75),
                g4!(x - rx, y_lo + mid_curve_h * 1.75).heading(Dir::U),
            ])
            .into_outline()
    }
}

struct SShape<O = OCapShape> {
    pub o_shape: O,
}

impl SShape {
    pub fn new(center: impl Into<Point2D>, radii: impl Into<Point2D>, ovs: f64) -> Self {
        Self {
            o_shape: OCapShape::new(center, radii, ovs),
        }
    }
}

impl IntoOutline for SShape<OCapShape> {
    fn into_outline(self) -> Arc<OutlineExpr> {
        let o_shape = self.o_shape;
        let Point2D { x, y } = o_shape.center();
        let Point2D { x: rx, y: ry } = o_shape.radii();
        let ovs = o_shape.ovs();

        let left = o_shape.left();
        let right = o_shape.right();
        let left1 = x - rx;
        let right1 = x + rx;
        let y_hi = y + ry;
        let y_lo = y - ry;

        let hook_h = CShape::from(o_shape).aperture_curve_h();
        let tip_aln = 0.9;

        SpiroBuilder::open()
            .insts([
                // Top arc
                g4!(right, y_hi - hook_h).heading(Dir::U).aligned(tip_aln),
                g4!(x, y_hi + ovs).heading(Dir::L).aligned(Alignment::Right),
                g4!(left1, y_hi - hook_h)
                    .heading(Dir::D)
                    .width(1.)
                    .aligned(Alignment::Right),
                // Midpoint
                g4!(x, y).width(1.1).aligned(Alignment::Middle),
                // Bottom arc
                g4!(right1, y_lo + hook_h)
                    .heading(Dir::D)
                    .width(1.)
                    .aligned(Alignment::Left),
                g4!(x, y_lo - ovs).heading(Dir::L).aligned(Alignment::Left),
                g4!(left, y_lo + hook_h)
                    .heading(Dir::U)
                    .aligned(1. - tip_aln),
            ])
            .into_outline()
    }
}
