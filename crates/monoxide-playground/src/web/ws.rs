use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::Response,
};
use monoxide_script::ast::GlyphEntry;
use serde::Serialize;

use crate::svg::{Scale, SvgPen};

use super::{AppState, XAppState};

#[derive(Serialize, Debug)]
#[serde(tag = "t")]
enum WsServerMsg {
    NewRendered(NewRenderedMsg),
    Error(String),
}

#[derive(Serialize, Debug)]
struct NewRenderedMsg {
    glyphs: Vec<GlyphOverview>,
    cmap: BTreeMap<char, usize>,
}

#[derive(Serialize, Debug)]
struct GlyphOverview {
    svg: String,
    // viewport
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
}

#[axum::debug_handler(state = Arc<AppState>)]
pub async fn serve_ws(State(state): XAppState, ws: WebSocketUpgrade) -> Response {
    let run_loop = |mut ws: WebSocket| async move {
        let mut rx = state.rx.clone();

        loop {
            // this shit is probably send-only
            let val = rx.borrow_and_update().clone();
            match &*val {
                super::RenderedFontState::Nothing => {
                    // No-op for now
                }

                super::RenderedFontState::Font(font_context) => {
                    // Maybe we shouldn't perform the render _here_, but YOLO so :)
                    // TODO: Yeah we probably should move it out to web thread

                    let cmap = font_context
                        .cmap
                        .iter()
                        .map(|(ch, id)| (*ch, id.0))
                        .collect();
                    let scale = Scale::default();

                    let mut glyphs = vec![];
                    for glyph in &font_context.glyphs {
                        let buf = String::new();
                        let svg = {
                            let mut pen = SvgPen::new(buf, scale);
                            pen.draw_glyph(&glyph, &mut ())?;
                            pen.finish()
                        };
                        let glyph = GlyphOverview {
                            svg,
                            x0: 0.0,
                            y0: 0.0,
                            x1: 1.0,
                            y1: 1.0, // TODO
                        };
                        glyphs.push(glyph);
                    }

                    let msg = WsServerMsg::NewRendered(NewRenderedMsg { glyphs, cmap });
                    let msg = serde_json::to_string(&msg)?;
                    if let Err(e) = ws.send(Message::Text(msg.into())).await {
                        tracing::error!("Failed to send message: {e}");
                        break;
                    }
                }

                super::RenderedFontState::Error(error) => todo!(),
            }
        }

        Ok(())
    };

    ws.on_upgrade(move |socket| async move {
        let res: anyhow::Result<()> = run_loop(socket).await;
        if let Err(e) = res {
            tracing::error!("Error in WebSocket loop: {e}");
        }
    })
}
