use monoxide_script::prelude::*;

use crate::{
    InputContext,
    font::{
        dir::{Alignment, Dir},
        glyph::o::OShape,
    },
};

pub fn zero(cx: &InputContext) -> Glyph {
    let_settings! { { mid, cap, ovs, ovh, sbl, stw } = cx.settings(); }

    let o_shape @ OShape {
        center: Point2D { y, .. },
        radii: Point2D { y: ry, .. },
        ..
    } = OShape::new((mid, cap / 2.), (mid - sbl, cap / 2.), ovs).with_ovh(ovh);

    let end_curve_h = o_shape.end_curve_h();

    let slash = SpiroBuilder::open().insts([
        g4!(o_shape.left() + stw, y - ry + end_curve_h)
            .heading(Dir::L)
            .aligned(Alignment::Left),
        g4!(o_shape.right() - stw, y + ry - end_curve_h)
            .heading(Dir::R)
            .aligned(Alignment::Right),
    ]);

    Glyph::builder()
        .outline(o_shape.stroked(stw))
        .outline(slash.stroked(stw))
        .build()
}
