use std::mem;

use monoxide_curves::{
    point::Point2D,
    stroke::{Tangent, TangentOverride},
};
use rquickjs::{prelude::*, Class};
use spiro::{SpiroCP, SpiroCpTy};

use super::outline_expr::OutlineExprObject;
use crate::ast::OutlineExpr;

#[rquickjs::class]
#[derive(rquickjs::JsLifetime, Default)]
pub struct SpiroBuilder {
    points: Vec<SpiroCP>,
    tangent_override: TangentOverride,
}

impl<'js> rquickjs::class::Trace<'js> for SpiroBuilder {
    fn trace<'a>(&self, _tracer: rquickjs::class::Tracer<'a, 'js>) {}
}

impl SpiroBuilder {
    pub fn new(cx: Ctx<'_>) -> rquickjs::Result<Class<'_, SpiroBuilder>> {
        Class::instance(cx, SpiroBuilder::default())
    }

    pub fn push_pt(&mut self, x: f64, y: f64, kind: SpiroCpTy) {
        self.points.push(SpiroCP { x, y, ty: kind });
    }

    /// Adds a tangent override to the builder corresponding to the current
    /// spiro control point.
    ///
    /// NOTE: For the sake of simplicity, we assume the in/out tangents are
    /// identical in this case.
    pub fn override_tangent(&mut self, x: f64, y: f64) {
        let tan = Some(Point2D::from((x, y)).normalize());
        self.tangent_override
            .insert(self.points.len() - 1, Tangent { in_: tan, out: tan });
    }
}

#[rquickjs::methods]
#[qjs(rename_all = "camelCase")]
impl SpiroBuilder {
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

    pub fn heading(
        this: This<Class<'_, Self>>,
        x: f64,
        y: f64,
    ) -> rquickjs::Result<Class<'_, Self>> {
        this.borrow_mut().override_tangent(x, y);
        Ok(this.0)
    }

    pub fn build<'js>(
        cx: Ctx<'js>,
        this_: This<Class<'js, Self>>,
    ) -> rquickjs::Result<Class<'js, OutlineExprObject>> {
        let mut this = this_.borrow_mut();
        let expr = OutlineExpr::Spiro(
            mem::take(&mut this.points),
            mem::take(&mut this.tangent_override),
        );
        drop(this);
        Class::instance(cx, OutlineExprObject::new(expr))
    }
}
