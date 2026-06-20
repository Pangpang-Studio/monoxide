use monoxide_script::prelude::*;

use crate::{
    InputContext,
    dir::Alignment,
    prelude::*,
    shape::{Rect, Slash},
};

pub fn four(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        sbl,
        sbr,
        stw,
        cap,
        mid,
        xh,
        ..
    } = cx.settings().view();

    let x = mid + stw;
    let pipe = Rect::new((x, 0.), (x, xh.midpoint(cap / 2.)));

    let y = xh * 0.3;
    let bar = Rect::new((sbl, y), (sbr, y)).aligned(Alignment::Right);
    let slash = Slash::new(sbl..x, y + stw..cap);

    Glyph::builder()
        .outline(pipe.stroked(stw))
        .outline(bar.stroked(stw))
        .outline(slash.stroked(stw * 0.9))
        .build()
}
