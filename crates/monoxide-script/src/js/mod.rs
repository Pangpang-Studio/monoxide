use rquickjs::{Class, Ctx, JsLifetime};

use crate::{ast::FontContext, FontParamSettings};

mod bezier_builder;
mod glyph_factory;
mod outline_expr;
mod spiro_builder;

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
    spiro_builder::SpiroBuilder::new(cx)
}

/// The struct to attach to the Monoxide-enabled QuickJS runtime. Contains data
/// for the rest of the program to evaluate.
#[derive(Clone, JsLifetime)]
pub struct ContextAttachment<'js> {
    pub font_params: FontParamSettings,
    factory: Class<'js, glyph_factory::GlyphFactory>,
}

impl std::fmt::Debug for ContextAttachment<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let factory = self.factory.borrow();

        f.debug_struct("ContextAttachment")
            .field("font_params", &self.font_params)
            .field("factory", &*factory)
            .finish()
    }
}

impl<'js> ContextAttachment<'js> {
    pub fn new(cx: Ctx<'js>, params: FontParamSettings) -> rquickjs::Result<Self> {
        let globals = &cx.globals();
        Class::<glyph_factory::GlyphFactory>::define(globals)?;
        Class::<glyph_factory::SimpleGlyphBuilder>::define(globals)?;
        Class::<bezier_builder::BezierBuilder>::define(globals)?;
        Class::<spiro_builder::SpiroBuilder>::define(globals)?;
        Class::<outline_expr::OutlineExprObject>::define(globals)?;

        let factory = glyph_factory::GlyphFactory::new(cx.clone(), params.clone())?;

        Ok(ContextAttachment {
            font_params: params,
            factory,
        })
    }

    pub fn take(&self) -> FontContext {
        let mut fac = self.factory.borrow_mut();
        fac.take()
    }
}

/// Represents a JS Module within the
pub struct MonoxideModule;

impl rquickjs::module::ModuleDef for MonoxideModule {
    fn declare(decl: &rquickjs::module::Declarations<'_>) -> rquickjs::Result<()> {
        decl.declare("glyph")?;
        decl.declare("bezier")?;
        decl.declare("spiro")?;
        decl.declare("settings")?;
        Ok(())
    }

    fn evaluate<'js>(
        cx: &Ctx<'js>,
        exports: &rquickjs::module::Exports<'js>,
    ) -> rquickjs::Result<()> {
        let ud = cx
            .userdata::<ContextAttachment<'js>>()
            .expect("Cannot find user adata");

        exports.export("glyph", ud.factory.clone())?;
        exports.export("bezier", js_declare_bezier)?;
        exports.export("spiro", js_declare_spiro)?;

        exports.export("settings", ud.font_params.populate(cx.clone())?)?;

        Ok(())
    }
}

pub fn insert_globals<'js>(cx: Ctx<'js>) -> rquickjs::Result<()> {
    let globals = cx.globals();
    let ud = cx
        .userdata::<ContextAttachment<'js>>()
        .expect("Cannot find user adata");

    globals.set("glyph", ud.factory.clone())?;
    globals.set("bezier", js_declare_bezier)?;
    globals.set("spiro", js_declare_spiro)?;

    globals.set("settings", ud.font_params.populate(cx.clone())?)?;

    Ok(())
}
