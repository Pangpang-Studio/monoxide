use std::{
    fmt::{self, Write},
    ops::Range,
};

use anyhow::Result;
use kurbo::{PathEl, Point};
use norad::{Contour, Glyph};

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

#[derive(Debug, Default, Clone)]
struct ViewBox {
    xs: Range<f64>,
    ys: Range<f64>,
}

impl ViewBox {
    fn merge_point(&mut self, point: &Point) {
        self.xs.start = self.xs.start.min(point.x);
        self.xs.end = self.xs.end.max(point.x);
        self.ys.start = self.ys.start.min(-point.y);
        self.ys.end = self.ys.end.max(-point.y);
    }

    fn merge_el(&mut self, el: &PathEl) {
        match el {
            PathEl::MoveTo(p) | PathEl::LineTo(p) => self.merge_point(p),
            PathEl::QuadTo(p, q) => {
                self.merge_point(p);
                self.merge_point(q);
            }
            PathEl::CurveTo(p, q, r) => {
                self.merge_point(p);
                self.merge_point(q);
                self.merge_point(r);
            }
            PathEl::ClosePath => (),
        }
    }

    fn merge_contour(&mut self, contour: &Contour) -> Result<()> {
        contour
            .to_kurbo()?
            .elements()
            .iter()
            .for_each(|el| self.merge_el(el));
        Ok(())
    }

    fn merge_glyph(&mut self, glyph: &Glyph) -> Result<()> {
        glyph
            .contours
            .iter()
            .try_for_each(|c| self.merge_contour(c))
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
    let glyph = Glyph::load("assets/R_.glif")?;
    let buf = String::new();
    let svg = {
        let mut pen = SvgPen { buf };
        pen.draw_glyph(&glyph)?;
        pen.finish()
    };

    let mut view_box = ViewBox::default();
    view_box.merge_glyph(&glyph)?;

    println!(
        r#"<!doctype html>
<html>
  <body>
    <div id="render">
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="{view_box}">
        <path
          d="{svg}"
          style="fill: none; stroke: gray; stroke-width: 3"
        />
        <defs>
          <style>
            svg {{
              position: fixed;
              top: 0;
              left: 0;
              height: 80%;
              width: 80%;
            }}
          </style>
        </defs>
      </svg>
    </div>
  </body>
</html>
    "#
    );
    Ok(())
}
