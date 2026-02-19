use axum::{
    body::Bytes,
    extract::State,
    http::{Response, StatusCode},
};

use crate::web::XAppState;

use super::RenderedFontState;

/// Get the full contents of the compiled font file
pub async fn font(State(state): XAppState) -> Result<Bytes, Response<String>> {
    let st = state.rx.borrow().clone();
    match &*st {
        RenderedFontState::Font(compiled_font) => match &compiled_font.ttf {
            Ok(ttf) => Ok(ttf.clone()),
            Err(e) => Err(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(format!("Font generation failed: {}", e))
                .unwrap()),
        },

        // Error cases
        RenderedFontState::Nothing => Err(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("Can't render because nothing has been compiled yet".into())
            .unwrap()),
        RenderedFontState::Error(error) => Err(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(format!("Can't render font because of {}", error))
            .unwrap()),
    }
}
