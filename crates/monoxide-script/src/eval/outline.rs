use monoxide_curves::{CubicBezier, SpiroCurve, point::Point2D, stroke::StrokeResult};

use crate::{ast::OutlineExpr, trace::EvaluationTracer};

pub fn eval_outline<E: EvaluationTracer>(
    expr: &OutlineExpr,
    out: &mut Vec<CubicBezier<Point2D>>,
    dbg: &mut E,
) -> Result<E::Id, EvalError<E::Id>> {
    let evaled = eval_outline_internal(expr, dbg)?;
    let id = match evaled.kind {
        EvalValueKind::Beziers(beziers) => {
            out.extend(beziers);
            evaled.id
        }
        EvalValueKind::Spiros(spiros) => {
            let output_size_before = out.len();
            for spiro in spiros {
                let bez = monoxide_curves::convert::spiro_to_cube(&spiro.points);

                out.extend(bez);
            }
            let id = dbg.spiro_to_bezier(evaled.id);
            dbg.intermediate_output(id, &out[output_size_before..]);
            id
        }
    };

    Ok(id)
}

/// Represents an intermediate value during the evaluation of a glyph.
#[derive(Debug, Clone)]
pub enum EvalValueKind {
    Beziers(Vec<CubicBezier<Point2D>>),
    Spiros(Vec<SpiroCurve>),
}

#[derive(Debug, Clone)]
pub struct EvalValue<Id> {
    pub kind: EvalValueKind,
    pub id: Id,
}

impl<Id> EvalValue<Id> {
    pub fn bezier(bezier: CubicBezier<Point2D>, id: Id) -> Self {
        Self {
            kind: EvalValueKind::Beziers(vec![bezier]),
            id,
        }
    }

    pub fn spiro(spiro: SpiroCurve, id: Id) -> Self {
        Self {
            kind: EvalValueKind::Spiros(vec![spiro]),
            id,
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum EvalError<Id> {
    #[error("Stroking a bezier is not supported yet; at {0}")]
    StrokingABezier(Id),
}

fn eval_outline_internal<E: EvaluationTracer>(
    expr: &OutlineExpr,
    dbg: &mut E,
) -> Result<EvalValue<E::Id>, EvalError<E::Id>> {
    match expr {
        OutlineExpr::Bezier(cubic_bezier) => {
            let id = dbg.constructed_bezier(cubic_bezier);
            Ok(EvalValue::bezier(cubic_bezier.clone(), id))
        }
        OutlineExpr::Spiro(spiro) => {
            let id = dbg.constructed_spiro(&spiro.points);

            if E::needs_evaluate_intermediate() {
                // convert to beziers if needed
                let bez = monoxide_curves::convert::spiro_to_cube(&spiro.points);
                dbg.intermediate_output(id, &bez);
            }

            Ok(EvalValue::spiro(spiro.clone(), id))
        }
        OutlineExpr::Stroked(outline_expr, width) => {
            let evaled = eval_outline_internal(outline_expr, dbg)?;
            match evaled.kind {
                EvalValueKind::Beziers(_) => Err(EvalError::StrokingABezier(evaled.id)),
                EvalValueKind::Spiros(eval_spiros) => {
                    eval_stroked(evaled.id, &eval_spiros, *width, dbg)
                }
            }
        }
    }
}

fn eval_stroked<E: EvaluationTracer>(
    evaled_id: E::Id,
    eval_spiros: &[SpiroCurve],
    width: f64,
    dbg: &mut E,
) -> Result<EvalValue<E::Id>, EvalError<E::Id>> {
    // For each of the spiro curves, stroke them. This may result in
    // more than one spiro per original spiro.
    let id = dbg.preallocate_next();

    let mut out_spiros: Vec<SpiroCurve> = vec![];
    for spiro in eval_spiros {
        let oc = monoxide_curves::stroke::stroke_spiro(spiro, width, &mut dbg.curve_debugger(id));
        match oc {
            StrokeResult::One(spiro_cps) => {
                out_spiros.push(spiro_cps);
            }
            StrokeResult::Two(first_curve, second_curve) => {
                out_spiros.push(first_curve);
                out_spiros.push(second_curve);
            }
        }
    }

    let id = dbg.stroked(
        evaled_id,
        width,
        out_spiros.iter().map(|s| s.points.as_slice()),
    );

    if E::needs_evaluate_intermediate() {
        // Convert both original spiro and the stroked spiro to beziers
        let mut bezs = vec![];
        for spiro in eval_spiros {
            let bez = monoxide_curves::convert::spiro_to_cube(&spiro.points);
            bezs.extend(bez);
        }
        for spiro in &out_spiros {
            let bez = monoxide_curves::convert::spiro_to_cube(&spiro.points);
            bezs.extend(bez);
        }
        dbg.intermediate_output(id, &bezs);
    }

    Ok(EvalValue {
        kind: EvalValueKind::Spiros(out_spiros),
        id, // Use the ID from dbg.stroked
    })
}
