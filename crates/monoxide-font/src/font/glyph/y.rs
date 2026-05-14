use monoxide_script::prelude::*;

use crate::{
    InputContext,
    font::{
        dir::{Alignment, Dir},
        math::mix,
        shape::{Rect, Slash, SlashAlignment},
    },
};

pub fn y(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, mid, sbr, stw, xh, dsc } = cx.settings(); }

    let aln = SlashAlignment::new(0.5, 0.8);

    let slash = Slash::new(mix(sbr, mid, dsc / xh)..sbr, dsc..xh).with_aln(SlashAlignment {
        bot: mix(aln.top, aln.bot, dsc / xh),
        ..aln
    });

    let backslash = Slash::new(sbl..mid, (0.)..xh).with_aln(aln).back();

    Glyph::builder()
        .outline(slash.stroked(stw))
        .outline(backslash.stroked(stw))
        .build()
}

pub fn y_cap(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, sbr, mid, xh, cap, stw } = cx.settings(); }

    let y_mih = cap / 2.;
    let stem = Rect::new((mid, y_mih), (mid, 0.));

    let aln = 0.2;
    let chevron = SpiroBuilder::open().insts([
        corner!(sbl, cap).aligned(1. - aln).heading(Dir::U),
        corner!(mid, y_mih).aligned(Alignment::Middle),
        corner!(sbr, cap).aligned(1. - aln).heading(Dir::U),
    ]);

    Glyph::builder()
        .outline(chevron.stroked(stw))
        .outline(stem.stroked(stw))
        .build()
}
