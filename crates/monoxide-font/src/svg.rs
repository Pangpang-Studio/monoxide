use std::{
    fmt::{self, Write},
    mem,
    ops::Range,
};

use anyhow::Result;
use monoxide_curves::{CubicBezier, CubicSegment, point::Point2D};
use monoxide_script::{ast::GlyphEntry, eval::eval_outline};

pub struct SvgPen<W> {
    buf: W,
    scale: Scale,
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

impl<W: Write> SvgPen<W> {
    pub fn new(buf: W, scale: Scale) -> Self {
        Self { buf, scale }
    }

    fn draw_point(&mut self, p: &Point2D) -> fmt::Result {
        let sc = self.scale;
        writeln!(self.buf, " {} {}", sc.x * p.x, sc.y * p.y)
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

    pub fn draw_glyph(&mut self, glyph: &GlyphEntry) -> Result<()> {
        match glyph {
            GlyphEntry::Simple(s) => {
                for outline in &s.outlines {
                    let mut contours = vec![];
                    eval_outline(outline, &mut contours, &mut ());
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
        Self {
            xs: Range::default(),
            ys: Range::default(),
            scale,
        }
    }

    pub fn merge_point(&mut self, point: &Point2D) {
        let &Point2D { x, y } = point;

        self.xs.start = self.xs.start.min(x);
        self.xs.end = self.xs.end.max(x);

        self.ys.start = self.ys.start.min(y);
        self.ys.end = self.ys.end.max(y);
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    fn merge_contour(&mut self, contour: &CubicBezier<Point2D>) {
        self.merge_point(&contour.start);
        for el in &contour.segments {
            self.merge_el(el);
        }
    }

    #[allow(dead_code)]
    pub fn merge_glyph(&mut self, glyph: &GlyphEntry) -> Result<()> {
        match glyph {
            GlyphEntry::Simple(s) => {
                for outline in &s.outlines {
                    let mut contours = vec![];
                    eval_outline(outline, &mut contours, &mut ());
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
        let s = self.scale;
        let x0 = s.x * self.xs.start;
        let x1 = s.x * self.xs.end;
        let y0 = s.y * self.ys.start;
        let y1 = s.y * self.ys.end;
        write!(
            f,
            "{x} {y} {dx} {dy}",
            x = x0.min(x1),
            y = y0.min(y1),
            dx = (x1 - x0).abs(),
            dy = (y1 - y0).abs(),
        )
    }
}

pub struct SvgDebugPrinter {
    buf: String,
    scale: Scale,
}

impl SvgDebugPrinter {
    pub fn new(scale: Scale) -> Self {
        Self {
            buf: String::new(),
            scale,
        }
    }

    pub fn finish(self) -> String {
        self.buf
    }
}
