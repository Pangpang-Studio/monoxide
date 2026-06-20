use std::{ops::Range, sync::Arc};

use monoxide_script::prelude::*;

use crate::{
    InputContext,
    dir::{Alignment, Dir},
    glyph::o::{IOShape, OShape},
    math::mix,
    prelude::*,
};

pub fn nine(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        sbl,
        sbr,
        stw,
        cap,
        mid,
        ovs,
        ..
    } = cx.settings().view();

    Glyph::builder()
        .outlines(NineShape::new(sbl..sbr, 0.0..cap, mid - stw / 2., ovs).stroked(stw))
        .build()
}

pub struct NineShape {
    pub circle: OShape,
    pub slash: SpiroBuilder,
}

impl NineShape {
    pub const DEFAULT_CIRCLE_H: f64 = 0.6;

    pub fn new(xr: Range<f64>, yr: Range<f64>, tip_x: f64, ovs: f64) -> Self {
        let Range {
            start: x_lo,
            end: x_hi,
        } = xr;
        let Range {
            start: bottom,
            end: y_hi,
        } = yr;

        let y_lo = mix(bottom, y_hi, Self::DEFAULT_CIRCLE_H);
        let x = x_lo.midpoint(x_hi);
        let y = y_lo.midpoint(y_hi);
        let rx = x - x_lo;
        let ry = y - y_lo;

        let circle = OShape::new((x, y), (rx, ry), ovs);

        let right = circle.right();
        let mid_curve_w = circle.mid_curve_w();
        let mid_curve_h = circle.mid_curve_h();
        let end_curve_h = circle.end_curve_h();

        let slash = SpiroBuilder::open().insts([
            // Slash
            flat!(tip_x, 0.).heading(Dir::D).aligned(Alignment::Middle),
            corner!(x + mid_curve_w, y_lo + mid_curve_h / 2.)
                .width(1.)
                .aligned(Alignment::Right),
            // Right side
            flat!(right, y_lo + end_curve_h).width(0.),
            curl!(right, y_hi - end_curve_h),
        ]);

        Self { circle, slash }
    }
}

impl IntoOutlines for NineShape {
    type Outlines = [Arc<OutlineExpr>; 2];

    fn into_outlines(self) -> Self::Outlines {
        [self.circle.into_outline(), self.slash.into_outline()]
    }
}
