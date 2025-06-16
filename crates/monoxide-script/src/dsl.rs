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

#[doc(hidden)]
#[macro_export]
macro_rules! ctrl_pt {
    ($ctor:expr, $elem:expr $(,)?) => { ($ctor)($elem) };
    ($ctor:expr, $($x:expr, $y:expr),+ $(,)?) => { $crate::ctrl_pt!($ctor, ($($x, $y),+)) };
}
