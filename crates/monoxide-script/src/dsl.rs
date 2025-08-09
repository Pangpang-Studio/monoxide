mod bezier_builder;
mod spiro_builder;

use std::sync::Arc;

pub use bezier_builder::{BezierBuilder, BezierInst};
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
    fn into_outlines(self) -> impl Iterator<Item = Arc<OutlineExpr>>;
}

impl<I: IntoIterator<Item = Arc<OutlineExpr>>> IntoOutlines for I {
    fn into_outlines(self) -> impl Iterator<Item = Arc<OutlineExpr>> {
        self.into_iter()
    }
}

pub trait IntoOutlinesExt: IntoOutlines {
    fn stroked(self, width: f64) -> impl Iterator<Item = Arc<OutlineExpr>>
    where
        Self: Sized,
    {
        self.into_outlines()
            .map(move |outline| outline.stroked(width))
    }
}

impl<T: IntoOutlines> IntoOutlinesExt for T {}

#[doc(hidden)]
#[macro_export]
macro_rules! ctrl_pt {
    ($ctor:expr, $elem:expr $(,)?) => { ($ctor)($elem) };
    ($ctor:expr, $($x:expr, $y:expr),+ $(,)?) => { $crate::ctrl_pt!($ctor, ($($x, $y),+)) };
}
