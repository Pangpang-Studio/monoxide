pub mod ast;
pub mod dsl;
pub mod eval;
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
///   descender _|________| width
/// ```
///
/// In other words, a regular glyph must be designed to follow the below
/// coordinate restrictions:
///
/// * 0 <= x <= width
/// * descender <= y <= ascender := 1 + descender
///
/// See: <http://designwithfontforge.com/en-US/The_EM_Square.html>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FontParamSettings {
    pub width: f64,
    pub x_height: f64,
    pub descender: f64,
    pub cap_height: f64,

    /// Primary stroke width.
    pub stroke_width: f64,

    /// Vertical overshoot for arcs.
    /// See: <http://designwithfontforge.com/en-US/Creating_o_and_n.html>
    pub overshoot: f64,

    /// Primary side bearing, a.k.a. the horizontal margin of glyphs.
    pub side_bearing: f64,

    /// Size of the dot in glyphs like 'i' and 'j'.
    pub dot_size: f64,
}

impl FontParamSettings {
    /// The full width of a half-width character.
    pub const fn wth(&self) -> f64 {
        self.width
    }

    /// The x-height.
    pub const fn xh(&self) -> f64 {
        self.x_height
    }

    /// The cap height.
    pub const fn cap(&self) -> f64 {
        self.cap_height
    }

    /// The descender.
    ///
    /// NOTE: This is usually a negative value.
    pub const fn dsc(&self) -> f64 {
        self.descender
    }

    /// The primary stroke width.
    pub const fn stw(&self) -> f64 {
        self.stroke_width
    }

    /// The overshoot.
    pub const fn ovs(&self) -> f64 {
        self.overshoot
    }

    /// The left side bearing.
    pub const fn sbl(&self) -> f64 {
        self.side_bearing
    }

    /// The dot size in glyphs like 'i' and 'j'.
    pub const fn dot(&self) -> f64 {
        self.dot_size
    }

    /// The horizontal midline of a half-width character.
    pub const fn mid(&self) -> f64 {
        self.wth() / 2.
    }

    /// The ascender.
    pub const fn asc(&self) -> f64 {
        1. + self.dsc()
    }

    /// The right side bearing.
    pub const fn sbr(&self) -> f64 {
        self.wth() - self.sbl()
    }

    /// The dot radius in glyphs like 'i' and 'j'.
    pub const fn dtr(&self) -> f64 {
        self.dot() / 2.
    }
}
