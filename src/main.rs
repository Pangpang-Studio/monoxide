use std::fmt::Write;

use anyhow::Result;
use kurbo::PathEl;
use norad::{Contour, Glyph};
use url::Url;

struct SvgPen<W> {
    buf: W,
}

impl<W: Write> SvgPen<W> {
    fn draw_el(&mut self, el: &PathEl) -> Result<()> {
        match el {
            PathEl::MoveTo(p) => writeln!(self.buf, "M {} {}", p.x, -p.y)?,
            PathEl::LineTo(p) => writeln!(self.buf, "L {} {}", p.x, -p.y)?,
            PathEl::QuadTo(p, q) => writeln!(self.buf, "Q {} {} {} {}", p.x, -p.y, q.x, -q.y)?,
            PathEl::CurveTo(p, q, r) => writeln!(
                self.buf,
                "C {} {} {} {} {} {}",
                p.x, -p.y, q.x, -q.y, r.x, -r.y
            )?,
            PathEl::ClosePath => writeln!(self.buf, "Z")?,
        }
        Ok(())
    }

    fn draw_contour(&mut self, contour: &Contour) -> Result<()> {
        contour
            .to_kurbo()?
            .elements()
            .iter()
            .try_for_each(|el| self.draw_el(el))
    }

    fn draw_glyph(&mut self, glyph: &Glyph) -> Result<()> {
        glyph.contours.iter().try_for_each(|c| self.draw_contour(c))
    }
}

impl<W: Write + Default> SvgPen<W> {
    fn finish(&mut self) -> W {
        std::mem::take(&mut self.buf)
    }
}

fn main() -> Result<()> {
    let glyph = Glyph::load("assets/R_.glif")?;
    let buf = String::new();
    let mut svg = {
        let mut pen = SvgPen { buf };
        pen.draw_glyph(&glyph)?;
        pen.finish()
    };
    let endpoint = Url::parse("https://svg-path-visualizer.netlify.app/")?.join({
        svg.insert(0, '#');
        &svg
    })?;
    opener::open_browser(endpoint.as_str())?;
    Ok(())
}
