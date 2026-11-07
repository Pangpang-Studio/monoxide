use std::{ops::Range, sync::Arc};

use monoxide_script::prelude::*;

use crate::{
    InputContext,
    dir::{Alignment, Dir},
    math::mix,
    prelude::*,
};

pub fn square_bracket_l(cx: &InputContext) -> Glyph {
    bracket(
        cx,
        |xr, yr, stw| SquareBracketShape::new(xr, yr).stroked(stw),
        1.,
    )
}

pub fn square_bracket_r(cx: &InputContext) -> Glyph {
    bracket(
        cx,
        |xr, yr, stw| SquareBracketShape::new(xr, yr).stroked(stw),
        -1.,
    )
}

struct SquareBracketShape {
    pub xr: Range<f64>,
    pub yr: Range<f64>,
}

impl SquareBracketShape {
    pub fn new(xr: Range<f64>, yr: Range<f64>) -> Self {
        Self { xr, yr }
    }
}

impl IntoOutline for SquareBracketShape {
    fn into_outline(self) -> Arc<OutlineExpr> {
        let Range { start: x0, end: x1 } = self.xr;
        let Range { start: y0, end: y1 } = self.yr;

        let aln = if x0.total_cmp(&x1) == y0.total_cmp(&y1) {
            Alignment::Right
        } else {
            Alignment::Left
        };
        SpiroBuilder::open()
            .insts([
                corner!(x1, y1).aligned(aln),
                corner!(x0, y1),
                corner!(x0, y0),
                corner!(x1, y0),
            ])
            .into_outline()
    }
}

pub fn round_bracket_l(cx: &InputContext) -> Glyph {
    bracket(
        cx,
        |xr, yr, stw| RoundBracketShape::new(xr, yr).stroked(stw),
        1.,
    )
}

pub fn round_bracket_r(cx: &InputContext) -> Glyph {
    bracket(
        cx,
        |xr, yr, stw| RoundBracketShape::new(xr, yr).stroked(stw),
        -1.,
    )
}

struct RoundBracketShape {
    pub xr: Range<f64>,
    pub yr: Range<f64>,
}

impl RoundBracketShape {
    pub fn new(xr: Range<f64>, yr: Range<f64>) -> Self {
        Self { xr, yr }
    }
}

impl IntoOutline for RoundBracketShape {
    fn into_outline(self) -> Arc<OutlineExpr> {
        let Range { start: x0, end: x1 } = self.xr;
        let Range { start: y0, end: y1 } = self.yr;

        let aln = if x0.total_cmp(&x1) == y0.total_cmp(&y1) {
            Alignment::Right
        } else {
            Alignment::Left
        };
        SpiroBuilder::open()
            .insts([
                g4!(x1, y1).aligned(aln),
                g4!(x0, y0.midpoint(y1)),
                g4!(x1, y0),
            ])
            .into_outline()
    }
}

fn bracket<S: IntoOutline>(
    cx: &InputContext,
    shape: impl FnOnce(Range<f64>, Range<f64>, f64) -> S,
    fact: f64,
) -> Glyph {
    let FontParamSettingsView {
        cap,
        dsc,
        sbl,
        mid,
        stw,
        ..
    } = cx.settings().view();

    let dx = fact * (mid - sbl) * 0.75;
    let dy = -dsc / 2.;
    Glyph::builder()
        .outline(shape(mid - dx..mid + dx, -dy..cap + dy, stw))
        .build()
}
