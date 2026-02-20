use std::sync::Arc;

use monoxide_script::{g2, prelude::*};

use crate::font::{
    InputContext,
    dir::{Alignment, Dir},
    glyph::{j::JShape, n::Hook},
    math::mix,
    settings::FontParamSettings,
    shape::Rect,
};

pub fn a(cx: &InputContext) -> Glyph {
    Glyph::builder()
        .outlines(AShape::from_settings(&cx.settings))
        .build()
}

pub struct AShape {
    pub hook: Arc<OutlineExpr>,
    pub bowl: Arc<OutlineExpr>,
}

impl AShape {
    pub fn from_settings(settings: &FontParamSettings) -> Self {
        let_settings! { { mid, mih, ovs, sbl, sbr, stw, xh } = settings; }

        let hook = Hook::new((mid, mih), (mid - sbl, mih), ovs)
            .with_hook_tip_heading(JShape::HOOK_TIP_HEADING)
            .stroked(stw * 1.05);

        let bowl = SpiroBuilder::open()
            .insts([
                g4!(sbr, mih * 1.2).heading(Dir::R).width(0.9),
                g4!(mid, mih * 1.2).width(0.95),
                g4!(sbl, mih / 2.).width(1.05),
                g2!(mid, 0.).width(0.9),
                g4!(sbr, mih * 0.8).aligned(Alignment::Right).width(0.1),
            ])
            .stroked(stw);

        Self { hook, bowl }
    }
}

impl IntoOutlines for AShape {
    fn into_outlines(self) -> impl Iterator<Item = Arc<OutlineExpr>> {
        [self.hook.into_outline(), self.bowl.into_outline()].into_iter()
    }
}

pub fn a_cap(cx: &InputContext) -> Glyph {
    let_settings! {
        {
            lower_left,
            lower_right,
            upper_mid,
            lower_mid,
            stw,
        } = cx.settings();
    }

    let bar_height = 0.65;

    let left = SpiroBuilder::open()
        .insts([
            g4!(lower_left).heading(Dir::D),
            g4!(upper_mid).heading(Dir::U).aligned(Alignment::Middle),
        ])
        .stroked(stw);

    let right = left
        .clone()
        .transformed(Affine2D::mirrored_along(lower_mid, Point2D::unit_y()));

    let bar = Rect::new(
        mix(lower_left, upper_mid, bar_height),
        mix(lower_right, upper_mid, bar_height),
    )
    .stroked(stw)
    .into_outline();

    Glyph::builder().outlines([left, bar, right]).build()
}
