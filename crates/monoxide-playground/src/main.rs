mod svg;

use std::{borrow::Cow, fmt::Write as _, fs, path::PathBuf};

use anyhow::{Context as _, Result, anyhow};
use monoxide_script::{
    FontParamSettings,
    js::{ContextAttachment, MonoxideModule, insert_globals},
};
use rquickjs::{
    CatchResultExt, Context, Module, Runtime,
    loader::{BuiltinResolver, ModuleLoader},
};

use crate::svg::{Scale, SvgPen, ViewBox};

fn main() -> Result<()> {
    let rt = Runtime::new()?;
    let cx = Context::full(&rt)?;

    let mut module_resolver = BuiltinResolver::default();
    module_resolver.add_module("monoxide");

    let mut module_loader = ModuleLoader::default();
    module_loader.add_module("monoxide", MonoxideModule);
    rt.set_loader(module_resolver, module_loader);

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

        insert_globals(cx.clone()).unwrap();

        let m = Module::evaluate(
            cx.clone(),
            "font",
            r"
import { bezier, spiro, settings, glyph } from 'monoxide'

let g = glyph.simple(b => {
    b.add(
        bezier(0.3, 0)
            .lineTo(0.6, 0)
            .lineTo(1, settings.width)
            .lineTo(0.3, 0)
            .build()
    )
})
glyph.assignChar(g, 'c')
",
        )
        .catch(&cx)
        .map_err(|e| anyhow!("uncaught JavaScript exception: {e}"))?;
        m.finish::<()>().expect("failed to finish module");

        let ud = cx
            .userdata::<ContextAttachment>()
            .context("failed to retrieve ContextAttachment from Ctx")?;
        anyhow::Ok(ud.take())
    })?;

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
    println!("{}", playground_dir.display());
    Ok(())
}
