use std::{ops::Range, sync::Arc};

use monoxide_script::prelude::*;

use crate::{InputContext, dir::Dir, prelude::*};

pub fn caret(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        sbl,
        sbr,
        xh,
        cap,
        stw,
        ..
    } = cx.settings().view();

    Glyph::builder()
        .outline(Caret::new(sbl..sbr, xh - stw..cap, stw))
        .build()
}

pub fn less(cx: &InputContext) -> Glyph {
    angle_bracket(cx, true)
}

pub fn greater(cx: &InputContext) -> Glyph {
    angle_bracket(cx, false)
}

fn angle_bracket(cx: &InputContext, left: bool) -> Glyph {
    let FontParamSettingsView {
        sbl,
        mid,
        sbr,
        cap,
        stw,
        ..
    } = cx.settings().view();

    let gap = cap * 0.6;
    let mut xform = Affine2D::reflect((0., 0.), (1., 1.));
    if left {
        xform = xform.then_reflect((mid, 0.), (0., 1.));
    }
    Glyph::builder()
        .outline(
            Caret::new(cap.midpoint(-gap)..cap.midpoint(gap), sbl..sbr, stw).transformed(xform),
        )
        .build()
}

/// A caret heading up, delimited within the given x and y ranges.
#[derive(Clone, Debug)]
pub struct Caret {
    pub xr: Range<f64>,
    pub yr: Range<f64>,
    pub stw: f64,
}

impl Caret {
    pub fn new(xr: Range<f64>, yr: Range<f64>, stw: f64) -> Self {
        Self { xr, yr, stw }
    }
}

impl IntoOutline for Caret {
    fn into_outline(self) -> Arc<OutlineExpr> {
        let Range { start: x0, end: x1 } = self.xr;
        let Range { start: y0, end: y1 } = self.yr;

        let xm = x0.midpoint(x1);
        let slash = |x| {
            SpiroBuilder::open()
                .insts([g4!(x, y0), g4!(xm, y1).heading(Dir::U)])
                .stroked(self.stw)
        };
        slash(x0).or(slash(x1))
    }
}
