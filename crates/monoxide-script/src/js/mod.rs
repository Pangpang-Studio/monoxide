use rquickjs::{Class, Ctx, Object};

mod bezier_builder;
mod glyph_factory;
mod outline_expr;
mod spiro_builder;

pub fn import_globals<'js>(cx: Ctx<'js>, global_object: Object<'js>) -> rquickjs::Result<()> {
    let factory = glyph_factory::GlyphFactory::new(cx)?;
    global_object.set("glyph", factory)?;
    global_object.set("bezier", js_declare_bezier)?;
    global_object.set("spiro", js_declare_spiro)?;

    Ok(())
}

#[rquickjs::function]
fn declare_bezier(
    cx: Ctx<'_>,
    x: f64,
    y: f64,
) -> rquickjs::Result<Class<'_, bezier_builder::BezierBuilder>> {
    bezier_builder::BezierBuilder::new(cx, (x, y).into())
}

#[rquickjs::function]
fn declare_spiro(cx: Ctx<'_>) -> rquickjs::Result<Class<'_, spiro_builder::SpiroBuilder>> {
    Class::instance(cx, spiro_builder::SpiroBuilder::new())
}
