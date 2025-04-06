use monoxide_curves::{point::Point2D, CubicBezier};

use crate::ast::{FontContext, OutlineExpr};

pub fn eval(cx: &FontContext) -> monoxide_ttf::model::FontFile {
    todo!()
}

pub fn eval_outline(expr: &OutlineExpr, out: &mut Vec<CubicBezier<Point2D>>) {
    match expr {
        OutlineExpr::Bezier(cubic_bezier) => out.push(cubic_bezier.clone()),
        OutlineExpr::Spiro(spiro_cps) => {
            let bez = monoxide_curves::convert::spiro_to_cube(spiro_cps);
            out.extend_from_slice(&bez);
        }
        OutlineExpr::Stroked(outline_expr, width) => match &**outline_expr {
            OutlineExpr::Spiro(spiro_cps) => {
                let oc =
                    monoxide_curves::stroke::stroke_spiro(spiro_cps, *width, Default::default());
                let bz = monoxide_curves::convert::spiro_to_cube(&oc);
                out.extend_from_slice(&bz);
            }
            other => {
                panic!("Not strokable for now: {:?}", other)
            }
        },
    }
}
