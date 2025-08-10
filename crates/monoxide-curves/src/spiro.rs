//! Represents a spiro curve with additional attributes.

use std::collections::HashMap;

use monoxide_spiro::SpiroCp;

use crate::stroke::Tangent;

/// Represents a curve made of Spiro control points.
///
/// The type can be used to represent a curve that will later be stroked, or
/// just a raw spiro curve. In the latter case, the additional attributes
/// (tangent and stroke) are not used.
#[derive(Debug, Clone, Default)]
pub struct SpiroCurve {
    pub is_closed: bool,
    pub points: Vec<SpiroCp>,

    pub tangents: HashMap<usize, Tangent>,

    /// The factor by which the width of the stroke is multiplied.
    /// If not provided, interpolates between surrounding points.
    ///
    /// If no points to interpolate, the value is the same as 1.0.
    pub width_factors: HashMap<usize, f64>,

    /// The alignment of the stroke at this point.
    ///
    /// If not provided, inherits the previous point. If no previous point to
    /// inherit, this is the same as `Aligned(0.5)`.
    ///
    /// When inheriting, this value inherits the abstract attribute, not the
    /// concrete value. For example, for a list of alignments `[1, interpolate,
    /// inherit, 0]` the resulting list of alignments will be `[1, interpolate,
    /// interpolate, 0]`, which may later resolved to something like `[1, 0.7,
    /// 0.3, 0]`, instead of first interpolating and then inherit like `[1, 0.7,
    /// 0.7, 0]`.
    pub alignment: HashMap<usize, StrokeAlignment>,
}

impl From<Vec<SpiroCp>> for SpiroCurve {
    fn from(points: Vec<SpiroCp>) -> Self {
        Self {
            points,
            ..Default::default()
        }
    }
}

pub fn default_width_factor() -> f64 {
    1.0
}

pub fn default_alignment() -> StrokeAlignment {
    StrokeAlignment::Aligned(0.5)
}

#[derive(Debug, Clone, PartialEq)]
pub enum StrokeAlignment {
    /// At the tangent at this point, the distance between the control point
    /// and the left edge of the stroke is `strokewidth * self.0`. This value
    /// it not limited to `0 <= x <= 1`, so that the stroke can be placed
    /// outside the curve if needed.
    ///
    /// Quick lookup:
    /// - Align the stroke to the right of the point -> 0
    /// - Middle aligned -> 0.5
    /// - Aligned the stroke to the left of the point -> 1
    ///
    /// ```plaintext
    /// | <- Stroke left edge
    /// |     ^ <- The curve being stroked, and its direction
    /// |     .        | <- Stroke right edge
    /// |     .        |
    /// *-----*--------*
    /// |     .        |
    /// |--x--|        |
    /// |-----w--------|
    ///
    /// self.0 = x / w
    /// ```
    Aligned(f64),

    /// Linear interpolate between surrounding points.
    ///
    /// Interpolates based on (approximated) curve length, not point count.
    Interpolate,
}

impl From<f64> for StrokeAlignment {
    fn from(value: f64) -> Self {
        StrokeAlignment::Aligned(value)
    }
}
