use std::{f64::consts::TAU, sync::Arc};

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
pub fn quotes(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        mid,
        xh,
        dtr,
        stw,
        cap,
        ..
    } = cx.settings().view();

    let gap = 1.25 * stw;
    let left = mid - gap;
    let right = mid + gap;
    let top = cap + dtr;
    let bot = xh - stw;

    Glyph::builder()
        .outlines(
            [
                Tick::new((left, top), (left, bot)),
                Tick::new((right, top), (right, bot)),
            ]
            .map(|it| it.stroked(stw)),
        )
        .build()
}

pub fn asterisk(cx: &InputContext) -> Glyph {
    let FontParamSettingsView {
        mid, xh, cap, stw, ..
    } = cx.settings().view();

    let center = (mid, xh.midpoint(cap));
    let tick_h = 0.2;
    let tick = Tick::new(Point2D::from(center) - (0., tick_h), center)
        .stroked(stw)
        .into_outline();

    let petals = 5;
    let base_rot = TAU / petals as f64;
    Glyph::builder()
        .or_outlines((0..petals).map(|i| {
            let rot = Affine2D::rotated_around(center, i as f64 * base_rot);
            tick.clone().transformed(rot)
        }))
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
