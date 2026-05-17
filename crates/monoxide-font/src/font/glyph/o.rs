use std::sync::Arc;

use monoxide_script::prelude::*;

use crate::{InputContext, font::dir::Alignment};

pub fn o(cx: &InputContext) -> Glyph {
    let_settings! { { mid, mih, ovs, sbl, stw } = cx.settings(); }

    Glyph::builder()
        .outline(OShape::new((mid, mih), (mid - sbl, mih), ovs).stroked(stw))
        .build()
}

pub fn o_cap(cx: &InputContext) -> Glyph {
    let_settings! { { mid, cap, ovs, ovh, sbl, stw } = cx.settings(); }

    Glyph::builder()
        .outline(
            OCapShape::new((mid, cap / 2.), (mid - sbl, cap / 2.), ovs)
                .with_ovh(ovh)
                .stroked(stw),
        )
        .build()
}

pub struct OShape {
    pub center: Point2D,
    pub radii: Point2D,
    pub ovs: f64,
    pub ovh: f64,
}

pub trait IOShape {
    fn center(&self) -> Point2D;
    fn radii(&self) -> Point2D;
    fn ovs(&self) -> f64;
    fn ovh(&self) -> f64;

    fn mid_curve_w(&self) -> f64;
    fn mid_curve_h(&self) -> f64;
    fn end_curve_h(&self) -> f64;

    fn left(&self) -> f64 {
        self.center().x - self.radii().x - self.ovh()
    }

    fn right(&self) -> f64 {
        self.center().x + self.radii().x + self.ovh()
    }
}

impl OShape {
    pub fn new(center: impl Into<Point2D>, radii: impl Into<Point2D>, ovs: f64) -> Self {
        Self {
            center: center.into(),
            radii: radii.into(),
            ovs,
            ovh: Default::default(),
        }
    }

    pub fn with_ovh(mut self, ovh: impl Into<Option<f64>>) -> Self {
        self.ovh = ovh.into().unwrap_or_default();
        self
    }
}

impl IOShape for OShape {
    fn center(&self) -> Point2D {
        self.center
    }

    fn radii(&self) -> Point2D {
        self.radii
    }

    fn ovs(&self) -> f64 {
        self.ovs
    }

    fn ovh(&self) -> f64 {
        self.ovh
    }

    fn mid_curve_w(&self) -> f64 {
        0.85 * self.radii.x
    }

    fn mid_curve_h(&self) -> f64 {
        (5. / 16.) * self.radii.y
    }

    fn end_curve_h(&self) -> f64 {
        (15. / 16.) * self.radii.y
    }
}

impl IntoOutline for OShape {
    fn into_outline(self) -> Arc<OutlineExpr> {
        let Self {
            center: Point2D { x, y },
            radii: Point2D { y: ry, .. },
            ovs,
            ..
        } = self;

        let mid_curve_w = self.mid_curve_w();
        let mid_curve_h = self.mid_curve_h();
        let end_curve_h = self.end_curve_h();

        let left = self.left();
        let right = self.right();
        let y_hi = y + ry;
        let y_lo = y - ry;

        SpiroBuilder::closed()
            .insts([
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
                flat!(right, y_lo + end_curve_h),
                curl!(right, y_hi - end_curve_h),
            ])
            .into_outline()
    }
}

pub struct OCapShape {
    pub o_shape: OShape,
}

impl OCapShape {
    pub fn new(center: impl Into<Point2D>, radii: impl Into<Point2D>, ovs: f64) -> Self {
        Self {
            o_shape: OShape::new(center, radii, ovs),
        }
    }

    pub fn with_ovh(mut self, ovh: impl Into<Option<f64>>) -> Self {
        self.o_shape = self.o_shape.with_ovh(ovh);
        self
    }
}

impl IOShape for OCapShape {
    fn center(&self) -> Point2D {
        self.o_shape.center()
    }

    fn radii(&self) -> Point2D {
        self.o_shape.radii()
    }

    fn ovs(&self) -> f64 {
        self.o_shape.ovs()
    }

    fn ovh(&self) -> f64 {
        self.o_shape.ovh()
    }

    fn mid_curve_h(&self) -> f64 {
        (3.75 / 16.) * self.radii().y
    }

    fn mid_curve_w(&self) -> f64 {
        self.o_shape.mid_curve_w()
    }

    fn end_curve_h(&self) -> f64 {
        (13. / 16.) * self.radii().y
    }
}

impl IntoOutline for OCapShape {
    fn into_outline(self) -> Arc<OutlineExpr> {
        let Point2D { x, y } = self.center();
        let Point2D { y: ry, .. } = self.radii();
        let ovs = self.ovs();

        let mid_curve_w = self.mid_curve_w();
        let mid_curve_h = self.mid_curve_h();
        let end_curve_h = self.end_curve_h();

        let left = self.left();
        let right = self.right();
        let y_hi = y + ry;
        let y_lo = y - ry;

        SpiroBuilder::closed()
            .insts([
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
                flat!(right, y_lo + end_curve_h),
                curl!(right, y_hi - end_curve_h),
            ])
            .into_outline()
    }
}
