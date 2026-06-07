use std::{ops::Range, sync::Arc};

use monoxide_script::prelude::*;

use crate::{InputContext, dir::Alignment, glyph::j::JShape, math::mix, prelude::*, shape::Rect};

pub fn f(cx: &InputContext) -> Glyph {
    Glyph::builder()
        .outlines(FShape::from_settings(&cx.settings))
        .build()
}

pub fn f_cap(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        stw, sbl, sbr, cap, ..
    } = cx.settings().view();

    Glyph::builder()
        .outlines(FCapShape::new(sbl + stw / 2.0..sbr, 0.0..cap).stroked(stw))
        .build()
}

pub struct FShape {
    pub hook: Arc<OutlineExpr>,
    pub crossbar: Arc<OutlineExpr>,
    pub offset: Point2D,
}

impl FShape {
    pub fn from_settings(settings: &FontParamSettings) -> Self {
        let FontParamSettingsView {
            mid,
            mih,
            sbl,
            stw,
            xh,
            cap,
            ..
        } = settings.view();

        let hook = JShape::hook_raw(settings, cap)
            .transformed(Affine2D::mirrored_along((mid, mih), (0., 1.)));

        let crossbar = Rect::new((mix(sbl, mid, 0.7), xh), (2. * mid, xh))
            .aligned(Alignment::Left)
            .stroked(stw)
            .transformed(Affine2D::translated((0., -stw * 0.9)));

        Self {
            hook,
            crossbar,
            offset: (-stw, 0.).into(),
        }
    }
}

impl IntoOutlines for FShape {
    type Outlines = [Arc<OutlineExpr>; 2];

    fn into_outlines(self) -> Self::Outlines {
        [self.hook, self.crossbar].map(move |it| it.transformed(Affine2D::translated(self.offset)))
    }
}

pub struct FCapShape {
    pub pipe: Rect,
    pub top: Rect,
    pub crossbar: Rect,
}

impl FCapShape {
    pub fn new(xr: Range<f64>, yr: Range<f64>) -> Self {
        let Range { start: x0, end: x1 } = xr;
        let Range { start: y0, end: y1 } = yr;
        let y = y0.midpoint(y1);

        Self {
            pipe: Rect::new((x0, y0), (x0, y1)).aligned(Alignment::Left),
            top: Rect::new((x0, y1), (x1, y1)).aligned(Alignment::Left),
            crossbar: Rect::new((x0, y), (mix(x1, x0, 0.85), y)),
        }
    }
}

impl IntoOutlines for FCapShape {
    type Outlines = [Arc<OutlineExpr>; 3];

    fn into_outlines(self) -> Self::Outlines {
        [self.pipe, self.top, self.crossbar].map(|it| it.into_outline())
    }
}
