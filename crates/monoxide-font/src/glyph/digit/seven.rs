use monoxide_script::prelude::*;

use crate::{
    InputContext,
    dir::Alignment,
    glyph::z::ZShape,
    prelude::*,
    shape::{Rect, Slash, SlashAlignment},
};

pub fn seven(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        sbl,
        sbr,
        stw,
        cap,
        mid,
        ovs,
        ..
    } = cx.settings().view();

    let bar = Rect::new((sbl, cap), (sbr, cap)).aligned(Alignment::Left);
    let slash = Slash::new(mid - stw..sbr, 0.0..cap - stw).with_aln(SlashAlignment::new(0.5, 1.));

    let serif_len = ZShape::new(sbl..sbr, 0.0..cap, ovs, stw).serif_len();
    let serif = Rect::new((sbl, cap - serif_len - stw), (sbl, cap)).aligned(Alignment::Left);

    Glyph::builder()
        .outline(serif.stroked(stw))
        .outline(bar.stroked(stw))
        .outline(slash.stroked(stw))
        .build()
}
