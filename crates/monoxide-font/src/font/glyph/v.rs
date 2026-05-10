use monoxide_script::prelude::*;

use crate::{
    InputContext,
    font::glyph::sym::{SlashAlignment, SlashShape},
};

pub fn v(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, mid, sbr, stw, xh } = cx.settings(); }

    let aln = 0.2;
    let slash = SlashShape::new(mid..sbr, 0.0..xh).with_aln(SlashAlignment::new(0.5, 1. - aln));

    let backslash = SlashShape {
        x_range: sbl..mid,
        ..slash.clone()
    }
    .back();

    Glyph::builder()
        .outlines(slash.stroked(stw))
        .outlines(backslash.stroked(stw))
        .build()
}
