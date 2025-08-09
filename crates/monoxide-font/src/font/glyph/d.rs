use monoxide_script::{ast::Glyph, dsl::IntoOutlineExt, let_settings};

use super::InputContext;
use crate::font::{glyph::o::OShape, shape::Rect};

pub fn d(cx: &InputContext) -> Glyph {
    let_settings! { { cap, mid, mih, ovs, sbl, sbr, stw } = cx.settings(); }

    let hstw = stw / 2.;

    let o_mid = mid;
    let o_radius_x = o_mid - (sbl + hstw);
    Glyph::builder()
        .outline(OShape::new((o_mid, mih), (o_radius_x, mih - hstw), ovs).stroked(stw))
        .outline(Rect::new((sbr - hstw, 0.), (sbr - hstw, cap), stw))
        .build()
}
