use std::{
    borrow::Cow,
    fmt::{self, Write},
    fs,
    ops::Range,
    path::PathBuf,
};

use anyhow::{Context as _, Result, anyhow};
use monoxide_curves::{CubicBezier, CubicSegment, point::Point2D};
use monoxide_script::{
    FontParamSettings,
    ast::GlyphEntry,
    eval::eval_outline,
    js::{ContextAttachment, MonoxideModule, insert_globals},
};
use rquickjs::{
    CatchResultExt, Context, Module, Runtime,
    loader::{BuiltinResolver, ModuleLoader},
};

struct SvgPen<W> {
    buf: W,
    scale: Scale,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Scale {
    x: f64,
    y: f64,
}

impl Default for Scale {
    fn default() -> Self {
        Self { x: 1., y: -1. }
    }
}

impl<W: Write> SvgPen<W> {
    fn draw_point(&mut self, p: &Point2D) -> fmt::Result {
        let sc = self.scale;
        writeln!(self.buf, " {} {}", sc.x * p.x, sc.y * p.y)
    }

    fn draw_start(&mut self, p: &Point2D) -> fmt::Result {
        write!(self.buf, "M ")?;
        self.draw_point(p)
    }

    fn draw_close(&mut self) -> fmt::Result {
        writeln!(self.buf, "Z")
    }

    fn draw_el(&mut self, el: &CubicSegment<Point2D>) -> fmt::Result {
        match el {
            CubicSegment::Line(p) => {
                write!(self.buf, "L ")?;
                self.draw_point(p)
            }
            CubicSegment::Curve(p, q, r) => {
                write!(self.buf, "C ")?;
                for pt in [p, q, r] {
                    self.draw_point(pt)?;
                }
                Ok(())
            }
        }
    }

    fn draw_contour(&mut self, contour: &CubicBezier<Point2D>) -> fmt::Result {
        self.draw_start(&contour.start)?;
        for el in &contour.segments {
            self.draw_el(el)?
        }
        if contour.closed {
            self.draw_close()?;
        }
        Ok(())
    }

    fn draw_glyph(&mut self, glyph: &GlyphEntry) -> Result<()> {
        match glyph {
            GlyphEntry::Simple(s) => {
                for outline in &s.outlines {
                    let mut contours = vec![];
                    eval_outline(outline, &mut contours);
                    for contour in &contours {
                        self.draw_contour(contour)?;
                    }
                }
                Ok(())
            }
            GlyphEntry::Compound(_) => todo!(),
        }
    }
}

impl<W: Write + Default> SvgPen<W> {
    fn finish(&mut self) -> W {
        std::mem::take(&mut self.buf)
    }
}

#[derive(Debug, Clone)]
struct ViewBox {
    xs: Range<f64>,
    ys: Range<f64>,
    scale: Scale,
}

impl ViewBox {
    fn new(scale: Scale) -> Self {
        Self {
            xs: Range::default(),
            ys: Range::default(),
            scale,
        }
    }

    fn merge_point(&mut self, point: &Point2D) {
        let sc = self.scale;

        let x = sc.x * point.x;
        self.xs.start = self.xs.start.min(x);
        self.xs.end = self.xs.end.max(x);

        let y = sc.y * point.y;
        self.ys.start = self.ys.start.min(y);
        self.ys.end = self.ys.end.max(y);
    }

    fn merge_el(&mut self, el: &CubicSegment<Point2D>) {
        match el {
            CubicSegment::Line(p) => self.merge_point(p),
            CubicSegment::Curve(p, q, r) => {
                for pt in [p, q, r] {
                    self.merge_point(pt);
                }
            }
        }
    }

    fn merge_contour(&mut self, contour: &CubicBezier<Point2D>) {
        self.merge_point(&contour.start);
        for el in &contour.segments {
            self.merge_el(el);
        }
    }

    fn merge_glyph(&mut self, glyph: &GlyphEntry) -> Result<()> {
        match glyph {
            GlyphEntry::Simple(s) => {
                for outline in &s.outlines {
                    let mut contours = vec![];
                    eval_outline(outline, &mut contours);
                    for contour in &contours {
                        self.merge_contour(contour);
                    }
                }
                Ok(())
            }
            GlyphEntry::Compound(_) => todo!(),
        }
    }
}

impl fmt::Display for ViewBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Range { start: x0, end: x1 } = self.xs;
        let Range { start: y0, end: y1 } = self.ys;
        write!(f, "{x0} {y0} {} {}", x1 - x0, y1 - y0)
    }
}

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
            let mut pen = SvgPen { buf, scale };
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
