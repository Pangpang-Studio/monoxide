use monoxide_curves::{cube::CubicBezierBuilder, point::Point2D};
use rquickjs::{class::Trace, prelude::This, Class, Ctx, JsLifetime};

use super::outline_expr::OutlineExprObject;
use crate::ast::OutlineExpr;

#[rquickjs::class]
#[derive(JsLifetime)]
pub struct BezierBuilder {
    value: Option<CubicBezierBuilder<Point2D>>,
}

impl<'js> Trace<'js> for BezierBuilder {
    fn trace<'a>(&self, _tracer: rquickjs::class::Tracer<'a, 'js>) {}
}

impl BezierBuilder {
    pub fn new(cx: Ctx<'_>, start: Point2D) -> rquickjs::Result<Class<'_, Self>> {
        Class::instance(
            cx,
            BezierBuilder {
                value: Some(CubicBezierBuilder::new(start)),
            },
        )
    }
}

#[rquickjs::methods]
#[qjs(rename_all = "camelCase")]
impl BezierBuilder {
    pub fn line_to(
        this_: This<Class<'_, Self>>,
        x: f64,
        y: f64,
    ) -> rquickjs::Result<Class<'_, Self>> {
        let mut this = this_.borrow_mut();
        let v = this.value.as_mut().unwrap();
        v.line_to((x, y).into());
        drop(this);
        Ok(this_.0)
    }

    pub fn curve_to(
        this_: This<Class<'_, Self>>,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        x3: f64,
        y3: f64,
    ) -> rquickjs::Result<Class<'_, Self>> {
        let mut this = this_.borrow_mut();
        let v = this.value.as_mut().unwrap();
        v.curve_to((x1, y1).into(), (x2, y2).into(), (x3, y3).into());
        drop(this);
        Ok(this_.0)
    }

    pub fn close(this_: This<Class<'_, Self>>) -> rquickjs::Result<Class<'_, Self>> {
        let mut this = this_.borrow_mut();
        let v = this.value.as_mut().unwrap();
        v.close();
        drop(this);
        Ok(this_.0)
    }

    pub fn build<'js>(
        this_: This<Class<'js, Self>>,
        cx: Ctx<'js>,
    ) -> rquickjs::Result<Class<'js, OutlineExprObject>> {
        let mut this = this_.borrow_mut();
        Class::instance(
            cx,
            OutlineExprObject::new(OutlineExpr::Bezier(
                this.value
                    .take()
                    .expect("This builder is already finished")
                    .build(),
            )),
        )
    }
}
