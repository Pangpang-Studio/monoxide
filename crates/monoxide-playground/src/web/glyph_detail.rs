use std::{fmt, sync::Arc};

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use monoxide_curves::{
    CubicBezier,
    debug::{CurveDebugger, DebugPointKind},
};
use monoxide_script::{
    ast::{FontContext, OutlineExpr},
    eval::{SerializedGlyphKind, eval_outline},
    prelude::*,
    trace::EvalTracer,
};
use monoxide_spiro::SpiroCp;

use super::XAppState;
use crate::model::{
    ConstructionKind, DebugLine, DebugPoint, GlyphDetail, GlyphOverview, Guideline, Guidelines,
    SerializedGlyphConstruction,
};

pub async fn glyph_detail(
    State(state): XAppState,
    Path(id): Path<usize>,
) -> Result<Json<GlyphDetail>, Response> {
    let latest_state = state.rx.borrow().clone();

    let (cx, ser_fcx) = match &*latest_state {
        crate::web::RenderedFontState::Nothing | crate::web::RenderedFontState::Error(_) => {
            return Err(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("No font loaded".into())
                .unwrap());
        }
        crate::web::RenderedFontState::Font(b) => (&b.defs, &b.ser_defs),
    };

    let glyph = ser_fcx.glyph_list.get(id).ok_or_else(|| {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Glyph not found".into())
            .unwrap()
    })?;

    let advance = glyph.advance.unwrap_or(cx.settings().mono_width());
    match &glyph.kind {
        SerializedGlyphKind::Simple(simple_glyph) => {
            let detail = simple_glyph_to_detail(id, cx, simple_glyph, advance);
            Ok(Json(detail))
        }
        SerializedGlyphKind::Compound(_compound_glyph) => {
            // we don't support compound glyphs yet
            Err(Response::builder()
                .status(StatusCode::NOT_IMPLEMENTED)
                .body("Compound glyphs are not supported yet".into())
                .unwrap())
        }
    }
}

fn simple_glyph_to_detail(
    id: usize,
    cx: &FontContext,
    outlines: &Vec<Arc<OutlineExpr>>,
    advance: f64,
) -> GlyphDetail {
    let mut tracer = GlyphDetailTracer::new();

    let mut output_outline = vec![];
    let mut out_ids = vec![];
    let mut out_errs = vec![];

    for outline in outlines {
        let id_or_err = eval_outline(outline, &mut output_outline, &mut tracer);
        match id_or_err {
            Ok(id) => out_ids.push(id),
            Err(err) => out_errs.push(err.to_string()),
        }
    }
    let output_id = tracer.boolean_added(&out_ids);
    tracer.intermediate_output(output_id, &output_outline);

    let overview = GlyphOverview {
        id,
        name: None,
        outline: output_outline,
        error: None,
        advance: cx.settings().mono_width(),
    };
    let guidelines = make_guidelines(cx, advance);

    GlyphDetail {
        overview,
        guidelines,
        construction: tracer.construction(),
        result_id: Some(output_id),
        errors: out_errs,
    }
}

fn make_guidelines(cx: &FontContext, advance: f64) -> Guidelines {
    let settings = cx.settings();
    Guidelines {
        h: vec![
            Guideline {
                pos: 0.0,
                label: Some("baseline".to_string()),
            },
            Guideline {
                pos: settings.x_height(),
                label: Some("x-height".to_string()),
            },
            Guideline {
                pos: settings.cap_height(),
                label: Some("cap-height".to_string()),
            },
            Guideline {
                pos: settings.descender(),
                label: Some("descender".to_string()),
            },
        ],
        v: vec![
            Guideline {
                pos: 0.0,
                label: None,
            },
            Guideline {
                pos: advance,
                label: Some("advance".to_string()),
            },
        ],
    }
}

struct GlyphDetailTracer {
    buf: Vec<SerializedGlyphConstruction>,
}

impl GlyphDetailTracer {
    fn new() -> Self {
        GlyphDetailTracer { buf: Vec::new() }
    }

    fn construction(self) -> Vec<SerializedGlyphConstruction> {
        self.buf
    }

    fn preallocated(&self) -> bool {
        self.buf
            .last()
            .is_some_and(|ser| matches!(ser.kind, ConstructionKind::Placeholder))
    }

