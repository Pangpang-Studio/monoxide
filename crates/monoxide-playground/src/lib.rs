mod model;
mod web;

use std::{fmt::Debug, sync::Arc};

use anyhow::{Result, anyhow};
use clap::Parser;
use dioxus_devtools::subsecond;
use futures_util::StreamExt;
use monoxide_script::ast::FontContext;
use tokio::sync::watch;
use tracing::{debug, info};

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
            Subcommand::Serve(cmd) => cmd.run(&mut make_font).await,
        }
    }
}

impl web::ServerCommand {
    async fn run<E: Debug>(
        self,
        make_font: &mut impl FnMut() -> Result<FontContext, E>,
    ) -> Result<()> {
        let (tx, rx) = tokio::sync::mpsc::channel::<()>(10);
        let mut rx = std::pin::pin!(tokio_stream::wrappers::ReceiverStream::new(rx));

        let (render_tx, render_rx) = watch::channel(Arc::new(web::RenderedFontState::Nothing));

        // Establish the connection to the subsecond launcher.
        dioxus_devtools::connect_subsecond();

        let send = Arc::new(move || {
            _ = tx.try_send(());
        });

        // Trigger an initial evaluation and register the hot-reload handler.
        send();
        subsecond::register_handler(send);

        let _fut = tokio::spawn(self.start_web_server(render_rx));

        while rx.next().await.is_some() {
            info!("Re-evaluating playground");
            match CompiledFont::new(make_font) {
                Ok(compiled) => {
                    debug!("Successfully evaluated playground");
                    render_tx
                        .send(Arc::new(web::RenderedFontState::Font(Box::new(compiled))))
                        .unwrap();
                }
                Err(e) => send_error(e, &render_tx),
            }
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
