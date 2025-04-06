use monoxide_curves::{point::Point2D, CubicBezier};

use crate::ast::{FontContext, OutlineExpr};

pub fn eval(cx: &FontContext) -> monoxide_ttf::model::FontFile {
    todo!()
}

pub fn eval_outline(expr: &OutlineExpr, out: &mut Vec<CubicBezier<Point2D>>) {
    todo!()
}
