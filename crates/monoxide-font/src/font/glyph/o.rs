use monoxide_curves::point::Point2D;
use monoxide_script::{
    ast::{FontContext, SimpleGlyph},
    curl,
    dsl::{IntoOutline, IntoOutlineExt, SpiroBuilder},
    flat, g4,
};

pub fn o(fcx: &FontContext) -> SimpleGlyph {
    let s = fcx.settings();

    let mid = s.mid();
    let mih = s.mih();
    let ovs = s.ovs();
    let sbl = s.sbl();
    let stw = s.stw();

    let hstw = stw / 2.;

    SimpleGlyph::new()
        .outline(o_shape((mid, mih), (mid - sbl - hstw, mih - hstw), ovs).stroked(stw))
}

pub fn o_shape(
    center: impl Into<Point2D>,
    radii: impl Into<Point2D>,
    ovs: f64,
) -> impl IntoOutline {
    let Point2D { x, y } = center.into();
    let Point2D { x: rx, y: ry } = radii.into();

    let mid_curve_w = 0.85 * rx;
    let mid_curve_h = (5. / 16.) * ry;
    let end_curve_h = (13. / 16.) * ry;

    let y_hi = y + ry;
    let y_lo = y - ry;

    SpiroBuilder::closed().insts([
        // Bottom arc
        g4!(x - mid_curve_w, y_lo + mid_curve_h,),
        g4!(x, y_lo - ovs),
        g4!(x + mid_curve_w, y_lo + mid_curve_h),
        // Right side
        flat!(x + rx, y_lo + end_curve_h),
        curl!(x + rx, y_hi - end_curve_h),
        // Top arc
        g4!(x + mid_curve_w, y_hi - mid_curve_h),
        g4!(x, y_hi + ovs),
        g4!(x - mid_curve_w, y_hi - mid_curve_h),
        // Left side
        flat!(x - rx, y_hi - end_curve_h),
        curl!(x - rx, y_lo + end_curve_h),
    ])
}
