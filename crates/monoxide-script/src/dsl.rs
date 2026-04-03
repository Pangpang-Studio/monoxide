mod bezier_builder;
mod spiro_builder;

use std::{iter, sync::Arc};

pub use bezier_builder::{BezierBuilder, BezierInst};
use itertools::chain;
use monoxide_curves::{point::Point2D, xform::Affine2D};
pub use spiro_builder::{SpiroBuilder, SpiroInst, SpiroInstOpts};

use crate::ast::OutlineExpr;

pub trait IntoOutline {
    fn into_outline(self) -> Arc<OutlineExpr>;
}

impl IntoOutline for Arc<OutlineExpr> {
    fn into_outline(self) -> Arc<OutlineExpr> {
        self
    }
}

impl IntoOutline for OutlineExpr {
    fn into_outline(self) -> Arc<OutlineExpr> {
        Arc::new(self)
    }
}

pub trait IntoOutlineExt: IntoOutline {
    fn stroked(self, width: f64) -> Arc<OutlineExpr>
    where
        Self: Sized,
    {
        self.into_outline().stroked(width)
    }

    fn transformed(self, xform: Affine2D<Point2D>) -> Arc<OutlineExpr>
    where
        Self: Sized,
    {
        self.into_outline().transformed(xform)
    }
}

impl<T: IntoOutline> IntoOutlineExt for T {}

pub trait IntoStrokeAlignment {
    fn into_alignment(self) -> f64;
}

impl IntoStrokeAlignment for f64 {
    fn into_alignment(self) -> f64 {
        self
    }
}

pub trait IntoOutlines {
    type Outlines: IntoIterator<Item = Arc<OutlineExpr>>;

    fn into_outlines(self) -> Self::Outlines;
}

impl<I: IntoIterator<Item = Arc<OutlineExpr>>> IntoOutlines for I {
    type Outlines = Self;

    fn into_outlines(self) -> Self::Outlines {
        self
    }
}

pub trait IntoOutlinesExt: IntoOutlines {
    fn stroked(self, width: f64) -> impl IntoIterator<Item = Arc<OutlineExpr>>
    where
        Self: Sized,
    {
        self.into_outlines()
            .into_iter()
            .map(move |outline| outline.stroked(width))
    }

    fn transformed(self, xform: Affine2D<Point2D>) -> impl IntoIterator<Item = Arc<OutlineExpr>>
    where
        Self: Sized,
    {
        self.into_outlines()
            .into_iter()
            .map(move |outline| outline.transformed(xform))
    }

    fn add<U: IntoOutlines>(self, other: U) -> Add<Self, U>
    where
        Self: Sized,
    {
        Add(self, other)
    }
}

impl<T: IntoOutlines> IntoOutlinesExt for T {}

pub struct Add<T, U>(T, U);

impl<T: IntoOutlines, U: IntoOutlines> IntoOutlines for Add<T, U> {
    type Outlines = iter::Chain<
        <T::Outlines as IntoIterator>::IntoIter,
        <U::Outlines as IntoIterator>::IntoIter,
    >;

    fn into_outlines(self) -> Self::Outlines {
        chain!(self.0.into_outlines(), self.1.into_outlines())
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! ctrl_pt {
    ($ctor:expr, $elem:expr $(,)?) => { ($ctor)($elem) };
    ($ctor:expr, $($x:expr, $y:expr),+ $(,)?) => { $crate::ctrl_pt!($ctor, ($($x, $y),+)) };
}
