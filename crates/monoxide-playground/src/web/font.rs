use axum::{body::Bytes, extract::State};

use crate::web::XAppState;

/// Get the full contents of the compiled font file
pub fn font(State(state): XAppState) -> anyhow::Result<Bytes> {
    todo!()
}
