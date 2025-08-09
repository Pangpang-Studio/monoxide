use monoxide_curves::point::Point2D;
use monoxide_script::{
    ast::Glyph,
    curl,
    dsl::{IntoOutline, IntoOutlineExt, SpiroBuilder},
    flat, g4, let_settings,
};

use super::InputContext;
use crate::font::{dir::Dir, glyph::o::OShape, shape::Rect};

pub fn n(cx: &InputContext) -> Glyph {
    let_settings! { { mid, mih, ovs, sbl, stw, xh } = cx.settings(); }

    let hstw = stw / 2.;
    let sbl1 = sbl + hstw;

    Glyph::builder()
        .outline(Rect::new((sbl1, 0.), (sbl1, xh), stw))
        .outline(n_curl((mid, mih), (mid - sbl - hstw, mih - hstw), ovs).stroked(stw))
        .build()
}

fn n_curl(center: impl Into<Point2D>, radii: impl Into<Point2D>, ovs: f64) -> impl IntoOutline {
    let o_shape @ OShape {
        center: Point2D { x, y },
        radii: Point2D { x: rx, y: ry },
        ..
    } = OShape::new(center, radii, ovs);

    let mid_curve_w = o_shape.mid_curve_w();
    let mid_curve_h = o_shape.mid_curve_h();

    let y_hi = y + ry;

    SpiroBuilder::open().insts([
        // Right side
        flat!(x + rx, 0.),
        curl!(x + rx, y + ry / 3.),
        // Top arc
        g4!(x + mid_curve_w, y_hi - mid_curve_h / 2.),
        g4!(x, y_hi + ovs).width(1.),
        g4!(x - mid_curve_w, y_hi - mid_curve_h)
            .heading(Dir::L)
            .width(0.5),
    ])
}
