use monoxide_script::prelude::*;

use crate::{
    InputContext,
    font::{
        glyph::sym::{SlashAlignment, SlashShape},
        math::mix,
    },
};

pub fn y(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, mid, sbr, stw, xh, dsc } = cx.settings(); }

    let aln = SlashAlignment::new(0.5, 0.8);

    let slash = SlashShape::new(mix(sbr, mid, dsc / xh)..sbr, dsc..xh).with_aln(SlashAlignment {
        bot: mix(aln.top, aln.bot, dsc / xh),
        ..aln
    });

    let backslash = SlashShape::new(sbl..mid, (0.)..xh).with_aln(aln).back();

    Glyph::builder()
        .outlines(slash.stroked(stw))
        .outlines(backslash.stroked(stw))
        .build()
}
