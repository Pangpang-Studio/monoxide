mod svg;

use std::{
    borrow::Cow,
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
    process::Stdio,
};

use anyhow::{Context, Result, anyhow, bail};
use clap::Parser;
use monoxide_script::{
    FontParamSettings,
    js::{ContextAttachment, MonoxideModule, insert_globals},
};
use notify::{RecursiveMode, Watcher};
use rquickjs::{
    CatchResultExt, Module, Runtime,
    loader::{BuiltinResolver, ModuleLoader},
};
use tokio::process::Command;

use crate::svg::{Scale, SvgPen, ViewBox};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Optional serve mode with custom command
    #[arg(long)]
    serve: Option<Option<String>>,

    /// The source directory to scan
    source: PathBuf,
}

/// Render glyphs to HTML files in the playground directory
fn render_glyphs(rt: &rquickjs::Runtime, source_dir: &Path, playground_dir: &Path) -> Result<()> {
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
        let cx_att = ContextAttachment::new(
            cx.clone(),
            FontParamSettings {
                width: 0.5,
                x_height: 0.6,
                descender: 0.2,
                cap_height: 1.,
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
                let m = Module::declare(cx.clone(), path.to_string_lossy().into_owned(), source)
                    .catch(&cx)
                    .map_err(|e| anyhow!("{:?}", e))
                    .with_context(|| format!("Cannot create module {}", path.display()))?;
                Ok(m)
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        for it in modules {
            let (m, p) = it
                .eval()
                .catch(&cx)
                .map_err(|e| anyhow::anyhow!(e.to_string()))
                .context("Unexpected JS exception")?;
            p.finish::<()>().expect("failed to finish module");
            // m.into_declared()?;
        }

        let ud = cx
            .userdata::<ContextAttachment>()
            .context("failed to retrieve ContextAttachment from Ctx")?;
        anyhow::Ok(ud.take())
    })?;

    // Generate individual glyph pages
    let scale = Scale::default();
    let mut glyph_links = String::new();

    let glyphs = fcx
        .cmap
        .iter()
        .map(|(ch, idx)| (ch, fcx.get_glyph(*idx).expect("glyph not found")));
    for (&ch, glyph) in glyphs {
        let buf = String::new();
        let svg = {
            let mut pen = SvgPen::new(buf, scale);
            pen.draw_glyph(glyph)?;
            pen.finish()
        };

        let mut view_box = ViewBox::new(scale);
        view_box.merge_glyph(glyph)?;

        // Create individual glyph page
        let ord = ch as u32;
        fs::write(
            playground_dir.join(format!("char/{ord}.html")),
            format!(
                include_str!("../assets/glyph.html.rsstr"),
                view_box = view_box,
                svg = svg,
                char = ch,
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
    let args = Args::parse();
    let rt = Runtime::new()?;

    // let mut module_resolver = BuiltinResolver::default();
    // module_resolver.add_module("monoxide");

    // let mut module_loader = ModuleLoader::default();
    // module_loader.add_module("monoxide", MonoxideModule);
    // rt.set_loader(module_resolver, module_loader);

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

    // Create playground and char directories
    fs::create_dir_all(&*playground_dir)?;
    fs::create_dir_all(playground_dir.join("char"))?;

    // Initial render
    render_glyphs(&rt, &args.source, &playground_dir)?;
    let Some(serve) = args.serve else {
        println!("{}", playground_dir.display());
        return Ok(());
    };

    // Set up file watcher
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    let mut watcher = notify::recommended_watcher(move |res| {
        if let Ok(_) = res {
            _ = tx.blocking_send(());
        }
    })?;
    watcher.watch(Path::new("font"), RecursiveMode::Recursive)?;

    // Spawn vite process
    let mut serve = match serve {
        None => Command::new("vite"),
        Some(cmd) => {
            let mut cmd = Command::new(cmd);
            cmd.arg("vite");
            cmd
        }
    };
    let mut child = serve
        .current_dir(dunce::simplified(&playground_dir))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    // Watch for changes and re-render
    loop {
        tokio::select! {
            _ = rx.recv() => render_glyphs(&rt, &args.source, &playground_dir)?,
            status = child.wait() => {
                let status = status?;
                if !status.success() {
                    bail!("dev server exited with error {status}");
                }
                break;
            }
        }
    }
    Ok(())
}
