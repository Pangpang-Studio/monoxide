mod model;
mod svg;
mod web;

use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use clap::Parser;
use dioxus_devtools::subsecond;
use monoxide_script::{FontParamSettings, ast::FontContext};
use tokio::sync::watch;
use tokio_stream::StreamExt;
use tracing::debug;
use web::RenderedFontState;

#[derive(Parser)]
#[command(author, version, about)]
struct Playground {
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
enum Subcommand {
    Serve(web::ServerCommand),
}

fn evaluate_playground() -> Result<FontContext> {
    let width = 0.5;
    let x_height = 0.5;

    let settings = FontParamSettings {
        width,
        x_height,
        descender: -0.2,
        cap_height: 0.7,
        side_bearing: 0.15 * width,
        overshoot: x_height / 50.,
    };

    let mut fcx = FontContext::new(settings);
    Ok(fcx)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing_subscriber::filter::LevelFilter::INFO.into()),
        )
        .init();

    let args = Playground::parse();

    let (tx, rx) = tokio::sync::mpsc::channel::<()>(10);
    let mut rx = std::pin::pin!(tokio_stream::wrappers::ReceiverStream::new(rx));

    let (render_tx, mut render_rx) = watch::channel(Arc::new(RenderedFontState::Nothing));

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
        let res = subsecond::call(evaluate_playground);
        match res {
            Ok(fcx) => {
                debug!("Successfully evaluated playground");
                render_tx
                    .send(Arc::new(RenderedFontState::Font(fcx)))
                    .unwrap();
            }
            Err(e) => {
                tracing::error!("{e:?}");
                render_tx
                    .send(Arc::new(RenderedFontState::Error(e)))
                    .unwrap();
            }
        }
        rx.next().await;
    }
}
