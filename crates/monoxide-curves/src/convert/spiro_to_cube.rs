//! This is a thin wrapper around the [`spiro`] crate, to convert a spiro curve
//! into the cubic bezier curve defined in this crate.

use monoxide_spiro::{BezCtx, SpiroCp};

use crate::{CubicBezier, cube::CubicBezierBuilder, point::Point2D};

pub fn spiro_to_cube(spiro: &[SpiroCp]) -> Vec<CubicBezier<Point2D>> {
    let mut ctx = BezierContext::new(false);
    assert!(ctx.run_spiro(spiro));
    assert!(ctx.active_builder.is_none());
    ctx.curves
}

/// Convert a spiro curve into a cubic bezier curve, and also return the
/// indices of the spiro control points within the bezier curves.
///
/// The indices are numbered in the order they were emitted, with two parts:
/// the curve index and the segment index. The curve index is the index of
/// the curve within the curve vector, starting at 0. The segment index
/// is the index of the segment within the curve, with 0 representing the
/// starting point of the curve, and subsequent ones representing _segments_,
/// starting from 1.
pub fn spiro_to_cube_with_indices(
    spiro: &[SpiroCp],
) -> (Vec<CubicBezier<Point2D>>, Vec<SpiroPointIndex>) {
    let mut ctx = BezierContext::new(true);
    assert!(ctx.run_spiro(spiro));
    assert!(ctx.active_builder.is_none());
    (ctx.curves, ctx.cp_indices)
}

#[derive(Debug)]
pub struct SpiroPointIndex {
    pub curve_index: usize,
    pub segment_index: usize,
}

#[derive(Default)]
struct BezierContext {
    curves: Vec<CubicBezier<Point2D>>,
    active_builder: Option<CubicBezierBuilder<Point2D>>,

    /// The indices of the spiro control points within the cubic bezier curve.
    /// Numbered in the order they were emitted, among all curves.
    cp_indices: Vec<SpiroPointIndex>,
    /// Whether to log the control points indices. If not set, the indices are
    /// not stored.
    log_cps: bool,
}

impl BezierContext {
    fn new(log_cps: bool) -> Self {
        Self {
            log_cps,
            ..Self::default()
        }
    }
}

impl BezCtx for BezierContext {
    fn end(&mut self) {
        let old_builder = self.active_builder.take();
        if let Some(old_builder) = old_builder {
            self.curves.push(old_builder.build());
        }
    }

    fn move_to(&mut self, x: f64, y: f64, is_open: bool) {
        let mut new_builder = CubicBezierBuilder::new((x, y).into());
        if !is_open {
            new_builder.close();
        }
        let Some(old_builder) = self.active_builder.replace(new_builder) else {
            return;
        };
        self.curves.push(old_builder.build());
    }

    fn line_to(&mut self, x: f64, y: f64) {
        let builder = self
            .active_builder
            .as_mut()
            .expect("line_fn called without currently building a curve");
        builder.line_to((x, y).into());
    }

    fn curve_to(&mut self, c1x: f64, c1y: f64, c2x: f64, c2y: f64, p2x: f64, p2y: f64) {
        let builder = self
            .active_builder
            .as_mut()
            .expect("curve_fn called without currently building a curve");
        builder.curve_to((c1x, c1y).into(), (c2x, c2y).into(), (p2x, p2y).into());
    }

    fn quad_to(&mut self, _: f64, _: f64, _: f64, _: f64) {
        unimplemented!("quad curves are not yet supported")
    }

    fn mark_knot(&mut self, id: usize) {
        if self.log_cps {
            assert_eq!(id, self.cp_indices.len(), "unexpected control point index");
            let curve_index = self.curves.len(); // the current curve is not yet in the list
            let segment_index = self
                .active_builder
                .as_ref()
                .map_or(0, |b| b.segment_count_so_far());
            self.cp_indices.push(SpiroPointIndex {
                curve_index,
                segment_index,
            });
        }
    }
}
