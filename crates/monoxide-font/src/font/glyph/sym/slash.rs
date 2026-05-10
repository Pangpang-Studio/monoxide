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
    pub aln: SlashAlignment,
}

#[derive(Clone, Copy, Debug)]
pub struct SlashAlignment {
    pub bot: f64,
    pub top: f64,
}

impl SlashAlignment {
    pub const fn new(bot: f64, top: f64) -> Self {
        Self { bot, top }
    }

    /// Returns a symmetric alignment where the bottom and top points are
    /// aligned to `bot` and `1 - bot` respectively.
    pub const fn symm(bot: f64) -> Self {
        Self::new(bot, 1. - bot)
    }

    pub const fn back(mut self) -> Self {
        self.bot = 1. - self.bot;
        self.top = 1. - self.top;
        self
    }
}

impl Default for SlashAlignment {
    fn default() -> Self {
        Self::symm(0.)
    }
}

impl SlashShape {
    pub fn new(x_range: Range<f64>, y_range: Range<f64>) -> Self {
        Self {
            x_range,
            y_range,
            aln: SlashAlignment::default(),
        }
    }

    pub fn with_aln(mut self, aln: impl Into<Option<SlashAlignment>>) -> Self {
        self.aln = aln.into().unwrap_or_default();
        self
    }

    pub fn back(mut self) -> Self {
        let Range { start: x0, end: x1 } = self.x_range;
        self.x_range = x1..x0;
        self.aln = self.aln.back();
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
                g4!(left, bot).heading(Dir::D).aligned(aln.bot),
                g4!(right, top).heading(Dir::U).aligned(aln.top),
            ])
            .into_outline()]
    }
}
