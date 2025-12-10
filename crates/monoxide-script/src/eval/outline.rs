use itertools::{Itertools, chain};
use monoxide_curves::{CubicBezier, SpiroCurve, point::Point2D, stroke::StrokedSpiroCurve};

use crate::{ast::OutlineExpr, trace::EvalTracer};

pub type EvalResult<E, A = EvalValue<<E as EvalTracer>::Id>> =
    Result<A, EvalError<<E as EvalTracer>::Id>>;

pub fn eval_outline<E: EvalTracer>(
    expr: &OutlineExpr,
    out: &mut Vec<CubicBezier<Point2D>>,
    dbg: &mut E,
) -> EvalResult<E, E::Id> {
    let evaled = eval_outline_internal(expr, dbg)?;
    let id = match evaled.kind {
        EvalValueKind::Beziers(beziers) => {
            out.extend(beziers);
            evaled.id
        }
        EvalValueKind::Spiros(spiros) => {
            let output_size_before = out.len();
            let id = dbg.spiro_to_bezier(evaled.id);
            for spiro in spiros {
                let bez = monoxide_curves::convert::spiro_to_cube(&spiro.points)
                    .map_err(|e| EvalError::CurveError(id, e))?;
                out.extend(bez);
            }
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

impl<Id: Copy> EvalValue<Id> {
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

    pub fn force_bezier<E>(self, dbg: &mut E) -> EvalResult<E, Vec<CubicBezier<Point2D>>>
    where
        E: EvalTracer<Id = Id>,
    {
        Ok(match self.kind {
            EvalValueKind::Beziers(beziers) => beziers,
            EvalValueKind::Spiros(spiros) => {
                let mut beziers = vec![];
                let id = dbg.spiro_to_bezier(self.id);
                for spiro in &spiros {
                    let bez = monoxide_curves::convert::spiro_to_cube(&spiro.points)
                        .map_err(|e| EvalError::CurveError(id, e))?;
                    beziers.extend(bez);
                }
                dbg.intermediate_output(id, &beziers);
                beziers
            }
        })
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum EvalError<Id> {
    #[error("stroking a bezier is not supported yet; at {0}")]
    StrokingABezier(Id),

    #[error("curve evaluation error at {0}: {1}")]
    CurveError(Id, #[source] monoxide_curves::error::Error),
}

fn eval_outline_internal<E: EvalTracer>(expr: &OutlineExpr, dbg: &mut E) -> EvalResult<E> {
    match expr {
        OutlineExpr::Bezier(cubic_bezier) => {
            let id = dbg.constructed_bezier(cubic_bezier);
            Ok(EvalValue::bezier(cubic_bezier.clone(), id))
        }
        OutlineExpr::Spiro(spiro) => {
            let id = dbg.constructed_spiro(&spiro.points);

            if E::needs_evaluate_intermediate() {
                // convert to beziers if needed
                let bez = monoxide_curves::convert::spiro_to_cube(&spiro.points)
                    .map_err(|e| EvalError::CurveError(id, e))?;
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
        OutlineExpr::Transformed(expr, xform) => {
            let evaled = eval_outline_internal(expr, dbg)?;
            let bezier = evaled.force_bezier(dbg)?;
            let flips = xform.flips_direction();
            let xformed = bezier
                .iter()
                .map(|x| {
                    let xformed = x.xform(*xform);
                    if flips { xformed.reversed() } else { xformed }
                })
                .collect();

            let id = dbg.constructed_beziers(bezier.iter());
            Ok(EvalValue {
                kind: EvalValueKind::Beziers(xformed),
                id,
            })
        }
    }
}

fn eval_stroked<E: EvalTracer>(
    evaled_id: E::Id,
    eval_spiros: &[SpiroCurve],
    width: f64,
    dbg: &mut E,
) -> EvalResult<E> {
    // For each of the spiro curves, stroke them. This may result in
    // more than one spiro per original spiro.
    let id = dbg.preallocate_next();

    let mut out_spiros: Vec<SpiroCurve> = vec![];
    for spiro in eval_spiros {
        let oc = monoxide_curves::stroke::stroke_spiro(spiro, width, &mut dbg.curve_debugger(id))
            .map_err(|e| EvalError::CurveError(id, e))?;
        match oc {
            StrokedSpiroCurve::One(spiro_cps) => {
                out_spiros.push(spiro_cps);
            }
            StrokedSpiroCurve::Two(first_curve, second_curve) => {
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
        let bezs: Vec<_> = chain!(eval_spiros, &out_spiros)
            .map(|spiro| {
                monoxide_curves::convert::spiro_to_cube(&spiro.points)
                    .map_err(|e| EvalError::CurveError(id, e))
            })
            .flatten_ok()
            .try_collect()?;
        dbg.intermediate_output(id, &bezs);
    }

    Ok(EvalValue {
        kind: EvalValueKind::Spiros(out_spiros),
        id, // Use the ID from dbg.stroked
    })
}
