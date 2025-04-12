//! This is a thin wrapper around the [`spiro`] crate, to convert a spiro curve
//! into the cubic bezier curve defined in this crate.

use spiro::SpiroCP;

use crate::{CubicBezier, cube::CubicBezierBuilder, point::Point2D};

pub fn spiro_to_cube(spiro: &[SpiroCP]) -> Vec<CubicBezier<Point2D>> {
    let mut context = mk_bezier_context(false);
    context.run_spiro(spiro);
    assert!(context.data.active_builder.is_none());
    context.data.curves
}

pub fn spiro_to_cube_with_indices(
    spiro: &[SpiroCP],
) -> (Vec<CubicBezier<Point2D>>, Vec<SpiroPointIndex>) {
    let mut context = mk_bezier_context(true);
    context.run_spiro(spiro);
    assert!(context.data.active_builder.is_none());
    (context.data.curves, context.data.cp_indices)
}

#[derive(Debug)]
pub struct SpiroPointIndex {
    pub curve_index: usize,
    pub segment_index: usize,
}

#[derive(Default)]
struct BezierContextData {
    curves: Vec<CubicBezier<Point2D>>,
    active_builder: Option<CubicBezierBuilder<Point2D>>,

    /// The indices of the spiro control points within the cubic bezier curve.
    /// Numbered in the order they were emitted, among all curves.
    cp_indices: Vec<SpiroPointIndex>,
    /// Whether to log the control points indices. If not set, the indices are
    /// not stored.
    log_cps: bool,
}

type ThisBezierContext = spiro::BezierContext<BezierContextData, ()>;

fn mk_bezier_context(log_cps: bool) -> ThisBezierContext {
    spiro::BezierContext {
        move_fn,
        line_fn,
        curve_fn,
        mark_knot_fn,
        start,
        end,
        data: BezierContextData {
            log_cps,
            ..Default::default()
        },
    }
}

fn start(_this: &mut ThisBezierContext) {}

fn end(this: &mut ThisBezierContext) {
    let old_builder = this.data.active_builder.take();
    if let Some(old_builder) = old_builder {
        this.data.curves.push(old_builder.build());
    }
}

fn move_fn(this: &mut ThisBezierContext, x: f64, y: f64, is_open: bool) {
    let mut new_builder = CubicBezierBuilder::new((x, y).into());
    if !is_open {
        new_builder.close();
    }
    let old_builder = this.data.active_builder.replace(new_builder);

    if let Some(old_builder) = old_builder {
        this.data.curves.push(old_builder.build());
    }
}

fn line_fn(this: &mut ThisBezierContext, x: f64, y: f64) {
    let builder = this
        .data
        .active_builder
        .as_mut()
        .expect("line_fn called without currently building a curve");
    builder.line_to((x, y).into());
}

fn curve_fn(
    this: &mut ThisBezierContext,
    c1x: f64,
    c1y: f64,
    c2x: f64,
    c2y: f64,
    p2x: f64,
    p2y: f64,
) {
    let builder = this
        .data
        .active_builder
        .as_mut()
        .expect("curve_fn called without currently building a curve");
    builder.curve_to((c1x, c1y).into(), (c2x, c2y).into(), (p2x, p2y).into());
}

fn mark_knot_fn(this: &mut ThisBezierContext, id: usize) {
    if this.data.log_cps {
        assert_eq!(
            id,
            this.data.cp_indices.len(),
            "unexpected control point index"
        );
        let curve_index = this.data.curves.len(); // the current curve is not yet in the list
        let segment_index = this
            .data
            .active_builder
            .as_ref()
            .map_or(0, |b| b.segment_count_so_far());
        this.data.cp_indices.push(SpiroPointIndex {
            curve_index,
            segment_index,
        });
    }
}
