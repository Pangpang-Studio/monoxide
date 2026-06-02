use std::{ops::Range, sync::Arc};

use monoxide_curves::point::Point2D;
use monoxide_script::{
    ast::OutlineExpr,
    curl,
    dsl::{IntoOutline, SpiroBuilder},
    flat, g4,
};

use super::dir::{Alignment, Dir};

/// A rectangle formed by drawing a line between points `start` and
/// `end` and span it in the normal direction according to the given width.
pub struct Rect {
    pub start: Point2D,
    pub end: Point2D,
    pub width: Option<f64>,
    pub align: Alignment,
}

impl Rect {
    pub fn new(start: impl Into<Point2D>, end: impl Into<Point2D>) -> Self {
        Self {
            start: start.into(),
            end: end.into(),
            width: None,
            align: Alignment::Middle,
        }
    }

    pub fn stroked(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }

    pub fn aligned(mut self, align: Alignment) -> Self {
        self.align = align;
        self
    }
}

impl IntoOutline for Rect {
    fn into_outline(self) -> Arc<OutlineExpr> {
        let mut res = SpiroBuilder::open()
            .insts([flat!(self.start).aligned(self.align), curl!(self.end)])
            .into_outline();

        if let Some(width) = self.width {
            res = res.stroked(width);
        }

        res
    }
}

/// A ring delimited within the given x and y ranges.
#[derive(Clone, Debug)]
pub struct Ring {
    pub xr: Range<f64>,
    pub yr: Range<f64>,
}

impl Ring {
    pub fn new(xr: Range<f64>, yr: Range<f64>) -> Self {
        Self { xr, yr }
    }

    pub fn at(center: impl Into<Point2D>, radii: impl Into<Point2D>) -> Self {
        let c = center.into();
        let r = radii.into();
        Self::new((c.x - r.x)..(c.x + r.x), (c.y - r.y)..(c.y + r.y))
    }
}

impl IntoOutline for Ring {
    fn into_outline(self) -> Arc<OutlineExpr> {
        let Range { start: x0, end: x1 } = self.xr;
        let Range { start: y0, end: y1 } = self.yr;

        let xm = x0.midpoint(x1);
        let ym = y0.midpoint(y1);

        SpiroBuilder::closed()
            .insts([g4!(x0, ym), g4!(xm, y0), g4!(x1, ym), g4!(xm, y1)])
            .into_outline()
    }
}

/// A slash delimited within the given x and y ranges.
///
/// If both ranges are increasing at the same time, this will produce a slash
/// from the bottom-left corner to the top-right one.
#[derive(Clone, Debug)]
pub struct Slash {
    pub xr: Range<f64>,
    pub yr: Range<f64>,
    pub aln: SlashAlignment,
    pub heading: Dir,
}

impl Slash {
    pub const DEFAULT_HEADING: Dir = Dir::U;

    pub fn new(xr: Range<f64>, yr: Range<f64>) -> Self {
        Self {
            xr,
            yr,
            aln: SlashAlignment::default(),
            heading: Self::DEFAULT_HEADING,
        }
    }

    pub fn with_aln(mut self, aln: impl Into<Option<SlashAlignment>>) -> Self {
        self.aln = aln.into().unwrap_or_default();
        self
    }

    pub fn with_heading(mut self, tip_dir: impl Into<Option<Dir>>) -> Self {
        self.heading = tip_dir.into().unwrap_or(Self::DEFAULT_HEADING);
        self
    }

    /// Turns the slash around, e.g. from the regular slash to a backslash.
    pub fn back(self) -> Self {
        let Range { start: x0, end: x1 } = self.xr;
        Self {
            xr: x1..x0,
            aln: self.aln.back(),
            ..self
        }
    }
}

impl IntoOutline for Slash {
    fn into_outline(self) -> Arc<OutlineExpr> {
        let Self {
            xr: Range {
                start: left,
                end: right,
            },
            yr: Range {
                start: bot,
                end: top,
            },
            aln,
            heading,
        } = self;

        SpiroBuilder::open()
            .insts([
                g4!(left, bot).heading(heading).aligned(aln.bot),
                g4!(right, top).heading(heading).aligned(aln.top),
            ])
            .into_outline()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SlashAlignment {
    pub bot: f64,
    pub top: f64,
}

impl SlashAlignment {
    pub const fn new(bot: f64, top: f64) -> Self {
        Self { bot, top }
    }

    /// Returns a symmetric alignment where the bottom and top points are
    /// aligned to `bot` and `1 - bot` respectively.
    pub const fn symm(bot: f64) -> Self {
        Self::new(bot, 1. - bot)
    }

    pub const fn back(self) -> Self {
        Self::new(1. - self.bot, 1. - self.top)
    }
}

impl Default for SlashAlignment {
    fn default() -> Self {
        Self::symm(0.)
    }
}
