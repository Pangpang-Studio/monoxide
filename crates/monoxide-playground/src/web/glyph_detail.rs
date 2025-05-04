use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use monoxide_curves::debug::CurveDebugger;
use monoxide_script::{ast::FontContext, eval::eval_outline, trace::EvaluationTracer};

use crate::{
    model::{
        ConstructionKind, DebugLine, DebugPoint, GlyphDetail, GlyphOverview,
        SerializedGlyphConstruction,
    },
    svg::{Scale, SvgPen},
};

use super::XAppState;

pub async fn glyph_detail(
    State(state): XAppState,
    Path(id): Path<usize>,
) -> Result<Json<GlyphDetail>, Response> {
    let latest_state = state.rx.borrow().clone();

    let cx = match &*latest_state {
        crate::web::RenderedFontState::Nothing | crate::web::RenderedFontState::Error(_) => {
            return Err(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("No font loaded".into())
                .unwrap());
        }
        crate::web::RenderedFontState::Font(cx) => cx,
    };

    let glyph = cx.glyphs.get(id).ok_or_else(|| {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Glyph not found".into())
            .unwrap()
    })?;

    match glyph {
        monoxide_script::ast::GlyphEntry::Simple(simple_glyph) => {
            let detail = simple_glyph_to_detail(id, &cx, simple_glyph);
            return Ok(Json(detail));
        }
        monoxide_script::ast::GlyphEntry::Compound(_compound_glyph) => {
            // we don't support compound glyphs yet
            return Err(Response::builder()
                .status(StatusCode::NOT_IMPLEMENTED)
                .body("Compound glyphs are not supported yet".into())
                .unwrap());
        }
    }
}

fn simple_glyph_to_detail(
    id: usize,
    cx: &FontContext,
    glyph: &monoxide_script::ast::SimpleGlyph,
) -> GlyphDetail {
    let mut tracer = GlyphDetailTracer::new();

    let mut output_outline = vec![];
    let mut out_ids = vec![];
    let mut out_errs = vec![];

    for outline in &glyph.outlines {
        let id_or_err = eval_outline(outline, &mut output_outline, &mut tracer);
        match id_or_err {
            Ok(id) => out_ids.push(id),
            Err(err) => out_errs.push(err.to_string()),
        }
    }
    let output_id = tracer.boolean_added(&out_ids);
    tracer.intermediate_output(output_id, &output_outline);

    let out_outlines = output_outline
        .into_iter()
        .map(|outline| {
            let mut s = String::new();
            SvgPen::new(&mut s, Scale::default())
                .draw_contour(&outline)
                .unwrap();
            s
        })
        .collect();

    let char_mapped = cx
        .cmap
        .iter()
        .find_map(|(ch, ch_id)| if ch_id.0 == id { Some(*ch) } else { None });

    let overview = GlyphOverview {
        id,
        ch: char_mapped,
        name: None,
        outline: out_outlines,
    };

    GlyphDetail {
        overview,
        construction: tracer.construction(),
        result_id: Some(output_id),
        errors: out_errs,
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
        self.buf.last().map_or(false, |ser| {
            matches!(ser.kind, ConstructionKind::Placeholder)
        })
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

impl EvaluationTracer for GlyphDetailTracer {
    type Id = usize;

    type CurveDebugger<'a> = TracerCurveDebugger<'a>;

    fn needs_evaluate_intermediate() -> bool {
        true
    }

    fn preallocate_next(&mut self) -> Self::Id {
        self.allocate_next().1
    }

    fn constructed_beziers<'b>(
        &mut self,
        bezier: impl IntoIterator<
            Item = &'b monoxide_curves::CubicBezier<monoxide_curves::point::Point2D>,
        >,
    ) -> Self::Id
    where
        Self: 'b,
    {
        let (ser, id) = self.allocate_next();
        let curve = bezier.into_iter().cloned().collect();
        ser.kind = ConstructionKind::CubicBezier { curve };
        id
    }

    fn constructed_spiros<'b>(
        &mut self,
        spiros: impl IntoIterator<Item = &'b [monoxide_spiro::SpiroCp]>,
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
        spiros: impl IntoIterator<Item = &'b [monoxide_spiro::SpiroCp]>,
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

    fn curve_debugger<'a>(&'a mut self, id: Self::Id) -> Self::CurveDebugger<'a> {
        TracerCurveDebugger {
            item: self.buf.get_mut(id).unwrap(),
        }
    }

    fn intermediate_output(
        &mut self,
        id: Self::Id,
        curve: &[monoxide_curves::CubicBezier<monoxide_curves::point::Point2D>],
    ) {
        let it = self.buf.get_mut(id).unwrap();
        it.result_curve = Some(curve.iter().cloned().collect());
    }
}

struct TracerCurveDebugger<'a> {
    item: &'a mut SerializedGlyphConstruction,
}

impl<'a> CurveDebugger for TracerCurveDebugger<'a> {
    fn point(
        &mut self,
        kind: monoxide_curves::debug::DebugPointKind,
        at: monoxide_curves::point::Point2D,
        tag: std::fmt::Arguments<'_>,
    ) {
        self.item.debug_points.push(DebugPoint {
            kind: kind_to_string(&kind),
            at,
            tag: tag.to_string(),
        });
    }

    fn line(
        &mut self,
        from: monoxide_curves::point::Point2D,
        to: monoxide_curves::point::Point2D,
        tag: std::fmt::Arguments<'_>,
    ) {
        self.item.debug_lines.push(DebugLine {
            from,
            to,
            tag: tag.to_string(),
        });
    }
}

fn kind_to_string(kind: &monoxide_curves::debug::DebugPointKind) -> &'static str {
    match kind {
        monoxide_curves::debug::DebugPointKind::Corner => "corner",
        monoxide_curves::debug::DebugPointKind::Curve => "curve",
        monoxide_curves::debug::DebugPointKind::Control => "control",
        monoxide_curves::debug::DebugPointKind::Misc => "misc",
        monoxide_curves::debug::DebugPointKind::Hidden => "hidden",
    }
}
