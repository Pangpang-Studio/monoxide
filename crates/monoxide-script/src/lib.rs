pub mod ast;
pub mod eval;
pub mod js;
pub mod trace;

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
///  -descender _|________| width
/// ```
///
/// In other words, in the **scripting language** (but not in the
/// `monoxide-script` APIs), a regular glyph must be designed to
/// follow the below coordinate restrictions:
///
/// - 0 <= x <= width
/// - -descender <= y <= ascender := 1 - descender
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
}
