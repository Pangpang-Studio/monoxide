use monoxide_script::prelude::*;

use super::{InputContext, d::DShape, o::OCapShape};
use crate::font::shape::{Slash, SlashAlignment};

pub fn q(cx: &InputContext) -> Glyph {
    let_settings! { { xh, mih, dsc } = cx.settings; }
    Glyph::builder()
        .outlines(
            DShape::from_settings(&cx.settings)
                .with_height(xh - dsc)
                .transformed(Affine2D::mirrored_along((0., mih), (1., 0.))),
        )
        .build()
}

pub fn q_cap(cx: &InputContext) -> Glyph {
    let_settings! { { mid, cap, ovs, ovh, sbl, sbr, stw, dsc } = cx.settings(); }

    let o_cap = OCapShape::new((mid, cap / 2.), (mid - sbl, cap / 2.), ovs).with_ovh(ovh);
    let tail = Slash::new(mid..mid.midpoint(sbr), (dsc * 0.75)..0.)
        .with_aln(SlashAlignment::symm(0.5))
        .back();

    Glyph::builder()
        .outline(o_cap.stroked(stw))
        .outline(tail.stroked(stw))
        .build()
}
