use std::sync::Arc;

use monoxide_script::prelude::*;

use crate::{
    InputContext,
    dir::{Alignment, Dir},
    glyph::o::{IOShape, OShape},
    math::mix,
    prelude::*,
    shape::Rect,
};

pub fn m(cx: &InputContext) -> Glyph {
    Glyph::builder()
        .or_outlines(MShape::from_settings(&cx.settings))
        .build()
}

pub fn m_cap(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        sbl,
        sbr,
        mid,
        stw,
        xh,
        cap,
        ..
    } = cx.settings().view();
    let left = sbl + stw * 0.3;
    let right = sbr - stw * 0.3;
    let foot = |x| corner!(x, 0.).width(1.1).aligned(0.5);
    let shoulder = |x| corner!(x, cap).heading(Dir::U).width(1.).aligned(0.53);
    let stem = SpiroBuilder::open().insts([
        foot(left),
        shoulder(left),
        corner!(mid, cap - xh)
            .heading(Dir::U)
            .width(0.8)
            .aligned(0.5),
        shoulder(right),
        foot(right),
    ]);

    Glyph::builder().outline(stem.stroked(stw)).build()
}

struct MShape {
    pub hooks: [Arc<OutlineExpr>; 2],
    pub pipe: Rect,
}

impl MShape {
    pub fn from_settings(settings: &FontParamSettings) -> Self {
        let settings = FontParamSettings {
            side_bearing: settings.side_bearing / 1.5,
            ..*settings
        };
        let FontParamSettingsView {
            mid,
            mih,
            ovs,
            sbl,
            sbr,
            stw,
            xh,
            ..
        } = settings.view();

        let hook = Hook::new((mix(mid, sbr, 0.5), mih), ((mid - sbl) / 2., mih), ovs).stroked(stw);
        let hooks = [
            hook.clone(),
            hook.transformed(Affine2D::translate((sbl - mid + stw / 2., 0.))),
        ];
        let pipe = Rect::new((sbl, 0.), (sbl, xh))
            .aligned(Alignment::Left)
            .stroked(stw);

        Self { hooks, pipe }
    }
}

impl IntoOutlines for MShape {
    type Outlines = [Arc<OutlineExpr>; 3];

    fn into_outlines(self) -> Self::Outlines {
        let [h, h1] = self.hooks.map(|it| it.into_outline());
        [h, h1, self.pipe.into_outline()]
    }
}

struct Hook {
    pub o_shape: OShape,
}

impl Hook {
    pub fn new(center: impl Into<Point2D>, radii: impl Into<Point2D>, ovs: f64) -> Self {
        Self {
            o_shape: OShape::new(center, radii, ovs),
        }
    }
}

impl IntoOutline for Hook {
    fn into_outline(self) -> Arc<OutlineExpr> {
        let o_shape = &self.o_shape;
        let Point2D { x, y } = o_shape.center();
        let Point2D { x: rx, y: ry } = o_shape.radii();
        let ovs = o_shape.ovs();

        let mid_curve_w = o_shape.mid_curve_w();
        let mid_curve_h = o_shape.mid_curve_h();

        let y_hi = y + ry;

        SpiroBuilder::open()
            .insts([
                // Right side
                flat!(x + rx, 0.).aligned(Alignment::Right).width(1.),
                curl!(x + rx, y + ry / 2.),
                // Top arc
                g4!(x + mid_curve_w, y_hi - mid_curve_h / 2.).width(1.),
                g4!(x, y_hi + ovs).width(0.9),
                g4!(x - mid_curve_w * 1.5, y_hi - mid_curve_h * 1.25).width(0.7),
            ])
            .into_outline()
    }
}
