use std::{ops::Range, sync::Arc};

use monoxide_script::prelude::*;

use crate::{
    InputContext,
    font::{
        dir::{Alignment, Dir},
        math::mix,
    },
};

pub fn w(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, mid, sbr, stw, xh } = cx.settings(); }

    let chevron = Chevron::new(sbl..mid, 0.0..xh, 0.5, 0.75);
    Glyph::builder()
        .outlines(WShape::from(chevron).stroked(stw))
        .build()
}

pub fn w_cap(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, mid, sbr, stw, xh, cap } = cx.settings(); }

    let chevron = Chevron::new(sbl..mid, 0.0..cap, 0.5, xh / cap);
    Glyph::builder()
        .outlines(WShape::from(chevron).stroked(stw))
        .build()
}

pub struct WShape {
    pub chevron: Chevron,
    pub stw: Option<f64>,
}

impl WShape {
    pub fn stroked(mut self, stw: impl Into<Option<f64>>) -> Self {
        self.stw = stw.into();
        self
    }
}

impl From<Chevron> for WShape {
    fn from(chevron: Chevron) -> Self {
        Self { chevron, stw: None }
    }
}

impl IntoOutlines for WShape {
    type Outlines = [Arc<OutlineExpr>; 4];

    fn into_outlines(self) -> Self::Outlines {
        let mid = self.chevron.xr.end;

        let mut chevron = self.chevron.into_outlines();
        if let Some(stw) = self.stw {
            chevron = chevron.map(|it| it.stroked(stw));
        }

        let [c0, c1] = chevron;
        let xform = Affine2D::mirrored_along((mid, 0.), (0., 1.));
        [
            c0.clone().transformed(xform),
            c1.clone().transformed(xform),
            c0,
            c1,
        ]
    }
}

/// A left-biased chevron shape to be used by the left part of w.
#[derive(Clone)]
pub struct Chevron {
    pub xr: Range<f64>,
    pub yr: Range<f64>,
    pub aln: f64,

    /// The relative height of the middle peak compared to the overall height.
    pub mih_scale: f64,

    /// The relative width of the middle peak compared to the overall width.
    ///
    /// If `None` is provided, it will fall back on a default value.
    pub mid_scale: Option<f64>,

    pub bot_width_scale: Option<f64>,
}

impl Chevron {
    pub const DEFAULT_MID_SCALE: f64 = 0.4;
    pub const DEFAULT_BOT_WIDTH_SCALE: f64 = 0.8;

    pub fn new(xr: Range<f64>, yr: Range<f64>, aln: f64, mih_scale: f64) -> Self {
        Self {
            xr,
            yr,
            aln,
            mih_scale,
            mid_scale: None,
            bot_width_scale: None,
        }
    }

    pub fn with_mid_scale(mut self, mid_scale: impl Into<Option<f64>>) -> Self {
        self.mid_scale = mid_scale.into();
        self
    }

    pub fn with_bot_width_scale(mut self, bot_width_scale: impl Into<Option<f64>>) -> Self {
        self.bot_width_scale = bot_width_scale.into();
        self
    }
}

impl IntoOutlines for Chevron {
    type Outlines = [Arc<OutlineExpr>; 2];

    fn into_outlines(self) -> Self::Outlines {
        let Self {
            xr: Range {
                start: sbl,
                end: sbr,
            },
            yr: Range {
                start: bse,
                end: xh,
            },
            aln,
            mih_scale,
            mid_scale,
            bot_width_scale,
        } = self;

        let mid = mix(sbr, sbl, mid_scale.unwrap_or(Self::DEFAULT_MID_SCALE));
        let mih = mix(xh, bse, mih_scale);

        let bot_width_scale = bot_width_scale.unwrap_or(Self::DEFAULT_BOT_WIDTH_SCALE);
        let bot = g4!(mid, bse)
            .heading(Dir::D)
            .aligned(aln)
            .width(bot_width_scale);

        let slash = SpiroBuilder::open().insts([
            bot.clone(),
            g4!(sbr, mih)
                .heading(Dir::U)
                .aligned(Alignment::Middle)
                .width(Self::DEFAULT_BOT_WIDTH_SCALE),
        ]);

        let backslash =
            SpiroBuilder::open().insts([bot, g4!(sbl, xh).heading(Dir::U).aligned(aln).width(1.1)]);

        [slash, backslash].map(|it| it.into_outline())
    }
}
