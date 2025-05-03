use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::model::GlyphDetail;

use super::XAppState;

pub async fn glyph_detail(
    State(state): XAppState,
    Path(id): Path<usize>,
) -> Result<Json<GlyphDetail>, StatusCode> {
    let latest_state = state.rx.borrow().clone();

    let cx = match &*latest_state {
        crate::web::RenderedFontState::Nothing | crate::web::RenderedFontState::Error(_) => {
            return Err(StatusCode::NOT_FOUND);
        }
        crate::web::RenderedFontState::Font(cx) => cx,
    };

    let glyph = cx.glyphs.get(id).ok_or(StatusCode::NOT_FOUND)?;

    match glyph {
        _ => {}
    }

    todo!()
}
