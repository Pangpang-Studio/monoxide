mod model;
mod web;

use std::{fmt::Debug, sync::Arc};

use anyhow::{Result, anyhow};
use bytes::{BufMut, BytesMut};
use clap::Parser;
use dioxus_devtools::subsecond;
use futures_util::StreamExt;
use monoxide_script::{
    ast::FontContext,
    eval::{AuxiliarySettings, SerializedFontContext, eval, layout_glyphs},
};
use tokio::sync::watch;
use tracing::{debug, info};

use crate::model::{GlyphOverview, PrebuiltMetadata};
use crate::web::CompiledFont;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Playground {
    #[clap(subcommand)]
    cmd: Subcommand,
}

#[derive(clap::Parser)]
pub enum Subcommand {
    Serve(web::ServerCommand),
}

impl Playground {
    pub async fn dispatch<E: Debug>(
        mut make_font: impl FnMut() -> Result<FontContext, E>,
    ) -> Result<()> {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(tracing_subscriber::filter::LevelFilter::INFO.into()),
            )
            .init();

        let args = Playground::parse();
        match args.cmd {
            Subcommand::Serve(cmd) => run_serve(cmd, &mut make_font).await,
        }
    }
}

async fn run_serve<E: Debug>(
    cmd: web::ServerCommand,
    make_font: &mut impl FnMut() -> Result<FontContext, E>,
) -> Result<()> {
    let (tx, rx) = tokio::sync::mpsc::channel::<()>(10);
    let mut rx = std::pin::pin!(tokio_stream::wrappers::ReceiverStream::new(rx));

    let (render_tx, render_rx) = watch::channel(Arc::new(web::RenderedFontState::Nothing));

    // Establish the connection to the subsecond launcher.
    dioxus_devtools::connect_subsecond();
    // Trigger an initial evaluation before registering the hot-reload handler.
    _ = tx.try_send(());
    let send = Arc::new(move || {
        _ = tx.try_send(());
    });
    subsecond::register_handler(send);

    let _fut = tokio::spawn(web::start_web_server(cmd, render_rx));

    while rx.next().await.is_some() {
        info!("Re-evaluating playground");
        match compile_font(make_font) {
            Ok(compiled) => {
                debug!("Successfully evaluated playground");
                render_tx
                    .send(Arc::new(web::RenderedFontState::Font(Box::new(compiled))))
                    .unwrap();
            }
            Err(e) => {
                send_error(e, &render_tx);
            }
        }
    }

    Ok(())
}

fn compile_font<E: Debug>(
    make_font: &mut impl FnMut() -> Result<FontContext, E>,
) -> Result<CompiledFont> {
    let fcx = make_font().map_err(|e| anyhow!("{e:?}"))?;
    let ser_fcx = layout_glyphs(&fcx)?;
    let metadata = build_prebuilt_metadata(&fcx, ser_fcx);

    let file = eval(
        &fcx,
        &AuxiliarySettings {
            point_per_em: 2048,
            font_name: "Monoxide".into(),
        },
    );
    let ttf = file
        .map(|f| {
            let mut out_ttf = BytesMut::new().writer();
            f.write(&mut out_ttf).expect("Writing to memory can't fail");
            out_ttf.into_inner().freeze()
        })
        .map_err(Into::into);

    Ok(CompiledFont {
        metadata: Box::new(metadata),
        ttf,
    })
}

fn build_prebuilt_metadata(fcx: &FontContext, ser_fcx: SerializedFontContext) -> PrebuiltMetadata {
    let SerializedFontContext {
        cmap, glyph_list, ..
    } = ser_fcx;

    let glyphs = glyph_list
        .iter()
        .enumerate()
        .map(|(i, glyph)| {
            let outline = web::ws::render_glyph_to_beziers(glyph);
            let (outline, error) = match outline {
                Ok(outline) => (outline, None),
                Err(e) => (vec![], Some(e.to_string())),
            };
            GlyphOverview {
                id: i,
                name: None,
                outline,
                error,
                advance: fcx.settings().mono_width(),
            }
        })
        .collect();

    let glyph_details = glyph_list
        .iter()
        .enumerate()
        .map(|(i, glyph)| web::glyph_detail::serialized_glyph_to_detail(i, fcx, glyph))
        .collect();

    PrebuiltMetadata {
        cmap,
        glyphs,
        glyph_details,
    }
}

fn send_error(e: impl Debug, render_tx: &watch::Sender<Arc<web::RenderedFontState>>) {
    tracing::error!("{e:?}");
    render_tx
        .send(Arc::new(web::RenderedFontState::Error(anyhow!("{e:?}"))))
        .unwrap();
}
