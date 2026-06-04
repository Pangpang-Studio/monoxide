use std::sync::Arc;

use monoxide_script::prelude::*;

use crate::{
    InputContext,
    dir::{Alignment, Dir},
    glyph::{
        c::CShape,
        d::Bowl,
        j::JShape,
        o::{IOShape, OCapShape, OShape},
    },
    prelude::*,
    shape::Rect,
};

pub fn g(cx: &InputContext) -> Glyph {
    Glyph::builder()
        .outlines(GShape::from_settings(&cx.settings))
        .build()
}

pub fn g_cap(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        mid,
        cap,
        ovs,
        ovh,
        sbl,
        stw,
        ..
    } = cx.settings().view();

    Glyph::builder()
        .outlines(
            GCapShape::new((mid, cap / 2.), (mid - sbl, cap / 2.), ovs)
                .with_ovh(ovh)
                .stroked(stw),
        )
        .build()
}

pub struct GShape {
    bowl: Arc<OutlineExpr>,
    hook: Arc<OutlineExpr>,
}

impl GShape {
    pub fn from_settings(settings: &FontParamSettings) -> Self {
        let FontParamSettingsView {
            mid,
            mih,
            ovs,
            sbl,
            stw,
            xh,
            dsc,
            ..
        } = settings.view();

        let bowl = Bowl::new((mid - stw / 4., mih), (mid - sbl - stw / 4., mih), ovs)
            .stroked(stw)
            .into_outline();
        let hook = Hook::new((mid, mih - dsc), (mid - sbl, mih), ovs)
            .stroked(1.05 * stw)
            .transformed(Affine2D::mirrored_along((0., xh / 2.), (1., 0.)));

        Self { bowl, hook }
    }
}

impl IntoOutlines for GShape {
    type Outlines = [Arc<OutlineExpr>; 2];

    fn into_outlines(self) -> Self::Outlines {
        [self.bowl.into_outline(), self.hook.into_outline()]
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
                flat!(x + rx, 0.).aligned(Alignment::Right).width(1.1),
                curl!(x + rx, y + ry / 3.),
                // Top arc
                g4!(x + mid_curve_w * 0.9, y_hi - mid_curve_h / 2.).width(1.),
                g4!(x, y_hi + ovs).width(0.9).heading(Dir::L),
                g4!(x - mid_curve_w * 0.9, y_hi - mid_curve_h / 2.)
                    .width(1.)
                    .aligned(Alignment::Right),
                g4!(x - rx, y_hi - mid_curve_h * 1.2)
                    .width(1.)
                    .heading(JShape::HOOK_TIP_HEADING),
            ])
            .into_outline()
    }
}

pub struct GCapShape {
    pub bowl: CapBowl,
}

impl GCapShape {
    pub fn new(center: impl Into<Point2D>, radii: impl Into<Point2D>, ovs: f64) -> Self {
        Self {
            bowl: CapBowl::new(center, radii, ovs),
        }
    }

    pub fn with_ovh(mut self, ovh: impl Into<Option<f64>>) -> Self {
        self.bowl = self.bowl.with_ovh(ovh);
        self
    }

    pub fn center(&self) -> Point2D {
        self.bowl.center()
    }

    pub fn right(&self) -> f64 {
        self.bowl.right()
    }
}

impl IntoOutlines for GCapShape {
    type Outlines = [Arc<OutlineExpr>; 2];

    fn into_outlines(self) -> [Arc<OutlineExpr>; 2] {
        let Point2D { x, y } = self.center();
        let bar = Rect::new((x, y), (self.right(), y));
        [self.bowl.into_outline(), bar.into_outline()]
    }
}
pub struct CapBowl {
    pub o_shape: OCapShape,
}

impl CapBowl {
    pub fn new(center: impl Into<Point2D>, radii: impl Into<Point2D>, ovs: f64) -> Self {
        Self {
            o_shape: OCapShape::new(center, radii, ovs),
        }
    }

    pub fn with_ovh(mut self, ovh: impl Into<Option<f64>>) -> Self {
        self.o_shape = self.o_shape.with_ovh(ovh);
        self
    }

    pub fn center(&self) -> Point2D {
        self.o_shape.center()
    }

    pub fn right(&self) -> f64 {
        let o_shape = &self.o_shape;
        o_shape.center().x + o_shape.radii().x
    }
}

impl IntoOutline for CapBowl {
    fn into_outline(self) -> Arc<OutlineExpr> {
        let o_shape = self.o_shape;
        let Point2D { x, y } = o_shape.center();
        let Point2D { x: rx, y: ry, .. } = o_shape.radii();
        let ovs = o_shape.ovs();

        let mid_curve_w = o_shape.mid_curve_w();
        let mid_curve_h = o_shape.mid_curve_h();
        let end_curve_h = o_shape.end_curve_h();

        // TODO: Find out why the ovh is not applied (compared to `C`)
        let left = o_shape.left();
        let right = x + rx;
        let aperture_curve_h = CShape::from(o_shape).aperture_curve_h();

        let y_hi = y + ry;
        let y_lo = y - ry;

        SpiroBuilder::open()
            .insts([
                // Right side (upper)
                curl!(right, y_hi - aperture_curve_h).heading(Dir::D),
                // Top arc
                g4!(x + mid_curve_w, y_hi - mid_curve_h),
                g4!(x, y_hi + ovs),
                g4!(x - mid_curve_w, y_hi - mid_curve_h),
                // Left side
                flat!(left, y_hi - end_curve_h),
                curl!(left, y_lo + end_curve_h),
                // Bottom arc
                g4!(x - mid_curve_w, y_lo + mid_curve_h).aligned(Alignment::Right),
                g4!(x, y_lo - ovs),
                g4!(x + mid_curve_w, y_lo + mid_curve_h),
                // Right side
                corner!(right, y_lo + aperture_curve_h),
                curl!(right, y).heading(Dir::U),
            ])
            .into_outline()
    }
}
