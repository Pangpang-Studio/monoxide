use monoxide_script::ast::Glyph;

use super::InputContext;

pub fn space(_cx: &InputContext) -> Glyph {
    Glyph::builder().build()
}
