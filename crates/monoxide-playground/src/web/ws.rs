use std::{collections::BTreeMap, sync::Arc};

use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::Response,
};
use futures_util::{SinkExt, StreamExt, stream::SplitSink};
use serde::Serialize;
use tokio::sync::watch;
use tracing::{debug, info};

use super::{AppState, RenderedFontState, XAppState};
use crate::svg::{Scale, SvgPen};

#[derive(Serialize, Debug)]
#[serde(tag = "t")]
enum WsServerMsg {
    NewRendered(NewRenderedMsg),
    Error { msg: String },
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

pub async fn serve_ws(State(state): XAppState, ws: WebSocketUpgrade) -> Response {
    info!("WebSocket connection established");

    ws.on_upgrade(|socket| async move { handle_ws(state, socket).await })
}

async fn handle_ws(state: Arc<AppState>, ws: WebSocket) {
    let rx = state.rx.clone();
    let (ws_tx, ws_rx) = ws.split();

    let handle = tokio::spawn(send_ws_task(ws_tx, rx));
    let rx = tokio::spawn(ws_rx.for_each(|msg| async {
        match msg {
            Ok(msg) => {
                info!("Received message: {:?}", msg);
            }
            Err(e) => {
                info!("Error receiving message: {:?}", e);
            }
        }
    }));

    // Abort for either task if one of them fails
    tokio::select! {
        _ = handle => {},
        _ = rx => {},
    }
}

async fn send_ws_task(
    mut ws: SplitSink<WebSocket, Message>,
    mut rx: watch::Receiver<Arc<RenderedFontState>>,
) -> anyhow::Result<()> {
    loop {
        // this shit is probably send-only
        let val = rx.borrow_and_update().clone();
        match &*val {
            super::RenderedFontState::Nothing => {
                debug!("nothing received")
                // No-op for now
            }

            super::RenderedFontState::Font(font_context) => {
                // Maybe we shouldn't perform the render _here_, but YOLO so :)
                // TODO: Yeah we probably should move it out to web thread
                debug!("info received");

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
                        pen.draw_glyph(glyph)?;
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
                ws.send(Message::Text(msg.into())).await?;
            }

            super::RenderedFontState::Error(error) => {
                debug!("error received: {:?}", error);
                let msg = WsServerMsg::Error {
                    msg: format!("{:?}", error),
                };
                let msg = serde_json::to_string(&msg)?;
                ws.send(Message::Text(msg.into())).await?;
            }
        }

        rx.changed().await?;
        debug!("changed");
    }
}
