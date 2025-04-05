use std::rc::Rc;

use rquickjs::{class::Trace, JsLifetime};

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
