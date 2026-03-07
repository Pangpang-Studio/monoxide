mod model;
mod web;

use std::{fmt::Debug, io::Write as _, path::PathBuf, sync::Arc};

use anyhow::{Result, anyhow};
use bytes::{BufMut, BytesMut};
use clap::Parser;
use dioxus_devtools::subsecond;
use flate2::{Compression, write::GzEncoder};
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
    Render(RenderCommand),
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
            Subcommand::Render(cmd) => run_render(cmd, &mut make_font).await,
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

async fn run_render<E: Debug>(
    cmd: RenderCommand,
    make_font: &mut impl FnMut() -> Result<FontContext, E>,
) -> Result<()> {
    let out_dir = if cmd.dir.is_absolute() {
        cmd.dir
    } else {
        std::env::current_dir()?.join(cmd.dir)
    };

    let compiled = compile_font(make_font)?;
    let ttf = compiled
        .ttf
        .map_err(|e| anyhow!("Font generation failed: {e}"))?;
    let metadata = serde_json::to_vec(&*compiled.metadata)?;

    std::fs::create_dir_all(&out_dir)?;

    let ttf_path = out_dir.join("monoxide.ttf");
    std::fs::write(&ttf_path, &ttf)?;

    info!("Wrote {}", ttf_path.display());

    if cmd.meta_compress.contains(&MetaCompressKind::None) {
        let metadata_path = out_dir.join("monoxide.ttf.meta");
        std::fs::write(&metadata_path, &metadata)?;
        info!("Wrote {}", metadata_path.display());
    }

    if cmd.meta_compress.contains(&MetaCompressKind::Gz) {
        let metadata_gz = compress_gzip(&metadata)?;
        let metadata_gz_path = out_dir.join("monoxide.ttf.meta.gz");
        std::fs::write(&metadata_gz_path, &metadata_gz)?;
        info!("Wrote {}", metadata_gz_path.display());
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

fn compress_gzip(payload: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(payload)?;
    let compressed = encoder.finish()?;
    Ok(compressed)
}
