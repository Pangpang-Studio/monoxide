mod model;
mod svg;
mod web;

use std::{
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
use tokio::sync::watch;
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
