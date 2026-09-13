use std::sync::Arc;

use monoxide_script::prelude::*;

use crate::{
    InputContext,
    dir::{Alignment, Dir},
    glyph::{
        digit::three::ThreeShape,
        o::{IOShape, OCapShape},
    },
    math::mix,
    prelude::*,
};

pub fn five(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        mid,
        ovs,
        sbl,
        stw,
        cap,
        ..
    } = cx.settings().view();

    Glyph::builder()
        .or_outlines(FiveShape::new(
            (mid, cap / 2.),
            (mid - sbl, cap / 2.),
            ovs,
            stw,
        ))
        .build()
}

pub struct FiveShape<O = OCapShape> {
    pub o_shape: O,
    pub stw: f64,
}

impl FiveShape {
    pub const FLAG_FACT: f64 = 0.42;

    pub fn new(center: impl Into<Point2D>, radii: impl Into<Point2D>, ovs: f64, stw: f64) -> Self {
        Self {
            o_shape: OCapShape::new(center, radii, ovs),
            stw,
        }
    }
}

impl IntoOutlines for FiveShape {
    type Outlines = [Arc<OutlineExpr>; 2];

    fn into_outlines(self) -> Self::Outlines {
        let stw = self.stw;
        let o_shape = self.o_shape;
        let Point2D { x, y } = o_shape.center();
        let Point2D { y: ry, .. } = o_shape.radii();
        let ovs = o_shape.ovs();
        let end_curve_h = mix(o_shape.end_curve_h(), ry, 0.1);

        let top_fact = Self::FLAG_FACT;
        let bot_fact = 1. - top_fact;
        let end_curve_h_bot = end_curve_h * bot_fact;

        let left = o_shape.left();
        let right = o_shape.right();
        let right1 = right - stw / 3.;
        let y_hi = y + ry;
        let y_lo = y - ry;

        let (y3, end_curve_h_bot3) = {
            let top_fact = ThreeShape::TOP_BOWL_FACT;
            let end_curve_h_bot = end_curve_h * (1. - top_fact);
            let y = mix(y_lo, y_hi, top_fact);
            (y, end_curve_h_bot)
        };

        let y = mix(y_lo, y_hi, top_fact);
        let y_corner = y - end_curve_h_bot * 0.55;

        let flag = SpiroBuilder::open().insts([
            g4!(right1, y_hi).aligned(Alignment::Right),
            corner!(left, y_hi).aligned(Alignment::Right),
            corner!(left, y_corner),
        ]);

        let bowl = SpiroBuilder::open().insts([
            curl!(left + stw, y_corner).aligned(Alignment::Right),
            // Top arc
            g4!(x, y).heading(Dir::R).aligned(Alignment::Middle),
            // Bottom arc
            flat!(right, y - end_curve_h_bot)
                .width(1.)
                .aligned(Alignment::Left),
            curl!(right, y_lo + end_curve_h_bot),
            g4!(x, y_lo - ovs)
                .heading(Dir::L)
                .width(1.)
                .aligned(Alignment::Left),
            flat!(left, y_lo + end_curve_h_bot3).aligned(0.05),
            curl!(left, y3 - end_curve_h_bot3).width(1.025),
        ]);

        [flag, bowl].map(|s| s.stroked(stw))
    }
}
