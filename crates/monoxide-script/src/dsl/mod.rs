mod bezier_builder;
mod spiro_builder;

pub use bezier_builder::{BezierBuilder, BezierInst};
pub use spiro_builder::{SpiroBuilder, SpiroInst, SpiroInstOpts};

#[doc(hidden)]
#[macro_export]
macro_rules! ctrl_pt {
    ($ctor:expr, $elem:expr $(,)?) => { ($ctor)($elem) };
    ($ctor:expr, $($x:expr, $y:expr),+ $(,)?) => { $crate::ctrl_pt!($ctor, ($($x, $y),+)) };
}