    fn allocate_next(&mut self) -> (&mut SerializedGlyphConstruction, usize) {
        if !self.preallocated() {
            let id = self.buf.len();
            let kind = ConstructionKind::Placeholder;
            let ser = SerializedGlyphConstruction::new(id, kind);
            self.buf.push(ser);
        }
        let id = self.buf.len() - 1;
        (self.buf.last_mut().unwrap(), id)
    }
}

impl EvalTracer for GlyphDetailTracer {
    type CurveDebugger<'a> = TracerCurveDebugger<'a>;
    type Id = usize;

    fn needs_evaluate_intermediate() -> bool {
        true
    }

    fn preallocate_next(&mut self) -> Self::Id {
        self.allocate_next().1
    }

    fn constructed_beziers<'b>(
        &mut self,
        beziers: impl IntoIterator<Item = &'b CubicBezier<Point2D>>,
    ) -> Self::Id
    where
        Self: 'b,
    {
        let (ser, id) = self.allocate_next();
        let curve = beziers.into_iter().cloned().collect();
        ser.kind = ConstructionKind::CubicBezier { curve };
        id
    }

    fn constructed_spiros<'b>(
        &mut self,
        spiros: impl IntoIterator<Item = &'b [SpiroCp]>,
    ) -> Self::Id
    where
        Self: 'b,
    {
        let (ser, id) = self.allocate_next();
        let curve = spiros
            .into_iter()
            .map(|s| s.iter().cloned().map(|x| x.into()).collect())
            .collect();
        ser.kind = ConstructionKind::Spiro { curve };
        id
    }

    fn stroked<'b>(
        &mut self,
        parent: Self::Id,
        width: f64,
        spiros: impl IntoIterator<Item = &'b [SpiroCp]>,
    ) -> Self::Id
    where
        Self: 'b,
    {
        let (ser, id) = self.allocate_next();
        ser.kind = ConstructionKind::Stroke {
            parent,
            width,
            curve: spiros
                .into_iter()
                .map(|s| s.iter().cloned().map(|x| x.into()).collect())
                .collect(),
        };
        id
    }

    fn transformed<'b>(
        &mut self,
        parent: Self::Id,
        xform: &Affine2D<Point2D>,
        beziers: impl IntoIterator<Item = &'b CubicBezier<Point2D>>,
    ) -> Self::Id {
        let (ser, id) = self.allocate_next();
        ser.kind = ConstructionKind::Transform {
            parent,
            mov: xform.translation(),
            mat: xform.matrix(),
            curve: beziers.into_iter().cloned().collect(),
        };
        id
    }

    fn spiro_to_bezier(&mut self, parent: Self::Id) -> Self::Id {
        let (ser, id) = self.allocate_next();
        ser.kind = ConstructionKind::SpiroToBezier { parent };
        id
    }

    fn boolean_added<'b>(&mut self, parents: impl IntoIterator<Item = &'b Self::Id>) -> Self::Id
    where
        Self: 'b,
    {
        let (ser, id) = self.allocate_next();
        ser.kind = ConstructionKind::BooleanAdd {
            parents: parents.into_iter().cloned().collect(),
        };
        id
    }

    fn curve_debugger(&mut self, id: Self::Id) -> Self::CurveDebugger<'_> {
        TracerCurveDebugger {
            item: self.buf.get_mut(id).unwrap(),
        }
    }

    fn intermediate_output(&mut self, id: Self::Id, curve: &[CubicBezier<Point2D>]) {
        let it = self.buf.get_mut(id).unwrap();
        it.result_curve = Some(curve.to_vec());
    }
}

struct TracerCurveDebugger<'a> {
    item: &'a mut SerializedGlyphConstruction,
}

impl CurveDebugger for TracerCurveDebugger<'_> {
    fn point(&mut self, kind: DebugPointKind, at: Point2D, tag: fmt::Arguments<'_>) {
        self.item.debug_points.push(DebugPoint {
            kind: kind_to_string(&kind),
            at,
            tag: tag.to_string(),
        });
    }

    fn line(&mut self, from: Point2D, to: Point2D, tag: fmt::Arguments<'_>) {
        self.item.debug_lines.push(DebugLine {
            from,
            to,
            tag: tag.to_string(),
        });
    }
}

fn kind_to_string(kind: &DebugPointKind) -> &'static str {
    match kind {
        DebugPointKind::Corner => "corner",
        DebugPointKind::Curve => "curve",
        DebugPointKind::Control => "control",
        DebugPointKind::Misc => "misc",
        DebugPointKind::Hidden => "hidden",
    }
}
