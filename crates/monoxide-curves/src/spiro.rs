//! Represents a spiro curve with additional attributes.

use std::collections::BTreeMap;

use monoxide_spiro::SpiroCp;

use crate::point::Point2D;

/// Represents a curve made of Spiro control points.
///
/// The type can be used to represent a curve that will later be stroked, or
/// just a raw spiro curve. In the latter case, the additional attributes
/// (tangent and stroke) are not used.
#[derive(Debug, Clone, Default)]
pub struct SpiroCurve {
    pub points: Vec<SpiroCp>,

    /// The overridden tangents at each point. This overrides both the in and
    /// out tangents at the point. (It makes no sense overriding only one, since
    /// that would make an angled corner with at least one edge with the wrong
    /// stroke width, which is almost always undesirable.)
    pub tangents: BTreeMap<usize, Point2D>,

    /// The factor by which the width of the stroke is multiplied.
    /// If not provided, interpolates between surrounding points.
    ///
    /// If no points to interpolate, the value is the same as 1.0.
    pub width_factors: BTreeMap<usize, f64>,

    /// The alignment of the stroke at this point.
    /// If not provided, interpolates between surrounding points.
    /// If no points to interpolate, the value is the same as 0.5.
    ///
    /// # Value interpretation
    ///
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
    pub alignment: BTreeMap<usize, f64>,
}

impl SpiroCurve {
    pub fn from_points(points: Vec<SpiroCp>, _is_closed: bool) -> Self {
        Self {
            points,
            ..Default::default()
        }
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
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

pub fn default_alignment() -> f64 {
    0.5
}
