use monoxide_script::prelude::*;

use crate::{
    InputContext,
    font::{
        dir::Dir,
        glyph::o::{OCapShape, OShape},
        shape::Slash,
    },
};

pub fn zero(cx: &InputContext) -> Glyph {
    let_settings! { { mid, cap, ovs, ovh, sbl, stw } = cx.settings(); }

    let o_cap_shape @ OCapShape {
        o_shape:
            OShape {
                center: Point2D { y, .. },
                radii: Point2D { y: ry, .. },
                ..
            },
    } = OCapShape::new((mid, cap / 2.), (mid - sbl, cap / 2.), ovs).with_ovh(ovh);

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
