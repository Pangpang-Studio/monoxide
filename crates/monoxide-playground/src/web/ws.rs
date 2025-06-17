use std::sync::Arc;

use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::Response,
};
use futures_util::{SinkExt, StreamExt, stream::SplitSink};
use monoxide_curves::{CubicBezier, point::Point2D};
use monoxide_script::eval::{eval_outline, layout_glyphs};
use serde::Serialize;
use tokio::sync::watch;
use tracing::{debug, info};

use super::{AppState, RenderedFontState, XAppState};
use crate::model::{FontOverview, GlyphOverview};

#[derive(Serialize, Debug)]
#[serde(tag = "t")]
enum WsServerMsg {
    /// Notify the client that new font data is going to arrive
    PrepareForNewEpoch,
    /// Send a single glyph to the client. This is to avoid having a too large
    /// WebSocket message
    Glyph(GlyphOverview),
    /// Notify the client that construction is now complete, and the client can
    /// flush the pending data to UI.
    EpochComplete(FontOverview),
    /// There's an error when evaluating the font
    Error { msg: String },
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

            super::RenderedFontState::Font(font_context, ser_font_context) => {
                // Maybe we shouldn't perform the render _here_, but YOLO so :)
                // TODO: Yeah we probably should move it out to web thread
                debug!("info received");

                ws.feed(Message::Text(
                    serde_json::to_string(&WsServerMsg::PrepareForNewEpoch)?.into(),
                ))
                .await?;

                for (i, glyph) in ser_font_context.glyph_list.iter().enumerate() {
                    let outline = render_glyph_to_beziers(glyph);
                    let (outline, error) = match outline {
                        Ok(outline) => (outline, None),
                        Err(e) => (vec![], Some(e.to_string())),
                    };
                    let glyph = GlyphOverview {
                        id: i,
                        name: None,
                        outline,
                        error,
                        advance: font_context.settings().width,
                    };
                    ws.feed(Message::Text(
                        serde_json::to_string(&WsServerMsg::Glyph(glyph))?.into(),
                    ))
                    .await?;
                }

                let overview = FontOverview {
                    cmap: ser_font_context.cmap.clone(),
                };

                ws.feed(Message::Text(
                    serde_json::to_string(&WsServerMsg::EpochComplete(overview))?.into(),
                ))
                .await?;

                ws.flush().await?;
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

fn render_glyph_to_beziers(
    glyph: &monoxide_script::ast::GlyphEntry,
) -> anyhow::Result<Vec<CubicBezier<Point2D>>> {
    let mut rendered = vec![];
    match glyph {
        monoxide_script::ast::GlyphEntry::Simple(simple_glyph) => {
            for outline in &simple_glyph.outlines {
                eval_outline(outline, &mut rendered, &mut ())?;
            }
        }
        monoxide_script::ast::GlyphEntry::Compound(_) => todo!(),
    }
    Ok(rendered)
}
