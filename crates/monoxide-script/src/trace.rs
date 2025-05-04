//! Provides trait for tracing the evaluation of a glyph

use monoxide_curves::{debug::CurveDebugger, point::Point2D, CubicBezier};

/// Trace the evaluation of a glyph. A no-op tracer is provided in [`()`].
pub trait EvaluationTracer {
    /// Represents the ID of a intermediate part of the glyph
    type Id: Copy + Eq;
    /// The [`CurveDebugger`] to use for debugging lower-level processes. If
    /// this type is used, it should be associated a single intermediate output
    /// ID. Use `()` if you can't provide one.
    type CurveDebugger<'a>: CurveDebugger + 'a
    where
        Self: 'a;

    /// Whether it is needed to evaluate intermediate outputs. Supply `false`
    /// here may reduce the number of calculation performed.
    fn needs_evaluate_intermediate() -> bool;
    /// Preallocate the next ID. This is used to provide curve debugger output
    /// before we can supply the full construction arguments.
    fn preallocate_next(&mut self) -> Self::Id;

    fn constructed_beziers<'b>(
        &mut self,
        bezier: impl IntoIterator<Item = &'b CubicBezier<Point2D>>,
    ) -> Self::Id
    where
        Self: 'b;
    fn constructed_spiros<'b>(
        &mut self,
        spiros: impl IntoIterator<Item = &'b [monoxide_spiro::SpiroCp]>,
    ) -> Self::Id
    where
        Self: 'b;
    fn stroked<'b>(
        &mut self,
        parent: Self::Id,
        width: f64,
        spiros: impl IntoIterator<Item = &'b [monoxide_spiro::SpiroCp]>,
    ) -> Self::Id
    where
        Self: 'b;
    fn spiro_to_bezier(&mut self, parent: Self::Id) -> Self::Id;
    fn boolean_added<'b>(&mut self, parents: impl IntoIterator<Item = &'b Self::Id>) -> Self::Id
    where
        Self: 'b;

    fn constructed_bezier(&mut self, bezier: &CubicBezier<Point2D>) -> Self::Id {
        self.constructed_beziers(std::iter::once(bezier))
    }

    fn constructed_spiro(&mut self, spiro: &[monoxide_spiro::SpiroCp]) -> Self::Id {
        self.constructed_spiros(std::iter::once(spiro))
    }

    /// Provide the intermediate output of the given ID for additional debug
    /// info. The callee may omit this if the intermediate output is not needed.
    fn intermediate_output(&mut self, id: Self::Id, curve: &[CubicBezier<Point2D>]) {
        let _ = curve;
        let _ = id;
    }

    /// Provide additional debug information for the given ID.
    fn curve_debugger(&mut self, id: Self::Id) -> Self::CurveDebugger<'_>;
}

/// A no-op tracer that does nothing. This is useful for when you don't need
/// to trace the evaluation of a glyph.
impl EvaluationTracer for () {
    type CurveDebugger<'a> = ();
    type Id = ();

    fn needs_evaluate_intermediate() -> bool {
        false
    }

    fn preallocate_next(&mut self) -> Self::Id {}

    fn constructed_beziers<'b>(
        &mut self,
        _bezier: impl IntoIterator<Item = &'b CubicBezier<Point2D>>,
    ) -> Self::Id {
    }

    fn constructed_spiros<'b>(
        &mut self,
        _spiros: impl IntoIterator<Item = &'b [monoxide_spiro::SpiroCp]>,
    ) -> Self::Id {
    }

    fn stroked<'b>(
        &mut self,
        _parent: Self::Id,
        _width: f64,
        _spiros: impl IntoIterator<Item = &'b [monoxide_spiro::SpiroCp]>,
    ) -> Self::Id {
    }

    fn spiro_to_bezier(&mut self, _parent: Self::Id) -> Self::Id {}

    fn boolean_added<'b>(&mut self, _parents: impl IntoIterator<Item = &'b Self::Id>) -> Self::Id {}

    fn intermediate_output(&mut self, _id: Self::Id, _curve: &[CubicBezier<Point2D>]) {}

    fn curve_debugger(&mut self, _id: Self::Id) -> Self::CurveDebugger<'_> {}
}
