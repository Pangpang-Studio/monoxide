use std::{ops::Range, sync::Arc};

use monoxide_curves::point::Point2D;
use monoxide_script::{ast::OutlineExpr, curl, dsl::SpiroBuilder, flat, g4};

use super::math::mix;

/// Renders a rectangle formed by drawing a line between points `start` and
/// `end` and span it in the normal direction according to the given width.
pub fn rect(start: impl Into<Point2D>, end: impl Into<Point2D>, width: f64) -> Arc<OutlineExpr> {
    SpiroBuilder::new(false)
        .extend([flat!(start.into()), curl!(end.into())])
        .build()
        .stroked(width)
}

/// Renders a ring delimited within the given x and y ranges.
pub fn ring(xr: Range<f64>, yr: Range<f64>) -> Arc<OutlineExpr> {
    let x0 = xr.start;
    let x1 = xr.end;
    let y0 = yr.start;
    let y1 = yr.end;
    let xm = mix(x0, x1, 0.5);
    let ym = mix(y0, y1, 0.5);

    SpiroBuilder::new(true)
        .extend([g4!(x0, ym), g4!(xm, y0), g4!(x1, ym), g4!(xm, y1)])
        .build()
}
