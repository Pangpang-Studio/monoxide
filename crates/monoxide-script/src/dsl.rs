mod bezier_builder;
mod spiro_builder;

use std::{iter, sync::Arc};

pub use bezier_builder::{BezierBuilder, BezierInst};
use linesweeper::BinaryOp;
use monoxide_curves::xform::Affine2D;
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

    fn transformed(self, xform: Affine2D) -> Arc<OutlineExpr>
    where
        Self: Sized,
    {
        self.into_outline().transformed(xform)
    }

    fn or<U: IntoOutline>(self, other: U) -> Arc<OutlineExpr>
    where
        Self: Sized,
    {
        self.into_outline()
            .bool(other.into_outline(), BinaryOp::Union)
    }

    fn diff<U: IntoOutline>(self, other: U) -> Arc<OutlineExpr>
    where
        Self: Sized,
    {
        self.into_outline()
            .bool(other.into_outline(), BinaryOp::Difference)
    }

    fn and<U: IntoOutline>(self, other: U) -> Arc<OutlineExpr>
    where
        Self: Sized,
    {
        self.into_outline()
            .bool(other.into_outline(), BinaryOp::Intersection)
    }

    fn xor<U: IntoOutline>(self, other: U) -> Arc<OutlineExpr>
    where
        Self: Sized,
    {
        self.into_outline()
            .bool(other.into_outline(), BinaryOp::Xor)
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

impl<T: IntoOutline, I: IntoIterator<Item = T>> IntoOutlines for I {
    type Outlines = iter::Map<I::IntoIter, fn(T) -> Arc<OutlineExpr>>;

    fn into_outlines(self) -> Self::Outlines {
        self.into_iter().map(T::into_outline)
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

    fn transformed(self, xform: Affine2D) -> impl IntoIterator<Item = Arc<OutlineExpr>>
    where
        Self: Sized,
    {
        self.into_outlines()
            .into_iter()
            .map(move |outline| outline.transformed(xform))
    }

    fn or(self) -> Option<Arc<OutlineExpr>>
    where
        Self: Sized,
    {
        self.into_outlines()
            .into_iter()
            .reduce(|acc, it| acc.or(it))
    }
}

impl<T: IntoOutlines> IntoOutlinesExt for T {}

#[doc(hidden)]
#[macro_export]
macro_rules! ctrl_pt {
    ($ctor:expr, $elem:expr $(,)?) => { ($ctor)($elem) };
    ($ctor:expr, $($x:expr, $y:expr),+ $(,)?) => { $crate::ctrl_pt!($ctor, ($($x, $y),+)) };
}
