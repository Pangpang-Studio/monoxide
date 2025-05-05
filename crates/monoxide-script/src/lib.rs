pub mod ast;
pub mod eval;
pub mod js;
pub mod trace;

use rquickjs::{Ctx, Object};
use serde::{Deserialize, Serialize};

/// Defines the basic em parameters for the font.
///
/// The em square in this project is defined as follows:
///
/// ```txt
///              y
///    ascender _|________|
///  cap_height _|        |
///    x_height _| ||  || |
///             _| ||==|| |
///           0 _|_||__||_|____ x
///   descender _|________| width
/// ```
///
/// In other words, in the **scripting language** (but not in the
/// `monoxide-script` APIs), a regular glyph must be designed to
/// follow the below coordinate restrictions:
///
/// - 0 <= x <= width
/// - descender <= y <= ascender := 1 + descender
///
/// See: <http://designwithfontforge.com/en-US/The_EM_Square.html>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FontParamSettings {
    pub width: f64,
    pub x_height: f64,
    pub descender: f64,
    pub cap_height: f64,

    /// Vertical overshoot for arcs.
    /// See: <http://designwithfontforge.com/en-US/Creating_o_and_n.html>
    pub overshoot: f64,

    /// Primary side bearing, a.k.a. the horizontal margin of glyphs.
    pub side_bearing: f64,
}

impl FontParamSettings {
    fn populate<'js>(&self, cx: Ctx<'js>) -> rquickjs::Result<Object<'js>> {
        let settings = Object::new(cx.clone())?;
        settings.prop("width", self.width)?;
        settings.prop("xHeight", self.x_height)?;
        settings.prop("descender", self.descender)?;
        settings.prop("capHeight", self.cap_height)?;
        settings.prop("overshoot", self.overshoot)?;
        settings.prop("sideBearing", self.side_bearing)?;
        Ok(settings)
    }
}
