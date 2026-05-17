use monoxide_script::prelude::*;

use crate::{
    InputContext,
    font::{
        dir::Dir,
        glyph::o::{IOShape, OCapShape},
        shape::Slash,
    },
};

pub fn zero(cx: &InputContext) -> Glyph {
    let_settings! { { mid, cap, ovs, ovh, sbl, stw } = cx.settings(); }

    let o_cap_shape = OCapShape::new((mid, cap / 2.), (mid - sbl, cap / 2.), ovs).with_ovh(ovh);
    let o_shape = &o_cap_shape.o_shape;
    let Point2D { y, .. } = o_shape.center();
    let Point2D { y: ry, .. } = o_shape.radii();

    let end_curve_h = o_cap_shape.end_curve_h();

    let slash = Slash::new(
        (o_cap_shape.left() + stw)..(o_cap_shape.right() - stw),
        (y - ry + end_curve_h)..(y + ry - end_curve_h),
    )
    .with_heading(Dir::L);

    Glyph::builder()
        .outline(o_cap_shape.stroked(stw))
        .outline(slash.stroked(stw))
        .build()
}
