use std::{
    fmt::{self, Write},
    mem,
    ops::Range,
};

use monoxide_curves::{CubicBezier, CubicSegment, point::Point2D};
use monoxide_script::{ast::Glyph, eval::eval_outline, let_settings};

use crate::make_font_params;

const PRECISION: usize = 4;

pub struct SvgPen<W> {
    buf: W,
    scale: Scale,
}

impl<W: Write> SvgPen<W> {
    pub fn new(buf: W, scale: Scale) -> Self {
        Self { buf, scale }
    }

    fn draw_point(&mut self, p: &Point2D) -> fmt::Result {
        let sc = self.scale;
        writeln!(
            self.buf,
            " {:.PRECISION$} {:.PRECISION$}",
            sc.x * p.x,
            sc.y * p.y
        )
    }

    fn draw_start(&mut self, p: &Point2D) -> fmt::Result {
        write!(self.buf, "M")?;
        self.draw_point(p)
    }

    fn draw_close(&mut self) -> fmt::Result {
        writeln!(self.buf, "Z")
    }

    fn draw_el(&mut self, el: &CubicSegment<Point2D>) -> fmt::Result {
        match el {
            CubicSegment::Line(p) => {
                write!(self.buf, "L")?;
                self.draw_point(p)
            }
            CubicSegment::Curve(p, q, r) => {
                write!(self.buf, "C")?;
                for pt in [p, q, r] {
                    self.draw_point(pt)?;
                }
                Ok(())
            }
        }
    }

    pub fn draw_contour(&mut self, contour: &CubicBezier<Point2D>) -> fmt::Result {
        self.draw_start(&contour.start)?;

        for el in &contour.segments {
            self.draw_el(el)?
        }
        if contour.closed {
            self.draw_close()?;
        }
        Ok(())
    }

    pub fn draw_glyph(&mut self, glyph: &Glyph) -> anyhow::Result<()> {
        for outline in &glyph.outlines {
            let mut contours = vec![];
            eval_outline(outline, &mut contours, &mut ()).unwrap();
            for contour in &contours {
                self.draw_contour(contour)?;
            }
        }
        Ok(())
    }
}

impl<W: Write + Default> SvgPen<W> {
    pub fn finish(&mut self) -> W {
        mem::take(&mut self.buf)
    }
}

#[derive(Debug, Clone)]
pub struct ViewBox {
    xs: Range<f64>,
    ys: Range<f64>,
    scale: Scale,
}

impl ViewBox {
    pub fn new(scale: Scale) -> Self {
        let_settings! { { dsc, asc } = make_font_params(); }

        Self {
            xs: (0.)..1.,
            ys: dsc..asc,
            scale,
        }
    }
}

impl fmt::Display for ViewBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.scale;
        let padding = 0.25;
        let x0 = s.x * (self.xs.start - padding);
        let x1 = s.x * (self.xs.end + padding);
        let y0 = s.y * (self.ys.start - padding);
        let y1 = s.y * (self.ys.end + padding);
        write!(
            f,
            "{x:.PRECISION$} {y:.PRECISION$} {dx:.PRECISION$} {dy:.PRECISION$}",
            x = x0.min(x1),
            y = y0.min(y1),
            dx = (x1 - x0).abs(),
            dy = (y1 - y0).abs(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scale {
    pub x: f64,
    pub y: f64,
}

impl Default for Scale {
    fn default() -> Self {
        Self { x: 1., y: -1. }
    }
}
