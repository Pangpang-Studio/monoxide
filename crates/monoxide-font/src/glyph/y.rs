use monoxide_script::prelude::*;

use crate::{
    InputContext,
    dir::{Alignment, Dir},
    math::mix,
    prelude::*,
    shape::{Rect, Slash, SlashAlignment},
};

pub fn y(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        sbl,
        mid,
        sbr,
        stw,
        xh,
        dsc,
        ..
    } = cx.settings().view();

    let aln = SlashAlignment::new(0.5, 0.8);

    let slash = Slash::new(mix(sbr, mid, dsc / xh)..sbr, dsc..xh).with_aln(SlashAlignment {
        bot: mix(aln.top, aln.bot, dsc / xh),
        ..aln
    });

    let backslash = Slash::new(sbl..mid, (0.)..xh).with_aln(aln).back();

    Glyph::builder()
        .or_outlines([slash.stroked(stw), backslash.stroked(stw)])
        .build()
}

pub fn y_cap(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        sbl,
        sbr,
        mid,
        cap,
        stw,
        ..
    } = cx.settings().view();

    let y_mih = cap / 2.;
    let stem = Rect::new((mid, y_mih), (mid, 0.));

    let aln = 0.2;
    let chevron = SpiroBuilder::open().insts([
        corner!(sbl, cap).aligned(1. - aln).heading(Dir::U),
        corner!(mid, y_mih).aligned(Alignment::Middle),
        corner!(sbr, cap).aligned(1. - aln).heading(Dir::U),
    ]);

    Glyph::builder()
        .or_outlines([chevron.stroked(stw), stem.stroked(stw).into_outline()])
        .build()
}
