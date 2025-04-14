use std::{
    fmt::{self, Write},
    mem,
    ops::Range,
};

use anyhow::Result;
use monoxide_curves::{
    CubicBezier, CubicSegment,
    debug::{CurveDebugger, DebugPointKind},
    point::Point2D,
};
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

    fn draw_el(&mut self, el: &CubicSegment<Point2D>, dbg: &mut impl CurveDebugger) -> fmt::Result {
        match el {
            CubicSegment::Line(p) => {
                write!(self.buf, "L")?;
                dbg.point(DebugPointKind::Corner, *p, "");
                self.draw_point(p)
            }
            CubicSegment::Curve(p, q, r) => {
                write!(self.buf, "C")?;
                for pt in [p, q, r] {
                    self.draw_point(pt)?;
                }
                dbg.point(DebugPointKind::Control, *p, "");
                dbg.point(DebugPointKind::Control, *q, "");
                dbg.point(DebugPointKind::Corner, *r, "");

                Ok(())
            }
        }
    }

    fn draw_contour(
        &mut self,
        contour: &CubicBezier<Point2D>,
        dbg: &mut impl CurveDebugger,
    ) -> fmt::Result {
        self.draw_start(&contour.start)?;
        dbg.point(DebugPointKind::Corner, contour.start, "start");

        for el in &contour.segments {
            self.draw_el(el, dbg)?
        }
        if contour.closed {
            self.draw_close()?;
        }
        Ok(())
    }

    pub fn draw_glyph(&mut self, glyph: &GlyphEntry, dbg: &mut impl CurveDebugger) -> Result<()> {
        match glyph {
            GlyphEntry::Simple(s) => {
                for outline in &s.outlines {
                    let mut contours = vec![];
                    eval_outline(outline, &mut contours, dbg);
                    for contour in &contours {
                        self.draw_contour(contour, dbg)?;
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

impl CurveDebugger for SvgDebugPrinter {
    fn point(&mut self, kind: DebugPointKind, at: Point2D, tag: &str) {
        let sc = self.scale;
        let x = sc.x * at.x;
        let y = sc.y * at.y;
        let size = 0.01;

        let element = match kind {
            DebugPointKind::Corner => format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" fill="white" stroke="black" stroke-width="0.003" />"#,
                x - size / 2.0,
                y - size / 2.0,
                size,
                size
            ),
            DebugPointKind::Curve => format!(
                r#"<circle cx="{}" cy="{}" r="{}" fill="white" stroke="black" stroke-width="0.003" />"#,
                x,
                y,
                size / 2.0
            ),
            DebugPointKind::Control => format!(
                r#"<circle cx="{}" cy="{}" r="{}" fill="white" stroke="blue" stroke-width="0.003" />"#,
                x,
                y,
                size / 2.0
            ),
            DebugPointKind::Misc => format!(
                r#"<circle cx="{}" cy="{}" r="{}" fill="white" stroke="gray" stroke-width="0.003" />"#,
                x,
                y,
                size / 2.0
            ),
        };
        // Add tag text to the right of the point
        let text = if !tag.is_empty() {
            format!(
                r#"<text x="{}" y="{}" font-size="0.01" fill="black" text-anchor="start" dominant-baseline="middle">{}</text>"#,
                x + size,
                y,
                tag
            )
        } else {
            String::new()
        };

        writeln!(self.buf, "{}", element).unwrap();
        if !text.is_empty() {
            writeln!(self.buf, "{}", text).unwrap();
        }
    }

    fn line(&mut self, from: Point2D, to: Point2D, tag: &str) {
        let sc = self.scale;
        let x1 = sc.x * from.x;
        let y1 = sc.y * from.y;
        let x2 = sc.x * to.x;
        let y2 = sc.y * to.y;

        let line = format!(
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="gray" stroke-width="0.003" />"#,
            x1, y1, x2, y2
        );

        // Add tag as text near the middle of the line if present
        let text = if !tag.is_empty() {
            let mid_x = (x1 + x2) / 2.0;
            let mid_y = (y1 + y2) / 2.0;
            format!(
                r#"<text x="{}" y="{}" font-size="0.01" fill="black" text-anchor="middle" dominant-baseline="hanging">{}</text>"#,
                mid_x, mid_y, tag
            )
        } else {
            String::new()
        };

        writeln!(self.buf, "{}", line).unwrap();
        if !text.is_empty() {
            writeln!(self.buf, "{}", text).unwrap();
        }
    }
}
