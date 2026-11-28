use itertools::Itertools;
use linesweeper::{BinaryOp, FillRule, binary_op};
use monoxide_curves::{
    CubicBezier, SpiroCurve,
    convert::spiro_to_cube,
    stroke::{stroke_bezier, stroke_spiro},
    xform::AffineExt,
};

use crate::{ast::OutlineExpr, trace::EvalTracer};

pub type EvalResult<E, A = EvalValue<<E as EvalTracer>::Id>> =
    Result<A, EvalError<<E as EvalTracer>::Id>>;

pub fn eval_outline<E: EvalTracer>(
    expr: &OutlineExpr,
    out: &mut Vec<CubicBezier>,
    dbg: &mut E,
) -> EvalResult<E, E::Id> {
    let (id, beziers) = eval_outline_internal(expr, dbg)?.force_bezier(dbg)?;
    out.extend(beziers);
    Ok(id)
}

/// Represents an intermediate value during the evaluation of a glyph.
#[derive(Debug, Clone)]
pub enum EvalValueKind {
    Beziers(Vec<CubicBezier>),
    Spiro(SpiroCurve),
}

#[derive(Debug, Clone)]
pub struct EvalValue<Id> {
    pub kind: EvalValueKind,
    pub id: Id,
}

impl<Id: Copy> EvalValue<Id> {
    pub fn bezier(bezier: CubicBezier, id: Id) -> Self {
        Self {
            kind: EvalValueKind::Beziers(vec![bezier]),
            id,
        }
    }

    pub fn spiro(spiro: SpiroCurve, id: Id) -> Self {
        Self {
            kind: EvalValueKind::Spiro(spiro),
            id,
        }
    }

    pub fn force_bezier<E>(self, dbg: &mut E) -> EvalResult<E, (E::Id, Vec<CubicBezier>)>
    where
        E: EvalTracer<Id = Id>,
    {
        Ok(match self.kind {
            EvalValueKind::Beziers(beziers) => (self.id, beziers),
            EvalValueKind::Spiro(spiro) => {
                let id = dbg.spiro_to_bezier(self.id);
                let beziers = eval_to_bez(id, &spiro, dbg)?;
                (id, beziers)
            }
        })
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum EvalError<Id> {
    #[error("boolean operation error at {0}: {1}")]
    Boolean(Id, #[source] linesweeper::Error),

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
            // convert to beziers if needed
            if E::needs_evaluate_intermediate() {
                eval_to_bez(id, spiro, dbg)?;
            }
            Ok(EvalValue::spiro(spiro.clone(), id))
        }
        OutlineExpr::Bool(op, lhs, rhs) => {
            let mut eval_half = |outline| {
                let evaled = eval_outline_internal(outline, dbg)?;
                let bez = match evaled.kind {
                    EvalValueKind::Beziers(bez) => bez,
                    EvalValueKind::Spiro(spiro) => eval_to_bez(evaled.id, &spiro, dbg)?,
                };
                Ok((evaled.id, bez))
            };
            eval_bool(*op, eval_half(lhs)?, eval_half(rhs)?, dbg)
        }
        OutlineExpr::Stroked(outline_expr, width) => {
            let evaled = eval_outline_internal(outline_expr, dbg)?;
            let id = dbg.stroked(evaled.id, *width);
            let result: Result<Vec<_>, _> = match evaled.kind {
                EvalValueKind::Beziers(curves) => curves
                    .iter()
                    .map(|c| stroke_bezier(c, *width))
                    .flatten_ok()
                    .try_collect(),
                EvalValueKind::Spiro(curve) => stroke_spiro(&curve, *width),
            };
            let outlines = result.map_err(|e| EvalError::CurveError(id, e))?;
            dbg.intermediate_output(id, &outlines);
            Ok(EvalValue {
                kind: EvalValueKind::Beziers(outlines),
                id,
            })
        }
        OutlineExpr::Transformed(expr, xform) => {
            let evaled = eval_outline_internal(expr, dbg)?;
            let (id, bezier) = evaled.force_bezier(dbg)?;
            let flips = xform.flips_direction();
            let xformed = bezier
                .iter()
                .map(|x| {
                    let xformed = x.xform(*xform);
                    if flips { xformed.reversed() } else { xformed }
                })
                .collect_vec();

            let id = dbg.transformed(id, xform, &xformed);
            dbg.intermediate_output(id, &xformed);
            Ok(EvalValue {
                id,
                kind: EvalValueKind::Beziers(xformed),
            })
        }
    }
}

fn eval_to_bez<E: EvalTracer>(
    id: E::Id,
    spiro: &SpiroCurve,
    dbg: &mut E,
) -> EvalResult<E, Vec<CubicBezier>> {
    let bez = spiro_to_cube(&spiro.points).map_err(|e| EvalError::CurveError(id, e))?;
    if E::needs_evaluate_intermediate() {
        dbg.intermediate_output(id, &bez);
    }
    Ok(bez)
}

fn eval_bool<E: EvalTracer>(
    op: BinaryOp,
    (lhs_id, lhs): (E::Id, Vec<CubicBezier>),
    (rhs_id, rhs): (E::Id, Vec<CubicBezier>),
    dbg: &mut E,
) -> EvalResult<E> {
    let id = dbg.boolean(op.into(), &[lhs_id, rhs_id]);
    let to_path = |contours: &[_]| contours.iter().flat_map(CubicBezier::to_kurbo).collect();
    let merged = binary_op(&to_path(&lhs), &to_path(&rhs), FillRule::NonZero, op)
        .map_err(|e| EvalError::Boolean(id, e))?
        .contours()
        // NOTE: We reverse the subpaths to avoid stacked beziers cancelling out each other.
        .map(|it| CubicBezier::from_kurbo(&it.path.reverse_subpaths()))
        .collect_vec();
    dbg.intermediate_output(id, &merged);
    Ok(EvalValue {
        kind: EvalValueKind::Beziers(merged),
        id,
    })
}
