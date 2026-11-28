use std::sync::Arc;

use monoxide_script::{g2, prelude::*};

use crate::{
    InputContext,
    dir::{Alignment, Dir},
    glyph::{
        o::{IOShape, OShape},
        p::PCapShape,
    },
    prelude::*,
    shape::{Rect, Slash, SlashAlignment},
};

pub fn r(cx: &InputContext) -> Glyph {
    Glyph::builder()
        .or_outlines(RShape::from_settings(&cx.settings))
        .build()
}

pub fn r_cap(cx: &InputContext) -> Glyph {
    let p_shape = PCapShape::from_settings(cx.settings());

    let tail = {
        let bowl = &p_shape.bowl;

        let o_shape = &bowl.o_shape;
        let Point2D { x, y } = o_shape.center();
        let Point2D { x: rx, y: ry } = o_shape.radii();
        let stw = p_shape.pipe.width.unwrap_or_default();

        Slash::new(x - bowl.end_curve_w()..x + rx, 0.0..y - ry + stw / 2.)
            .with_aln(SlashAlignment::new(0.3, 1.))
            .back()
            .stroked(stw)
    };

    Glyph::builder().or_outlines(p_shape).outline(tail).build()
}

pub fn cyrl_ya_cap(cx: &InputContext) -> Glyph {
    let mid = cx.settings().mid();
    let xform = Affine2D::reflect((mid, 0.), (0., 1.));
    let r = r_cap(cx);

    Glyph::builder()
        .outlines(r.outlines.clone().transformed(xform))
        .advance(r.advance)
        .build()
}

pub struct RShape {
    pub hook: Arc<OutlineExpr>,
    pub pipe: Rect,
    pub offset: Point2D,
}

impl RShape {
    pub fn from_settings(settings: &FontParamSettings) -> Self {
        let FontParamSettingsView {
            mid,
            mih,
            ovs,
            sbl,
            stw,
            xh,
            ..
        } = settings.view();

        let hook = {
            let rx = mid - sbl;
            let ry = mih;
            let hook_tip_width = 0.8;
            Hook::new((mid, mih), (rx, ry), ovs)
                .with_hook_tip_width(hook_tip_width)
                .with_hook_tip_r_fact(1. - stw * (1. - hook_tip_width) / rx)
                .stroked(stw)
        };

        let pipe = Rect::new((sbl, 0.), (sbl, xh))
            .aligned(Alignment::Left)
            .stroked(stw);

        Self {
            hook,
            pipe,
            offset: (stw / 2., 0.).into(),
        }
    }
}

impl IntoOutlines for RShape {
    type Outlines = [Arc<OutlineExpr>; 2];

    fn into_outlines(self) -> Self::Outlines {
        [self.hook.into_outline(), self.pipe.into_outline()]
            .map(move |it| it.transformed(Affine2D::translate(self.offset.to_vec2())))
    }
}

pub struct Hook {
    pub o_shape: OShape,
    pub hook_tip_width: Option<f64>,
    pub hook_tip_r_fact: Option<f64>,
}

impl Hook {
    pub fn new(center: impl Into<Point2D>, radii: impl Into<Point2D>, ovs: f64) -> Self {
        Self {
            o_shape: OShape::new(center, radii, ovs),
            hook_tip_width: None,
            hook_tip_r_fact: None,
        }
    }

    pub fn with_hook_tip_width(mut self, width: impl Into<Option<f64>>) -> Self {
        self.hook_tip_width = width.into();
        self
    }

    pub fn with_hook_tip_r_fact(mut self, offset: impl Into<Option<f64>>) -> Self {
        self.hook_tip_r_fact = offset.into();
        self
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

        let tip_r_factor = self.hook_tip_r_fact.unwrap_or(1.);
        let tip_width = self.hook_tip_width.unwrap_or(1.);

        let y_hi = y + ry;
        let y_hi1 = y + ry * tip_r_factor;

        SpiroBuilder::open()
            .insts([
                // Right side
                g4!(x + rx, y + ry * 0.4)
                    .aligned(Alignment::Right)
                    .heading(Dir::D)
                    .width(1.1),
                // Top arc
                g2!(x + mid_curve_w * 0.9, y_hi - mid_curve_h / 2.).width(1.),
                g4!(x, y_hi + ovs).width(1.).heading(Dir::L),
                g2!(
                    x - mid_curve_w * 0.9 * tip_r_factor,
                    y_hi1 - mid_curve_h / 2.
                )
                .aligned(Alignment::Right),
                g4!(x - rx * tip_r_factor, y_hi1 - mid_curve_h * 1.4).width(tip_width),
            ])
            .into_outline()
    }
}
