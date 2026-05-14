use std::{ops::Range, sync::Arc};

use monoxide_script::prelude::*;

use crate::{
    InputContext,
    font::shape::{Slash, SlashAlignment},
};

pub fn x(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, sbr, stw, xh } = cx.settings(); }

    Glyph::builder()
        .outlines(XShape::new(sbl..sbr, 0.0..xh).stroked(stw))
        .build()
}

pub fn x_cap(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, sbr, stw, cap } = cx.settings(); }

    Glyph::builder()
        .outlines(XShape::new(sbl..sbr, 0.0..cap).stroked(stw))
        .build()
}

struct XShape {
    xr: Range<f64>,
    yr: Range<f64>,
}

impl XShape {
    fn new(xr: Range<f64>, yr: Range<f64>) -> Self {
        Self { xr, yr }
    }
}

impl IntoOutlines for XShape {
    type Outlines = [Arc<OutlineExpr>; 2];

    fn into_outlines(self) -> Self::Outlines {
        let slash = Slash::new(self.xr, self.yr).with_aln(SlashAlignment::symm(0.2));
        let backslash = slash.clone().back();

        [slash.into_outline(), backslash.into_outline()]
    }
}
