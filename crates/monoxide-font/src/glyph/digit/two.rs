use std::sync::Arc;

use monoxide_script::prelude::*;

use crate::{
    InputContext,
    dir::{Alignment, Dir},
    glyph::{
        c::CShape,
        o::{IOShape, OCapShape},
    },
    prelude::*,
    shape::Rect,
};

pub fn two(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        sbl,
        sbr,
        stw,
        cap,
        mid,
        ovs,
        ..
    } = cx.settings().view();

    let hook = Hook::new((mid, cap / 2.), (mid - sbl, cap / 2.), ovs, stw);
    let bar = Rect::new((sbl, 0.), (sbr, 0.))
        .aligned(Alignment::Right)
        .stroked(stw);

    Glyph::builder()
        .or_outlines([hook.into_outline(), bar.into_outline()])
        .build()
}

struct Hook {
    o_shape: OCapShape,
    stw: f64,
}

impl Hook {
    pub fn new(center: impl Into<Point2D>, radii: impl Into<Point2D>, ovs: f64, stw: f64) -> Self {
        Self {
            o_shape: OCapShape::new(center, radii, ovs),
            stw,
        }
    }
}

impl IntoOutline for Hook {
    fn into_outline(self) -> Arc<OutlineExpr> {
        let stw = self.stw;
        let o_shape = self.o_shape;
        let Point2D { x, y } = o_shape.center();
        let Point2D { x: rx, y: ry } = o_shape.radii();
        let ovs = o_shape.ovs();

        let left = o_shape.left();
        let left1 = x - rx;
        let right1 = x + rx;
        let y_hi = y + ry;
        let y_lo = y - ry;

        let hook_h = CShape::from(o_shape).aperture_curve_h();

        SpiroBuilder::open()
            .insts([
                // Top arc
                g4!(left, y_hi - hook_h)
                    .width(1.1)
                    .heading(Dir::U)
                    .aligned(Alignment::Left),
                g4!(x, y_hi + ovs).heading(Dir::L),
                g4!(right1, y_hi - hook_h)
                    .width(1.)
                    .heading(Dir::D)
                    .aligned(Alignment::Left),
                // Midpoint
                flat!(x, y_lo + y * 0.7)
                    .width(0.9)
                    .aligned(Alignment::Middle),
                // Bottom arc
                corner!(left1, y_lo + stw)
                    .width(0.85)
                    .heading(Dir::D)
                    .aligned(Alignment::Right),
            ])
            .stroked(stw)
    }
}
