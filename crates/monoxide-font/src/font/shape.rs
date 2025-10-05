use std::{ops::Range, sync::Arc};

use monoxide_curves::point::Point2D;
use monoxide_script::{
    ast::OutlineExpr,
    curl,
    dsl::{IntoOutline, SpiroBuilder},
    flat, g4,
};

use super::{dir::Alignment, math::mix};

/// A rectangle formed by drawing a line between points `start` and
/// `end` and span it in the normal direction according to the given width.
pub struct Rect {
    pub start: Point2D,
    pub end: Point2D,
    pub width: f64,
    pub align: Alignment,
}

impl Rect {
    pub fn new(start: impl Into<Point2D>, end: impl Into<Point2D>, width: f64) -> Self {
        Self {
            start: start.into(),
            end: end.into(),
            width,
            align: Alignment::Middle,
        }
    }

    pub fn aligned(self, align: Alignment) -> Self {
        Self { align, ..self }
    }
}

impl IntoOutline for Rect {
    fn into_outline(self) -> Arc<OutlineExpr> {
        SpiroBuilder::open()
            .insts([flat!(self.start).aligned(self.align), curl!(self.end)])
            .into_outline()
            .stroked(self.width)
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

        let xm = mix(x0, x1, 0.5);
        let ym = mix(y0, y1, 0.5);

        SpiroBuilder::closed()
            .insts([g4!(x0, ym), g4!(xm, y0), g4!(x1, ym), g4!(xm, y1)])
            .into_outline()
    }
}
