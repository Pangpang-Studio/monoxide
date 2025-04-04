use rquickjs::{Class, Ctx};

mod bezier_builder;
mod outline_expr;
mod spiro_builder;

pub fn import_globals(cx: &Ctx<'_>) {
    Class::<spiro_builder::SpiroBuilder>::define(&cx.globals()).unwrap();
}

#[rquickjs::function]
fn declare_bezier<'js>(
    cx: Ctx<'_>,
    x: f64,
    y: f64,
) -> rquickjs::Result<Class<'js, bezier_builder::BezierBuilder>> {
    Ok(todo!())
}
