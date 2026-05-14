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

    let chevron = Chevron {
        xr: sbl..mid,
        yr: 0.0..xh,
        aln: 0.5,
        mih_scale: 0.75,
    };

    Glyph::builder()
        .outlines(WShape::from(chevron).stroked(stw))
        .build()
}

pub fn w_cap(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, mid, sbr, stw, xh, cap } = cx.settings(); }

    let chevron = Chevron {
        xr: sbl..mid,
        yr: 0.0..cap,
        aln: 0.5,
        mih_scale: xh / cap,
    };

    Glyph::builder()
        .outlines(WShape::from(chevron).stroked(stw))
        .build()
}

struct WShape {
    pub chevron: Chevron,
    pub stw: Option<f64>,
}

impl WShape {
    fn stroked(mut self, stw: impl Into<Option<f64>>) -> Self {
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
struct Chevron {
    pub xr: Range<f64>,
    pub yr: Range<f64>,
    pub aln: f64,
    /// The relative height of the middle peak compared to the overall height.
    pub mih_scale: f64,
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
        } = self;

        let mid = mix(sbr, sbl, 0.4);
        let mih = mix(xh, bse, mih_scale);

        let bot = g4!(mid, bse)
            .heading(Dir::D)
            .aligned(Alignment::Middle)
            .width(0.8);

        let slash = SpiroBuilder::open().insts([bot.clone(), g4!(sbr, mih).heading(Dir::U)]);

        let backslash =
            SpiroBuilder::open().insts([bot, g4!(sbl, xh).heading(Dir::U).aligned(aln).width(1.1)]);

        [slash, backslash].map(|it| it.into_outline())
    }
}
