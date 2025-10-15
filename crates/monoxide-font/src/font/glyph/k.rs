use monoxide_script::prelude::*;

use crate::font::{
    InputContext,
    dir::{Alignment, Dir},
    shape::Rect,
};

pub fn k(cx: &InputContext) -> Glyph {
    let_settings! {
        {
            sbl, sbr, xh, mih, stw,
            lower_left,
            lower_right,
            upper_left,
        } = cx.settings();
    }

    let k_mid_offset = Point2D::new(sbr - sbl, 0.) * 0.1;

    let pipe = Rect::new(lower_left, upper_left, stw)
        .aligned(Alignment::Left)
        .into_outline();

    let xh_right = Point2D::new(sbr, xh);
    let mid = Point2D::new(sbl + stw, mih) + k_mid_offset;

    let chevron = SpiroBuilder::open()
        .insts([
            corner!(lower_right)
                .aligned(Alignment::Right)
                .heading(Dir::U),
            corner!(mid).aligned(Alignment::Left),
            corner!(xh_right).aligned(Alignment::Right).heading(Dir::U),
        ])
        // TODO: Find out how the scaling factor is determined.
        .stroked(stw * 0.8);

    let bar = Rect::new(mid.with_x(sbl), mid + Point2D::unit_x() * stw, stw).into_outline();

    Glyph::builder().outlines([pipe, chevron, bar]).build()
}
