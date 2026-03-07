mod model;
mod web;

use std::{fmt::Debug, fs, io::Write as _, path::PathBuf, sync::Arc};

use anyhow::{Result, anyhow};
use clap::Parser;
use dioxus_devtools::subsecond;
use flate2::{Compression, write::GzEncoder};
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
    Render(RenderCommand),
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
            Subcommand::Render(cmd) => cmd.run(&mut make_font).await,
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

#[derive(Debug, clap::Parser)]
pub struct RenderCommand {
    /// Compress format(s) to be used by metadata output.
    #[clap(long, value_delimiter = ',', default_value = "gz", value_enum)]
    meta_compress: Vec<MetaCompressKind>,

    /// The directory where generated files will be written.
    dir: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
enum MetaCompressKind {
    None,
    Gz,
}

impl RenderCommand {
    async fn run<E: Debug>(
        self,
        make_font: &mut impl FnMut() -> Result<FontContext, E>,
    ) -> Result<()> {
        let mut out_dir = self.dir;
        if !out_dir.is_absolute() {
            out_dir = std::env::current_dir()?.join(out_dir);
        };

        let compiled = CompiledFont::new(make_font)?;
        let ttf = compiled
            .ttf
            .map_err(|e| anyhow!("Font generation failed: {e}"))?;
        let metadata = serde_json::to_vec(&*compiled.metadata)?;

        fs::create_dir_all(&out_dir)?;

        let ttf_path = out_dir.join("monoxide.ttf");
        fs::write(&ttf_path, &ttf)?;

        info!("Wrote {}", ttf_path.display());

        if self.meta_compress.contains(&MetaCompressKind::None) {
            let metadata_path = out_dir.join("monoxide.ttf.meta");
            fs::write(&metadata_path, &metadata)?;
            info!("Wrote {}", metadata_path.display());
        }

        if self.meta_compress.contains(&MetaCompressKind::Gz) {
            let metadata_gz = compress_gzip(&metadata)?;
            let metadata_gz_path = out_dir.join("monoxide.ttf.meta.gz");
            fs::write(&metadata_gz_path, &metadata_gz)?;
            info!("Wrote {}", metadata_gz_path.display());
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

fn compress_gzip(payload: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(payload)?;
    let compressed = encoder.finish()?;
    Ok(compressed)
}
