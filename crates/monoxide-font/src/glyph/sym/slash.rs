use monoxide_script::prelude::*;

use crate::{InputContext, prelude::*, shape::Slash};

pub fn slash(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        sbl,
        sbr,
        stw,
        cap,
        dsc,
        ..
    } = cx.settings().view();

    let ovs = -dsc / 2.;
    let slash = Slash::new(sbl..sbr, (-ovs)..(cap + ovs));

    Glyph::builder().outline(slash.stroked(stw * 1.05)).build()
}

pub fn backslash(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        sbl,
        sbr,
        stw,
        cap,
        dsc,
        ..
    } = cx.settings().view();

    let ovs = -dsc / 2.;
    let slash = Slash::new(sbl..sbr, (-ovs)..(cap + ovs)).back();

    Glyph::builder().outline(slash.stroked(stw)).build()
}
