use monoxide_script::prelude::*;

use crate::{
    InputContext,
    glyph::{
        d::DShape,
        p::{CapBowl, PCapShape},
    },
    prelude::*,
};

pub fn b(cx: &InputContext) -> Glyph {
    Glyph::builder()
        .or_outlines(
            DShape::from_settings(&cx.settings).transformed(Affine2D::mirrored_along(
                cx.settings.lower_mid(),
                Point2D::unit_y(),
            )),
        )
        .build()
}

pub fn b_cap(cx: &InputContext) -> Glyph {
    let settings = &cx.settings;
    let FontParamSettingsView {
        sbl, mid, stw, cap, ..
    } = settings.view();

    let bowl = {
        let p_bowl_h_factor = PCapShape::DEFAULT_BOWL_H_FACTOR;
        let bowl_h_factor = 1. - p_bowl_h_factor;
        let bowl_h = cap * bowl_h_factor + stw;
        let mid = mid - stw / 6.;
        CapBowl::new((mid, cap - bowl_h / 2.), (mid - sbl, bowl_h / 2.))
    };

    // NOTE: We have to merge the two bowls first to satisfy the single-bezier
    // requirement of the boolean operation.
    let mut outlines = Vec::with_capacity(3);
    outlines.push(bowl.stroked(stw));

    let p_cap_shape = PCapShape::from_settings(settings);
    outlines.extend(p_cap_shape.transformed(Affine2D::mirrored_along((0., cap / 2.), (1., 0.))));

    Glyph::builder().or_outlines(outlines).build()
}
