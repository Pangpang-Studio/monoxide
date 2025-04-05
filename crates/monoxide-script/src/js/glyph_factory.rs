use rquickjs::{class::Trace, prelude::This, Class, Ctx, Function, JsLifetime, Value};

use crate::ast::{FontContext, SimpleGlyph};

use super::outline_expr::OutlineExprObject;

#[derive(JsLifetime)]
#[rquickjs::class]
pub struct GlyphFactory {
    value: FontContext,
}

impl Trace<'_> for GlyphFactory {
    fn trace(&self, _tracer: rquickjs::class::Tracer<'_, '_>) {}
}

impl GlyphFactory {
    pub fn new(cx: Ctx<'_>) -> rquickjs::Result<Class<'_, Self>> {
        Class::instance(
            cx,
            GlyphFactory {
                value: FontContext::default(),
            },
        )
    }
}

#[rquickjs::methods]
impl GlyphFactory {
    pub fn simple<'js>(
        this_: This<Class<'js, Self>>,
        f: Function<'js>,
        cx: Ctx<'js>,
    ) -> rquickjs::Result<Value<'js>> {
        let builder = Class::instance(
            cx.clone(),
            SimpleGlyphBuilder {
                value: SimpleGlyph::default(),
            },
        )?;
        let _: () = f.call((builder.clone(),))?;
        let mut builder_ref = builder.borrow_mut();
        let builder = std::mem::take(&mut builder_ref.value);

        let mut this = this_.borrow_mut();
        let id = this
            .value
            .add_glyph(crate::ast::GlyphEntry::Simple(builder));

        Ok(Value::new_int(cx, id.0 as i32))
    }
}

#[derive(JsLifetime)]
#[rquickjs::class]
pub struct SimpleGlyphBuilder {
    value: SimpleGlyph,
}

impl Trace<'_> for SimpleGlyphBuilder {
    fn trace(&self, _tracer: rquickjs::class::Tracer<'_, '_>) {}
}

#[rquickjs::methods]
impl SimpleGlyphBuilder {
    pub fn add<'js>(
        this_: This<Class<'js, Self>>,
        outline: Class<'js, OutlineExprObject>,
    ) -> rquickjs::Result<()> {
        let mut this = this_.borrow_mut();
        let outline = outline.borrow();
        this.value.outlines.push(outline.item.clone());
        Ok(())
    }
}
