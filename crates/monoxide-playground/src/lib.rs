mod model;
mod svg;
mod web;

use std::{fmt::Debug, path::PathBuf, sync::Arc};

use anyhow::{Result, anyhow};
use clap::Parser;
use dioxus_devtools::subsecond;
use monoxide_script::ast::FontContext;
use tokio::sync::watch;
use tokio_stream::StreamExt;
use tracing::debug;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Playground {
    /// Optional serve mode with custom command.
    /// Use this flag directly to run a dev server with `vite`, or set it to
    /// `npx` to run `npx vite` instead (idem for `pnpx`).
    #[arg(long)]
    serve: Option<Option<String>>,

    /// The script directory to build the glyphs from.
    #[clap(default_value = "font")]
    source: PathBuf,

    #[clap(subcommand)]
    cmd: Option<Subcommand>,
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

        let (render_tx, mut render_rx) = watch::channel(Arc::new(web::RenderedFontState::Nothing));

        // Establish the connection to the subsecond launcher.
        dioxus_devtools::connect_subsecond();
        subsecond::register_handler(Arc::new(move || {
            _ = tx.try_send(());
        }));

        // Start the web server if requested.
        let _fut = if let Some(cmd) = args.cmd {
            match cmd {
                Subcommand::Serve(cmd) => tokio::spawn(web::start_web_server(cmd, render_rx)),
            }
        } else {
            tokio::spawn(async move {
                loop {
                    render_rx.borrow_and_update();
                    if render_rx.changed().await.is_err() {
                        break Ok(());
                    }
                }
            })
        };

        loop {
            debug!("Evaluating playground...");
            match make_font() {
                Ok(fcx) => {
                    debug!("Successfully evaluated playground");
                    render_tx
                        .send(Arc::new(web::RenderedFontState::Font(fcx)))
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
