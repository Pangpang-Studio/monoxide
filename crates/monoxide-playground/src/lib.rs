mod model;
mod svg;
mod web;

use std::{fmt::Debug, sync::Arc};

use anyhow::{Result, anyhow};
use clap::Parser;
use dioxus_devtools::subsecond;
use monoxide_script::{ast::FontContext, eval::layout_glyphs};
use tokio::sync::watch;
use tokio_stream::StreamExt;
use tracing::debug;

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

        let (tx, rx) = tokio::sync::mpsc::channel::<()>(10);
        let mut rx = std::pin::pin!(tokio_stream::wrappers::ReceiverStream::new(rx));

        let (render_tx, render_rx) = watch::channel(Arc::new(web::RenderedFontState::Nothing));

        // Establish the connection to the subsecond launcher.
        dioxus_devtools::connect_subsecond();
        subsecond::register_handler(Arc::new(move || {
            _ = tx.try_send(());
        }));

        let _fut = match args.cmd {
            Subcommand::Serve(cmd) => tokio::spawn(web::start_web_server(cmd, render_rx)),
        };

        loop {
            debug!("Evaluating playground...");
            match make_font() {
                Ok(fcx) => {
                    debug!("Successfully evaluated playground");
                    let ser_fcx = layout_glyphs(&fcx);
                    render_tx
                        .send(Arc::new(web::RenderedFontState::Font(fcx, ser_fcx)))
                        .unwrap();
                }
                Err(e) => {
                    tracing::error!("{e:?}");
                    render_tx
                        .send(Arc::new(web::RenderedFontState::Error(anyhow!("{e:?}"))))
                        .unwrap();
                }
            }
            rx.next().await;
        }
    }
}
