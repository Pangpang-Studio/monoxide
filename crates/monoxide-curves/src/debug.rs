//! Debug the generation of strokes

use crate::point::Point2D;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugPointKind {
    Corner,
    Curve,
    Control,
    Misc,
}

/// A trait for debugging the generation of curves and such.
///
/// A no-op implementation is provided for `()` for cases when you don't want
/// it.
pub trait CurveDebugger {
    /// Print a debug point of the given kind
    fn point(&mut self, kind: DebugPointKind, at: Point2D, tag: &str);
    /// Print a debug line
    fn line(&mut self, from: Point2D, to: Point2D, tag: &str);
}

impl CurveDebugger for () {
    fn point(&mut self, _: DebugPointKind, _: Point2D, _: &str) {}

    fn line(&mut self, _: Point2D, _: Point2D, _: &str) {}
}
