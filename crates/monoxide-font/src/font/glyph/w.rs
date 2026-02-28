use std::{ops::Range, sync::Arc};

use monoxide_script::prelude::*;

use super::InputContext;
use crate::font::{
    dir::{Alignment, Dir},
    math::mix,
};

pub fn w(cx: &InputContext) -> Glyph {
    let_settings! { { sbl, mid, sbr, stw, xh } = cx.settings(); }

    let chevron = Chevron {
        x_range: sbl..mid,
        y_range: 0.0..xh,
        aln: 0.5,
        mih_scale: 0.75,
    };

    Glyph::builder()
        .outlines(
            chevron
                .clone()
                .stroked(stw)
                .transformed(Affine2D::mirrored_along((mid, 0.), (0., 1.))),
        )
        .outlines(chevron.stroked(stw))
        .build()
}

/// A left-biased chevron shape to be used by the left part of w.
#[derive(Clone)]
struct Chevron {
    pub x_range: Range<f64>,
    pub y_range: Range<f64>,
    pub aln: f64,
    /// The relative height of the middle peak compared to the overall height.
    pub mih_scale: f64,
}

impl IntoOutlines for Chevron {
    fn into_outlines(self) -> impl Iterator<Item = Arc<OutlineExpr>> {
        let Self {
            x_range: Range {
                start: sbl,
                end: sbr,
            },
            y_range: Range {
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

        [slash, backslash].into_iter().map(|it| it.into_outline())
    }
}
