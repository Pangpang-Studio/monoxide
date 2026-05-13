use monoxide_script::prelude::*;

use crate::{
    InputContext,
    font::shape::{Slash, SlashAlignment},
};

pub fn v(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, mid, sbr, stw, xh } = cx.settings(); }

    let aln = 0.2;
    let slash = Slash::new(mid..sbr, 0.0..xh).with_aln(SlashAlignment::new(0.5, 1. - aln));

    let backslash = Slash {
        x_range: sbl..mid,
        ..slash.clone()
    }
    .back();

    Glyph::builder()
        .outline(slash.stroked(stw))
        .outline(backslash.stroked(stw))
        .build()
}
