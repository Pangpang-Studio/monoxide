use std::rc::Rc;

use rquickjs::{class::Trace, prelude::This, Class, Ctx, JsLifetime};

use crate::ast::OutlineExpr;

#[derive(JsLifetime)]
#[rquickjs::class]
pub struct OutlineExprObject {
    pub item: Rc<OutlineExpr>,
}

impl<'js> Trace<'js> for OutlineExprObject {
    fn trace<'a>(&self, _tracer: rquickjs::class::Tracer<'a, 'js>) {}
}

impl OutlineExprObject {
    pub fn new(expr: OutlineExpr) -> Self {
        Self {
            item: Rc::new(expr),
        }
    }
}

#[rquickjs::methods]
impl OutlineExprObject {
    pub fn stroked<'js>(
        this_: This<Class<'js, Self>>,
        width: f64,
        cx: Ctx<'js>,
    ) -> rquickjs::Result<Class<'js, Self>> {
        if width <= 0. {
            return Err(rquickjs::Exception::throw_message(
                &cx,
                "Cannot stroke with a width <= 0",
            ));
        }
        let outline = this_.borrow().item.clone();
        if !matches!(&*outline, OutlineExpr::Spiro(..)) {
            return Err(rquickjs::Exception::throw_message(
                &cx,
                "Stroke currently only supports Spiro outlines",
            ));
        }

        Class::instance(
            cx,
            OutlineExprObject::new(OutlineExpr::Stroked(outline, width)),
        )
    }
}
