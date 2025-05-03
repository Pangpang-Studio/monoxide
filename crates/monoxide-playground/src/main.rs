mod model;
mod svg;
mod web;

use std::{
    borrow::Cow,
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{Context, Result, anyhow};
use clap::Parser;
use monoxide_script::{
    FontParamSettings,
    ast::FontContext,
    js::{ContextAttachment, MonoxideModule},
};
use notify::{RecursiveMode, Watcher};
use path_slash::PathExt;
use rquickjs::{
    CatchResultExt, Module, Runtime,
    loader::{BuiltinResolver, FileResolver, ModuleLoader, ScriptLoader},
};
use svg::SvgDebugPrinter;
use tokio::sync::watch;
use tracing::debug;
use web::RenderedFontState;

use crate::svg::{Scale, SvgPen, ViewBox};

#[derive(Parser)]
#[command(author, version, about)]
struct Playground {
    /// Optional serve mode with custom command.
    /// Use this flag directly to run a dev server with `vite`, or set it to
    /// `npx` to run `npx vite` instead (idem for `pnpx`).
    #[arg(long)]
    serve: Option<Option<String>>,

    /// The script directory to build the glyphs from.
    source: PathBuf,

    #[clap(subcommand)]
    cmd: Option<Subcommand>,
}

#[derive(clap::Parser)]
enum Subcommand {
    Serve(web::ServerCommand),
}

fn evaluate_playground(rt: &rquickjs::Runtime, source_dir: &Path) -> Result<FontContext> {
    // Glob all js files in source_dir
    let mut js_files = vec![];
    for f in glob::glob(&format!("{}/**/*.js", source_dir.display()))? {
        let f = f?;
        if f.components().any(|x| x.as_os_str() == "node_modules") {
            continue;
        }

        let contents = fs::read_to_string(&f)?;
        js_files.push((f, contents));
    }

    let cx = rquickjs::Context::full(rt).context("Can't create context")?;
    let fcx = cx.with(|cx| {
        let width = 0.5;
        let descender = 0.2;
        let x_height = 0.5;
        let cap_height = 0.7;
        let overshoot = x_height / 120.;
        let cx_att = ContextAttachment::new(
            cx.clone(),
            FontParamSettings {
                width,
                descender,
                x_height,
                cap_height,
                overshoot,
            },
        )
        .expect("Cannot create attachment");
        cx.store_userdata(cx_att).unwrap();

        // Add monoxide module
        let (_monoxide_module, p) =
            Module::evaluate_def::<MonoxideModule, _>(cx.clone(), "monoxide")?;
        p.finish::<()>()?;

        let modules = js_files
            .into_iter()
            .map(|(path, source)| {
                let m = Module::declare(cx.clone(), path.to_slash_lossy().into_owned(), source)
                    .catch(&cx)
                    .map_err(|e| anyhow!("{e:?}"))
                    .with_context(|| format!("Cannot create module {}", path.display()))?;
                Ok(m)
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        for it in modules {
            let (_m, p) = it
                .eval()
                .catch(&cx)
                .map_err(|e| anyhow!("{e}"))
                .context("unexpected JS exception")?;
            p.finish::<()>()
                .catch(&cx)
                .map_err(|e| anyhow!("{e}"))
                .context("failed to finish module")?;
            // m.into_declared()?;
        }

        let ud = cx
            .userdata::<ContextAttachment<'_>>()
            .context("failed to retrieve ContextAttachment from Ctx")?;
        anyhow::Ok(ud.take())
    })?;

    Ok(fcx)
}

fn render_glyphs_html(fcx: &FontContext, playground_dir: &Path) -> anyhow::Result<()> {
    // Generate individual glyph pages
    let scale = Scale::default();
    let mut glyph_links = String::new();

    let glyphs = fcx
        .cmap
        .iter()
        .map(|(ch, idx)| (ch, fcx.get_glyph(*idx).expect("glyph not found")));
    for (&ch, glyph) in glyphs {
        let buf = String::new();
        let mut dbg = SvgDebugPrinter::new(scale);
        let svg = {
            let mut pen = SvgPen::new(buf, scale);
            pen.draw_glyph(glyph, &mut dbg)?;
            pen.finish()
        };

        let mut view_box = ViewBox::new(scale);
        view_box.merge_point(&(0., 0.).into());
        view_box.merge_point(&(1., 1.).into());

        // Create individual glyph page
        let ord = ch as u32;
        fs::write(
            playground_dir.join(format!("char/{ord}.html")),
            format!(
                include_str!("../assets/glyph.html.rsstr"),
                view_box = view_box,
                svg = svg,
                char = ch,
                dbg = dbg.finish()
            ),
        )?;

        // Add link to index
        writeln!(
            glyph_links,
            r#"<a href="char/{ord}.html" class="glyph-link">{ch}</a>"#,
        )?;
    }

    // Generate index page
    fs::write(
        playground_dir.join("index.html"),
        format!(
            include_str!("../assets/index.html.rsstr"),
            glyph_links = glyph_links,
        ),
    )?;

    Ok(())
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

    let rt = Runtime::new()?;

    let file_resolver = FileResolver::default();
    let mut builtin_resolver = BuiltinResolver::default();
    builtin_resolver.add_module("monoxide");

    let script_loader = ScriptLoader::default();
    let mut module_loader = ModuleLoader::default();
    module_loader.add_module("monoxide", MonoxideModule);

    rt.set_loader(
        (file_resolver, builtin_resolver),
        (script_loader, module_loader),
    );

    let playground_dir = PathBuf::from_iter([
        env!("CARGO_MANIFEST_DIR"),
        "..", // "crates"
        "..", // "monoxide"
        "target",
        "monoxide",
        "playground",
    ]);
    let playground_dir = playground_dir
        .canonicalize()
        .map_or(Cow::Borrowed(&playground_dir), Cow::Owned);

    // Set up file watcher
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<_>| {
        if res.is_ok() {
            _ = tx.blocking_send(());
        }
    })?;
    watcher.watch(Path::new("font"), RecursiveMode::Recursive)?;

    let (render_tx, render_rx) = watch::channel(Arc::new(RenderedFontState::Nothing));

    // TODO: organize logic
    let _fut = if let Some(cmd) = args.cmd {
        let fut = match cmd {
            Subcommand::Serve(cmd) => tokio::spawn(web::start_web_server(cmd, render_rx)),
        };
        Some(fut)
    } else {
        None
    };

    loop {
        debug!("Evaluating playground...");
        let res = evaluate_playground(&rt, &args.source);
        match res {
            Ok(fcx) => {
                debug!("Successfully evaluated playground");
                render_tx
                    .send(Arc::new(RenderedFontState::Font(fcx)))
                    .unwrap();
            }
            Err(e) => {
                tracing::error!("Error: {e}");
                render_tx
                    .send(Arc::new(RenderedFontState::Error(e)))
                    .unwrap();
            }
        }
        rx.recv().await;
    }
}
