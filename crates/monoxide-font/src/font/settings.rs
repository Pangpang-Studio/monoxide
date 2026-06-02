use monoxide_curves::point::Point2D;
use monoxide_script::EvalSettings;

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
#[derive(Clone, Debug)]
pub struct FontParamSettings {
    pub width: f64,
    pub x_height: f64,
    pub descender: f64,
    pub cap_height: f64,

    /// Primary stroke width.
    pub stroke_width: f64,

    /// Overshoot factor for arcs.
    /// For vertical overshoot, this is the ratio between the overshoot and the
    /// x-height; for horizontal overshoot, this is the ratio between the
    /// overshoot and the width between the left and right side bearings.
    /// See: <http://designwithfontforge.com/en-US/Creating_o_and_n.html>
    pub overshoot: f64,

    /// Primary side bearing, a.k.a. the horizontal margin of glyphs.
    pub side_bearing: f64,

    /// Size of the dot in glyphs like 'i' and 'j'.
    pub dot_size: f64,
}

/// Snapshot of the original and derived font parameters used by glyph builders.
#[derive(Clone, Copy, Debug)]
pub struct FontParamSettingsView {
    pub wth: f64,
    pub xh: f64,
    pub cap: f64,
    pub dsc: f64,
    pub stw: f64,
    pub ovf: f64,
    pub sbl: f64,
    pub dot: f64,
    pub ovs: f64,
    pub ovh: f64,
    pub mid: f64,
    pub mih: f64,
    pub asc: f64,
    pub sbr: f64,
    pub dtr: f64,
    pub lower_left: Point2D,
    pub lower_right: Point2D,
    pub lower_mid: Point2D,
    pub upper_left: Point2D,
    pub upper_right: Point2D,
    pub upper_mid: Point2D,
}

impl FontParamSettings {
    #[must_use]
    pub const fn view(&self) -> FontParamSettingsView {
        FontParamSettingsView {
            wth: self.wth(),
            xh: self.xh(),
            cap: self.cap(),
            dsc: self.dsc(),
            stw: self.stw(),
            ovf: self.ovf(),
            sbl: self.sbl(),
            dot: self.dot(),
            ovs: self.ovs(),
            ovh: self.ovh(),
            mid: self.mid(),
            mih: self.mih(),
            asc: self.asc(),
            sbr: self.sbr(),
            dtr: self.dtr(),
            lower_left: self.lower_left(),
            lower_right: self.lower_right(),
            lower_mid: self.lower_mid(),
            upper_left: self.upper_left(),
            upper_right: self.upper_right(),
            upper_mid: self.upper_mid(),
        }
    }

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

    /// The overshoot factor for arcs.
    pub const fn ovf(&self) -> f64 {
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

    /// The vertical overshoot.
    pub const fn ovs(&self) -> f64 {
        self.overshoot * self.xh()
    }

    /// The horizontal overshoot.
    pub const fn ovh(&self) -> f64 {
        self.overshoot * (self.sbr() - self.sbl())
    }

    /// The horizontal midline of a half-width character.
    pub const fn mid(&self) -> f64 {
        self.wth() / 2.
    }

    /// The vertical midline of the letter `x`.
    pub const fn mih(&self) -> f64 {
        self.xh() / 2.
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

    /// The lower left corner of the em square.
    pub const fn lower_left(&self) -> Point2D {
        Point2D::new(self.sbl(), 0.0)
    }

    /// The lower right corner of the em square.
    pub const fn lower_right(&self) -> Point2D {
        Point2D::new(self.sbr(), 0.0)
    }

    /// The lower mid point of the em square.
    pub const fn lower_mid(&self) -> Point2D {
        Point2D::new(self.mid(), 0.0)
    }

    /// The upper left corner of the em square.
    pub const fn upper_left(&self) -> Point2D {
        Point2D::new(self.sbl(), self.cap())
    }

    /// The upper right corner of the em square.
    pub const fn upper_right(&self) -> Point2D {
        Point2D::new(self.sbr(), self.cap())
    }

    /// The upper mid point of the em square.
    pub const fn upper_mid(&self) -> Point2D {
        Point2D::new(self.mid(), self.cap())
    }
}

impl EvalSettings for FontParamSettings {
    fn mono_width(&self) -> f64 {
        self.width
    }

    fn cap_height(&self) -> f64 {
        self.cap_height
    }

    fn descender(&self) -> f64 {
        self.descender
    }

    fn x_height(&self) -> f64 {
        self.x_height
    }
}
