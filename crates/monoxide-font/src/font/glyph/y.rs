use monoxide_script::prelude::*;

use crate::{
    InputContext,
    font::{
        math::mix,
        shape::{Slash, SlashAlignment},
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
