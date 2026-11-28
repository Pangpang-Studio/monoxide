use std::sync::Arc;

use monoxide_script::prelude::*;

use crate::{
    InputContext,
    dir::{Alignment, Dir},
    glyph::{j::JShape, n::Hook},
    math::mix,
    prelude::*,
    shape::Rect,
};

pub fn a(cx: &InputContext) -> Glyph {
    Glyph::builder()
        .or_outlines(AShape::from_settings(&cx.settings))
        .build()
}

pub fn a_cap(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        sbl,
        sbr,
        mid,
        cap,
        stw,
        ..
    } = cx.settings().view();

    let bar_height = 0.65;

    let foot = |x| corner!(x, 0.).heading(Dir::U).width(1.).aligned(0.4);
    let legs = SpiroBuilder::open().insts([
        foot(sbl),
        corner!(mid, cap).heading(Dir::U).width(0.9).aligned(0.5),
        foot(sbr),
    ]);

    let bar = Rect::new(
        mix(Point2D::new(sbl, 0.), (mid, cap).into(), bar_height),
        mix(Point2D::new(sbr, 0.), (mid, cap).into(), bar_height),
    );

    Glyph::builder()
        .or_outlines([legs.stroked(stw), bar.stroked(stw).into_outline()])
        .build()
}

pub struct AShape {
    pub hook: Arc<OutlineExpr>,
    pub bowl: Arc<OutlineExpr>,
}

impl AShape {
    pub fn from_settings(settings: &FontParamSettings) -> Self {
        let FontParamSettingsView {
            mid,
            mih,
            ovs,
            sbl,
            sbr,
            stw,
            ..
        } = settings.view();

        let hook = Hook::new((mid, mih), (mid - sbl, mih), ovs)
            .with_hook_tip_heading(JShape::HOOK_TIP_HEADING)
            .stroked(stw * 1.05);

        let bowl = SpiroBuilder::open()
            .insts([
                flat!(sbr, mih * 1.2).heading(Dir::L).width(0.9),
                curl!(mix(sbr, mid, 0.4), mih * 1.2).width(0.9),
                g4!(sbl, mih / 2.).width(1.),
                g4!(mid, 0.).width(0.9),
                g4!(sbr, mih * 0.8).aligned(Alignment::Right).width(0.1),
            ])
            .stroked(stw);

        Self { hook, bowl }
    }
}

impl IntoOutlines for AShape {
    type Outlines = [Arc<OutlineExpr>; 2];

    fn into_outlines(self) -> Self::Outlines {
        [self.hook.into_outline(), self.bowl.into_outline()]
    }
}
