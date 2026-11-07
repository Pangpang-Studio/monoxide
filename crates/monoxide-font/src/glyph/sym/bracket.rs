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
pub fn curly_bracket_l(cx: &InputContext) -> Glyph {
    bracket(cx, CurlyBracketShape::new, 1.)
}

pub fn curly_bracket_r(cx: &InputContext) -> Glyph {
    bracket(cx, CurlyBracketShape::new, -1.)
}

struct CurlyBracketShape {
    pub xr: Range<f64>,
    pub yr: Range<f64>,
    pub stw: f64,
}

impl CurlyBracketShape {
    pub fn new(xr: Range<f64>, yr: Range<f64>, stw: f64) -> Self {
        Self { xr, yr, stw }
    }
}

impl IntoOutline for CurlyBracketShape {
    fn into_outline(self) -> Arc<OutlineExpr> {
        let Range { start: x0, end: x1 } = self.xr;
        let Range { start: y0, end: y1 } = self.yr;
        let stw = self.stw;

        let x10s = (x1 - x0).signum();
        let y10s = (y1 - y0).signum();

        let x0 = x0 - stw * x10s / 2.;
        let x = x0.midpoint(x1);
        let y = y0.midpoint(y1);
        let aln = if x10s == y10s {
            Alignment::Right
        } else {
            Alignment::Left
        };

        let curl = SpiroBuilder::open()
            .insts([
                g4!(x1, y1).heading(Dir::L),
                g4!(mix(x0, x1, 0.2), mix(y1, y, 1.)),
                g4!(mix(x0, x1, 0.6), mix(y1, y, 0.75)),
                g4!(x, mix(y1, y, 0.35)).aligned(aln),
                g4!(mix(x0, x1, 0.55), mix(y1, y, 0.15)),
                corner!(x0, y0.midpoint(y1))
                    .heading(Dir::L)
                    .aligned(Alignment::Middle),
            ])
            .stroked(stw);

        curl.clone()
            .or(curl.transformed(Affine2D::mirrored_along((x, y), (1., 0.))))
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
