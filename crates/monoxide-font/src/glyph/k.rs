use monoxide_script::prelude::*;

use crate::{
    InputContext,
    dir::{Alignment, Dir},
    prelude::*,
    shape::Rect,
};

pub fn k(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        sbl,
        sbr,
        xh,
        mih,
        stw,
        cap,
        lower_right,
        ..
    } = cx.settings().view();

    let k_mid_offset = Point2D::new(sbr - sbl, 0.) * 0.1;

    let pipe = Rect::new((sbl + stw / 4., 0.), (sbl + stw / 4., cap))
        .aligned(Alignment::Left)
        .stroked(stw)
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

    let bar = Rect::new(mid.with_x(sbl + stw / 4.), mid + (stw, 0.).into())
        .stroked(0.9 * stw)
        .into_outline();

    Glyph::builder().outlines([pipe, chevron, bar]).build()
}
