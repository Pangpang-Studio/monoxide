use rquickjs::{prelude::*, Class};
use spiro::{SpiroCP, SpiroCpTy};

use crate::ast::OutlineExpr;

use super::outline_expr::OutlineExprObject;

#[rquickjs::class]
#[derive(rquickjs::JsLifetime)]
pub struct SpiroBuilder {
    points: Vec<SpiroCP>,
}

impl<'js> rquickjs::class::Trace<'js> for SpiroBuilder {
    fn trace<'a>(&self, _tracer: rquickjs::class::Tracer<'a, 'js>) {}
}

impl SpiroBuilder {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    pub fn push_pt(&mut self, x: f64, y: f64, kind: SpiroCpTy) {
        self.points.push(SpiroCP { x, y, ty: kind });
    }
}

impl Default for SpiroBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[rquickjs::methods]
#[qjs(rename_all = "camelCase")]
impl SpiroBuilder {
    #[qjs(constructor)]
    fn ctor() -> Self {
        Self::new()
    }

    pub fn corner(
        this: This<Class<'_, Self>>,
        x: f64,
        y: f64,
    ) -> rquickjs::Result<Class<'_, Self>> {
        this.borrow_mut().push_pt(x, y, SpiroCpTy::Corner);
        Ok(this.0)
    }

    pub fn g4(this: This<Class<'_, Self>>, x: f64, y: f64) -> rquickjs::Result<Class<'_, Self>> {
        this.borrow_mut().push_pt(x, y, SpiroCpTy::G4);
        Ok(this.0)
    }

    pub fn g2(this: This<Class<'_, Self>>, x: f64, y: f64) -> rquickjs::Result<Class<'_, Self>> {
        this.borrow_mut().push_pt(x, y, SpiroCpTy::G2);
        Ok(this.0)
    }

    pub fn flat(this: This<Class<'_, Self>>, x: f64, y: f64) -> rquickjs::Result<Class<'_, Self>> {
        this.borrow_mut().push_pt(x, y, SpiroCpTy::Left);
        Ok(this.0)
    }

    pub fn curl(this: This<Class<'_, Self>>, x: f64, y: f64) -> rquickjs::Result<Class<'_, Self>> {
        this.borrow_mut().push_pt(x, y, SpiroCpTy::Right);
        Ok(this.0)
    }

    pub fn anchor(
        this: This<Class<'_, Self>>,
        x: f64,
        y: f64,
    ) -> rquickjs::Result<Class<'_, Self>> {
        this.borrow_mut().push_pt(x, y, SpiroCpTy::Anchor);
        Ok(this.0)
    }

    pub fn handle(
        this: This<Class<'_, Self>>,
        x: f64,
        y: f64,
    ) -> rquickjs::Result<Class<'_, Self>> {
        this.borrow_mut().push_pt(x, y, SpiroCpTy::Handle);
        Ok(this.0)
    }

    pub fn end(this: This<Class<'_, Self>>, x: f64, y: f64) -> rquickjs::Result<Class<'_, Self>> {
        this.borrow_mut().push_pt(x, y, SpiroCpTy::End);
        Ok(this.0)
    }

    pub fn open(this: This<Class<'_, Self>>, x: f64, y: f64) -> rquickjs::Result<Class<'_, Self>> {
        this.borrow_mut().push_pt(x, y, SpiroCpTy::Open);
        Ok(this.0)
    }

    pub fn end_open(
        this: This<Class<'_, Self>>,
        x: f64,
        y: f64,
    ) -> rquickjs::Result<Class<'_, Self>> {
        this.borrow_mut().push_pt(x, y, SpiroCpTy::EndOpen);
        Ok(this.0)
    }

    pub fn build<'js>(
        cx: Ctx<'js>,
        this_: This<Class<'js, Self>>,
    ) -> rquickjs::Result<Class<'js, OutlineExprObject>> {
        let mut this = this_.borrow_mut();
        let expr = OutlineExpr::Spiro(std::mem::take(&mut this.points));
        drop(this);
        Class::instance(cx, OutlineExprObject::new(expr))
    }
}
