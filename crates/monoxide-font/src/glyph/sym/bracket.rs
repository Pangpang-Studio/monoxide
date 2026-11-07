use std::{ops::Range, sync::Arc};

use monoxide_script::prelude::*;

use crate::{InputContext, dir::Alignment, prelude::*};

pub fn square_bracket_l(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        cap,
        dsc,
        sbl,
        mid,
        stw,
        ..
    } = cx.settings().view();

    let dx = (mid - sbl) * 0.75;
    let dy = -dsc / 2.;
    Glyph::builder()
        .outline(SquareBracketShape::new(mid - dx..mid + dx, -dy..cap + dy).stroked(stw))
        .build()
}

pub fn square_bracket_r(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        cap,
        dsc,
        sbl,
        mid,
        stw,
        ..
    } = cx.settings().view();

    let dx = (mid - sbl) * -0.75;
    let dy = -dsc / 2.;
    Glyph::builder()
        .outline(SquareBracketShape::new(mid - dx..mid + dx, -dy..cap + dy).stroked(stw))
        .build()
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
