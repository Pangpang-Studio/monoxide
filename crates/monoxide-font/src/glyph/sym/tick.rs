use std::sync::Arc;

use monoxide_script::prelude::*;

use crate::{InputContext, prelude::*};

pub fn apostrophe(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        mid,
        xh,
        dtr,
        stw,
        cap,
        ..
    } = cx.settings().view();

    Glyph::builder()
        .outline(Tick::new((mid, cap + dtr), (mid, xh - stw)).stroked(stw))
        .build()
}

/// A rectangle formed by drawing a line between points `start` and
/// `end` and span it in the normal direction according to the given width.
pub struct Tick {
    pub start: Point2D,
    pub end: Point2D,
    pub width: Option<f64>,
}

impl Tick {
    pub fn new(start: impl Into<Point2D>, end: impl Into<Point2D>) -> Self {
        Self {
            start: start.into(),
            end: end.into(),
            width: None,
        }
    }

    pub fn stroked(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }
}

impl IntoOutline for Tick {
    fn into_outline(self) -> Arc<OutlineExpr> {
        let mut res = SpiroBuilder::open()
            .insts([flat!(self.start).width(1.2), curl!(self.end).width(1.)])
            .into_outline();

        if let Some(width) = self.width {
            res = res.stroked(width);
        }

        res
    }
}
