use std::{ops::Range, sync::Arc};

use monoxide_script::prelude::*;

use crate::{InputContext, font::dir::Dir};

pub fn slash(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, sbr, stw, cap, dsc } = cx.settings(); }

    let ovs = -dsc / 2.;
    let slash = SlashShape::new(sbl..sbr, (-ovs)..(cap + ovs));

    Glyph::builder().outlines(slash.stroked(stw * 1.05)).build()
}

pub fn backslash(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, sbr, stw, cap, dsc } = cx.settings(); }

    let ovs = -dsc / 2.;
    let slash = SlashShape::new(sbl..sbr, (-ovs)..(cap + ovs)).back();

    Glyph::builder().outlines(slash.stroked(stw)).build()
}

#[derive(Clone, Debug)]
pub struct SlashShape {
    pub x_range: Range<f64>,
    pub y_range: Range<f64>,
    pub aln: f64,
}

impl SlashShape {
    pub const DEFAULT_ALN: f64 = 0.;

    pub fn new(x_range: Range<f64>, y_range: Range<f64>) -> Self {
        Self {
            x_range,
            y_range,
            aln: Self::DEFAULT_ALN,
        }
    }

    pub fn with_aln(mut self, aln: impl Into<Option<f64>>) -> Self {
        self.aln = aln.into().unwrap_or(Self::DEFAULT_ALN);
        self
    }

    pub fn back(mut self) -> Self {
        let Range { start: x0, end: x1 } = self.x_range;
        self.x_range = x1..x0;
        self.aln = 1. - self.aln;
        self
    }
}

impl IntoOutlines for SlashShape {
    type Outlines = [Arc<OutlineExpr>; 1];

    fn into_outlines(self) -> Self::Outlines {
        let Self {
            x_range: Range {
                start: left,
                end: right,
            },
            y_range: Range {
                start: bot,
                end: top,
            },
            aln,
        } = self;

        [SpiroBuilder::open()
            .insts([
                g4!(left, bot).heading(Dir::D).aligned(aln),
                g4!(right, top).heading(Dir::U).aligned(1. - aln),
            ])
            .into_outline()]
    }
}
