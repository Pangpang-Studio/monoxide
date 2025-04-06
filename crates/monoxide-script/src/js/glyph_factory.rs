use rquickjs::{class::Trace, prelude::This, Class, Ctx, Function, JsLifetime, Value};

use crate::ast::{FontContext, SimpleGlyph};

use super::outline_expr::OutlineExprObject;

#[derive(JsLifetime, Debug)]
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

    pub fn take(&mut self) -> FontContext {
        std::mem::take(&mut self.value)
    }
}

#[rquickjs::methods]
#[qjs(rename_all = "camelCase")]
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

    pub fn assign_char<'js>(
        this_: This<Class<'js, Self>>,
        v: i32,
        ch: String,
        cx: Ctx<'js>,
    ) -> rquickjs::Result<()> {
        let mut this = this_.borrow_mut();
        let id = crate::ast::GlyphId(v as usize);
        if ch.chars().count() != 1 {
            return Err(rquickjs::Exception::throw_message(
                &cx,
                "Need exactly one character in assign_char",
            ));
        }
        let ch = ch.chars().next().unwrap();

        this.value.assign_char(id, ch);

        Ok(())
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
