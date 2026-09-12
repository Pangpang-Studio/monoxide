use std::sync::Arc;

use monoxide_script::{dsl::SpiroInst, prelude::*};

use crate::{
    InputContext,
    dir::{Alignment, Dir},
    glyph::{
        digit::eight::EightShape,
        o::{IOShape, OCapShape},
    },
    math::mix,
    prelude::*,
};

pub fn three(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        mid,
        ovs,
        sbl,
        stw,
        cap,
        ..
    } = cx.settings().view();

    Glyph::builder()
        .or_outlines(ThreeShape::new((mid, cap / 2.), (mid - sbl, cap / 2.), ovs).stroked(stw))
        .build()
}

pub struct ThreeShape<O = OCapShape> {
    pub o_shape: O,
}

impl ThreeShape {
    pub const TOP_BOWL_FACT: f64 = EightShape::TOP_CIRCLE_FACT;

    pub fn new(center: impl Into<Point2D>, radii: impl Into<Point2D>, ovs: f64) -> Self {
        Self {
            o_shape: OCapShape::new(center, radii, ovs),
        }
    }
}

impl IntoOutlines for ThreeShape {
    type Outlines = [Arc<OutlineExpr>; 2];

    fn into_outlines(self) -> Self::Outlines {
        let o_shape = self.o_shape;
        let Point2D { x, y } = o_shape.center();
        let Point2D { x: rx, y: ry } = o_shape.radii();
        let ovs = o_shape.ovs();
        let end_curve_h = mix(o_shape.end_curve_h(), ry, 0.1);
        let mid_curve_h = o_shape.mid_curve_h() / 6.;

        let top_fact = Self::TOP_BOWL_FACT;
        let top_ratio = top_fact / (1. - top_fact);
        let end_curve_h_top = end_curve_h * top_fact;
        let end_curve_h_bot = end_curve_h - end_curve_h_top;
        let mid_curve_h_top = mid_curve_h * top_fact;
        let mid_curve_h_bot = mid_curve_h - mid_curve_h_top;

        let left = o_shape.left();
        let right = o_shape.right();
        let left1 = mix(x - rx, x, top_ratio);
        let right1 = mix(x + rx, x, top_ratio);
        let y_hi = y + ry;
        let y_lo = y - ry;
        let y = mix(y_lo, y_hi, top_fact);

        let midpoint = |cp: SpiroInst| cp.width(0.8).aligned(Alignment::Middle);
        let x_midpoint_l = mix(left, x, 0.2);

        let top = SpiroBuilder::open().insts([
            // Top arc
            flat!(left1, y + end_curve_h_top)
                .width(1.)
                .aligned(Alignment::Left),
            curl!(left1, y_hi - end_curve_h_top),
            g4!(x, y_hi + ovs).heading(Dir::L),
            flat!(right1, y_hi - end_curve_h_top),
            curl!(right1, y + end_curve_h_top)
                .width(1.)
                .aligned(Alignment::Left),
            // Midpoint
            midpoint(flat!(x, y + mid_curve_h_top)),
            midpoint(curl!(x_midpoint_l, y)).heading(Dir::L),
        ]);

        let bot = SpiroBuilder::open().insts([
            // Midpoint
            midpoint(flat!(x_midpoint_l, y)).heading(Dir::R),
            midpoint(curl!(x, y - mid_curve_h_bot)),
            // Bottom arc
            flat!(right, y - end_curve_h_bot)
                .width(1.)
                .aligned(Alignment::Left),
            curl!(right, y_lo + end_curve_h_bot),
            g4!(x, y_lo - ovs).heading(Dir::L),
            flat!(left, y_lo + end_curve_h_bot),
            curl!(left, y - end_curve_h_bot)
                .width(1.)
                .aligned(Alignment::Left),
        ]);

        [top, bot].map(|s| s.into_outline())
    }
}
