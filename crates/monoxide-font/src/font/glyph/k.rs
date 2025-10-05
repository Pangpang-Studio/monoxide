use monoxide_script::prelude::*;

use crate::font::{
    InputContext,
    dir::{Alignment, Dir},
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

    let pipe = SpiroBuilder::open()
        .insts([
            corner!(lower_left).aligned(Alignment::Left).heading(Dir::U),
            corner!(upper_left).aligned(Alignment::Left).heading(Dir::U),
        ])
        .stroked(stw);

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
        .stroked(stw * 0.8);
    let bar = SpiroBuilder::open()
        .insts([
            corner!(mid.with_x(sbl)),
            corner!(mid + Point2D::unit_x() * stw),
        ])
        .stroked(stw);

    Glyph::builder().outlines([pipe, chevron, bar]).build()
}
