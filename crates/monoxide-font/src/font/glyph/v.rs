use std::{ops::Range, sync::Arc};

use monoxide_script::prelude::*;

use crate::{
    InputContext,
    font::shape::{Slash, SlashAlignment},
};

pub fn v(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, sbr, stw, xh } = cx.settings(); }

    Glyph::builder()
        .outlines(VShape::new(sbl..sbr, 0.0..xh).stroked(stw))
        .build()
}

struct VShape {
    xr: Range<f64>,
    yr: Range<f64>,
}

impl VShape {
    fn new(xr: Range<f64>, yr: Range<f64>) -> Self {
        Self { xr, yr }
    }
}

impl IntoOutlines for VShape {
    type Outlines = [Arc<OutlineExpr>; 2];

    fn into_outlines(self) -> Self::Outlines {
        let Self { xr, yr } = self;
        let mid = xr.start.midpoint(xr.end);

        let aln = 0.2;
        let slash =
            Slash::new(mid..xr.end, yr.clone()).with_aln(SlashAlignment::new(0.5, 1. - aln));
        let backslash = Slash {
            xr: xr.start..mid,
            ..slash.clone()
        }
        .back();

        [slash.into_outline(), backslash.into_outline()]
    }
}
