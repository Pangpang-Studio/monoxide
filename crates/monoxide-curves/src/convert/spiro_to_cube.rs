//! This is a thin wrapper around the [`spiro`] crate, to convert a spiro curve
//! into the cubic bezier curve defined in this crate.

use spiro::SpiroCP;

use crate::{CubicBezier, cube::CubicBezierBuilder};

pub fn spiro_to_cube(spiro: &[SpiroCP]) -> Vec<CubicBezier<(f64, f64)>> {
    let mut context = mk_bezier_context();
    context.run_spiro(spiro);
    assert!(context.data.active_builder.is_none());
    context.data.curves
}

#[derive(Default)]
struct BezierContextData {
    pub(self) curves: Vec<CubicBezier<(f64, f64)>>,
    pub(self) active_builder: Option<CubicBezierBuilder<(f64, f64)>>,
}

type ThisBezierContext = spiro::BezierContext<BezierContextData, ()>;

fn mk_bezier_context() -> ThisBezierContext {
    spiro::BezierContext {
        move_fn,
        line_fn,
        curve_fn,
        mark_knot_fn,
        start,
        end,
        data: BezierContextData::default(),
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
    let mut new_builder = CubicBezierBuilder::new((x, y));
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
    builder.line_to((x, y));
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
    builder.curve_to((c1x, c1y), (c2x, c2y), (p2x, p2y));
}

fn mark_knot_fn(_this: &mut ThisBezierContext, _id: usize) {
    // todo?
}
