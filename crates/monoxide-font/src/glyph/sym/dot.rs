use monoxide_script::prelude::*;

use crate::{InputContext, prelude::*, shape::Ring};

pub fn dot(cx: &InputContext) -> Glyph {
    let FontParamSettingsView { mid, dtr, .. } = cx.settings.view();

    Glyph::builder()
        .outline(Ring::at((mid, dtr), (dtr, dtr)))
        .build()
}
