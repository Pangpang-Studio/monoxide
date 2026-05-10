use monoxide_script::prelude::*;

use crate::{InputContext, font::glyph::sym::SlashShape};

pub fn x(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, sbr, stw, xh } = cx.settings(); }

    let slash = SlashShape::new(sbl..sbr, (0.)..xh).with_aln(0.2);
    let backslash = slash.clone().back();

    Glyph::builder()
        .outlines(slash.stroked(stw))
        .outlines(backslash.stroked(stw))
        .build()
}
