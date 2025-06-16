use std::sync::Arc;

use monoxide_curves::{cube::CubicBezierBuilder, point::Point2D};

use super::IntoOutline;
use crate::ast::OutlineExpr;

pub struct BezierBuilder {
    start: Point2D,
    insts: Vec<BezierInst>,
    is_closed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BezierInst {
    Line(Point2D),
    Curve(Point2D, Point2D, Point2D),
}

impl BezierInst {
    pub fn line(pt: impl Into<Point2D>) -> Self {
        BezierInst::Line(pt.into())
    }

    pub fn curve(
        pt0: impl Into<Point2D>,
        pt1: impl Into<Point2D>,
        pt2: impl Into<Point2D>,
    ) -> Self {
        BezierInst::Curve(pt0.into(), pt1.into(), pt2.into())
    }
}

/// Convenience macro to create a [`BezierInst::Line`].
///
/// # Examples
/// ```
/// # use monoxide_script::line;
/// let p = line!(3., 4.);
/// # _ = line!(3., 4.,);
/// let pair = (3., 4.);
/// let q = line!(pair);
/// # _ = line!(pair,);
/// assert_eq!(p, q);
/// ```
#[macro_export]
macro_rules! line {
    ($($body:tt)+) => {
        $crate::ctrl_pt!($crate::dsl::BezierInst::line, $($body)+)
    };
}

/// Convenience macro to create a [`BezierInst::Curve`].
///
/// # Examples
/// ```
/// # use monoxide_script::curve;
/// let p = curve!(3., 4., 5., 6., 7., 8.);
/// # _ = curve!(3., 4., 5., 6., 7., 8.,);
/// let pair = (3., 4.);
/// let q = curve!(pair, (5., 6.), (7., 8.));
/// # _ = curve!(pair, (5., 6.), (7., 8.),);
/// assert_eq!(p, q);
/// ```
#[macro_export]
macro_rules! curve {
    ($p:expr, $q:expr, $r:expr $(,)?) => { $crate::dsl::BezierInst::curve($p, $q, $r) };
    ($($x:expr, $y:expr),+ $(,)?) => { $crate::curve!($(($x, $y)),+) };
}

impl BezierBuilder {
    fn new(is_closed: bool, start: impl Into<Point2D>) -> Self {
        Self {
            start: start.into(),
            insts: vec![],
            is_closed,
        }
    }

    pub fn closed(start: impl Into<Point2D>) -> Self {
        Self::new(true, start)
    }

    pub fn open(start: impl Into<Point2D>) -> Self {
        Self::new(false, start)
    }

    pub fn inst(mut self, inst: BezierInst) -> Self {
        self.insts.push(inst);
        self
    }

    pub fn insts(mut self, insts: impl IntoIterator<Item = BezierInst>) -> Self {
        for inst in insts {
            self = self.inst(inst);
        }
        self
    }

    pub fn build(mut self) -> OutlineExpr {
        let mut b = CubicBezierBuilder::new(self.start);
        if self.is_closed {
            if let Some(BezierInst::Line(pt) | BezierInst::Curve(_, _, pt)) = self.insts.last_mut()
            {
                *pt = self.start;
            }
        }
        for inst in self.insts {
            match inst {
                BezierInst::Line(pt) => b.line_to(pt),
                BezierInst::Curve(p1, p2, p3) => b.curve_to(p1, p2, p3),
            };
        }
        if self.is_closed {
            b.close();
        }
        OutlineExpr::Bezier(b.build())
    }
}

impl IntoOutline for BezierBuilder {
    fn into_outline(self) -> Arc<OutlineExpr> {
        Arc::new(self.build())
    }
}
