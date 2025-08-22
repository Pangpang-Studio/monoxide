mod model;
mod web;

use std::{fmt::Debug, sync::Arc};

use anyhow::{Result, anyhow};
use clap::Parser;
use dioxus_devtools::subsecond;
use futures_util::StreamExt;
use monoxide_script::{ast::FontContext, eval::layout_glyphs};
use tokio::sync::watch;
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
        let send = Arc::new(move || {
            _ = tx.try_send(());
        });
        subsecond::register_handler(send.clone());

        let _fut = match args.cmd {
            Subcommand::Serve(cmd) => tokio::spawn(web::start_web_server(cmd, render_rx)),
        };

        // Send an initial message to trigger the evaluation.
        send();

        while rx.next().await.is_some() {
            debug!("Evaluating playground...");
            let fcx = match make_font() {
                Ok(fcx) => fcx,
                Err(e) => {
                    send_error(e, &render_tx);
                    continue;
                }
            };
            let ser_fcx = match layout_glyphs(&fcx) {
                Ok(ser_fcx) => ser_fcx,
                Err(e) => {
                    send_error(e, &render_tx);
                    continue;
                }
            };
            debug!("Successfully evaluated playground");
            render_tx
                .send(Arc::new(web::RenderedFontState::Font(
                    Box::new(fcx),
                    Box::new(ser_fcx),
                )))
                .unwrap();
        }

        Ok(())
    }
}

fn send_error(e: impl Debug, render_tx: &watch::Sender<Arc<web::RenderedFontState>>) {
    tracing::error!("{e:?}");
    render_tx
        .send(Arc::new(web::RenderedFontState::Error(anyhow!("{e:?}"))))
        .unwrap();
}
