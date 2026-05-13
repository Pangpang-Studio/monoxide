use monoxide_script::prelude::*;

use crate::{
    InputContext,
    font::shape::{Slash, SlashAlignment},
};

pub fn x(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, sbr, stw, xh } = cx.settings(); }

    let slash = Slash::new(sbl..sbr, (0.)..xh).with_aln(SlashAlignment::symm(0.2));
    let backslash = slash.clone().back();

    Glyph::builder()
        .outline(slash.stroked(stw))
        .outline(backslash.stroked(stw))
        .build()
}
