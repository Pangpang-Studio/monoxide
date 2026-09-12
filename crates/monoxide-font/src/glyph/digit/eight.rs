use std::sync::Arc;

use monoxide_script::prelude::*;

use crate::{
    InputContext,
    dir::{Alignment, Dir},
    glyph::{
        c::CShape,
        o::{IOShape, OCapShape},
    },
    math::mix,
    prelude::*,
};

pub fn eight(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        mid,
        ovs,
        sbl,
        stw,
        cap,
        ..
    } = cx.settings().view();

    Glyph::builder()
        .outline(EightShape::new((mid, cap / 2.), (mid - sbl, cap / 2.), ovs).stroked(stw))
        .build()
}

pub struct EightShape<O = OCapShape> {
    pub o_shape: O,
}

impl EightShape {
    pub const TOP_CIRCLE_FACT: f64 = 0.48;

    pub fn new(center: impl Into<Point2D>, radii: impl Into<Point2D>, ovs: f64) -> Self {
        Self {
            o_shape: OCapShape::new(center, radii, ovs),
        }
    }
}

impl IntoOutline for EightShape {
    fn into_outline(self) -> Arc<OutlineExpr> {
        let o_shape = self.o_shape;
        let Point2D { x, y } = o_shape.center();
        let Point2D { x: rx, y: ry } = o_shape.radii();
        let ovs = o_shape.ovs();

        let top_fact = Self::TOP_CIRCLE_FACT;
        let top_ratio = top_fact / (1. - top_fact);

        let left = o_shape.left();
        let right = o_shape.right();
        let left1 = mix(x - rx, x, top_ratio);
        let right1 = mix(x + rx, x, top_ratio);
        let y_hi = y + ry;
        let y_lo = y - ry;
        let y = mix(y_lo, y_hi, top_fact);

        let hook_h = CShape::from(o_shape).aperture_curve_h();
        let midpoint = g4!(x, y).width(0.85).aligned(Alignment::Middle);

        SpiroBuilder::closed()
            .insts([
                // Top arc
                g4!(right1, y_hi - hook_h)
                    .width(1.)
                    .heading(Dir::U)
                    .aligned(Alignment::Right),
                g4!(x, y_hi + ovs).heading(Dir::L).aligned(Alignment::Right),
                g4!(left1, y_hi - hook_h)
                    .width(1.)
                    .heading(Dir::D)
                    .aligned(Alignment::Right),
                // Midpoint
                midpoint.clone(),
                // Bottom arc
                g4!(right, y_lo + hook_h)
                    .width(1.)
                    .heading(Dir::D)
                    .aligned(Alignment::Left),
                g4!(x, y_lo - ovs).heading(Dir::L).aligned(Alignment::Left),
                g4!(left, y_lo + hook_h)
                    .width(1.)
                    .heading(Dir::U)
                    .aligned(Alignment::Left),
                // Midpoint
                midpoint,
            ])
            .into_outline()
    }
}
