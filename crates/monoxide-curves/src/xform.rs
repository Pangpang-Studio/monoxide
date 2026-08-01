//! Affine transform utilities.
//!
//! This module is a thin wrapper around [`kurbo::Affine`]. [`Affine2D`] is
//! simply [`kurbo::Affine`]; [`AffineExt`] adds a handful of
//! domain-specific constructors and introspection helpers (matching names
//! used throughout the rest of this codebase, and font-matrix decomposition
//! helpers) that `kurbo` doesn't provide out of the box.

pub use kurbo::Affine as Affine2D;

use crate::point::Point2D;

/// Extension methods for [`Affine2D`] used throughout this crate and its
/// consumers, on top of what `kurbo` already provides.
pub trait AffineExt: Sized {
    /// Creates an identity affine transformation.
    fn id() -> Self;

    /// Creates an affine transform from a translation and a column-major 2x2
    /// matrix.
    fn make(translation: impl Into<Point2D>, matrix: [impl Into<Point2D>; 2]) -> Self;

    /// Create a transformation that translates the point by `translation`.
    fn translated(translation: impl Into<Point2D>) -> Self;

    /// Create a transformation that rotates the point by `angle`.
    fn rotated(angle: f64) -> Self;

    /// Create a transformation that rotates the point by `angle` around
    /// `center`.
    fn rotated_around(center: impl Into<Point2D>, angle: f64) -> Self;

    /// Create a transformation that scales the point by `scale`.
    fn scaled(scale: f64) -> Self;

    /// Create a transformation that reflects the point along the line that
    /// crosses `base` and goes in `direction`.
    fn mirrored_along(base: impl Into<Point2D>, direction: impl Into<Point2D>) -> Self;

    /// Reflects the input across the line through `point` in `direction`
    /// *before* applying `self`, i.e. the resulting transform is
    /// `self ∘ Reflect_line(point, direction)`.
    fn mirror_along(self, point: impl Into<Point2D>, direction: impl Into<Point2D>) -> Self;

    /// Applies this transformation to a point.
    fn apply(&self, point: &Point2D) -> Point2D;

    /// The translation component of the transform.
    fn translation(&self) -> Point2D;

    /// The scaling/rotation matrix, in column-major order.
    fn matrix(&self) -> [Point2D; 2];

    /// Returns whether the transform does not scale the point.
    fn scale_is_identity(&self) -> bool;

    /// Returns `Some(scale)` if the transformation's matrix part
    /// only contains a uniform scaling operation, `None` otherwise.
    fn scale_is_uniform(&self) -> Option<f64>;

    /// Returns `Some((scale_x, scale_y))` if the transformation's matrix part
    /// only contains a scaling operation, `None` otherwise. This function does
    /// not check the translation part.
    fn mat_is_only_scale(&self) -> Option<(f64, f64)>;

    /// Returns true if the linear part of the transform flips orientation
    /// (i.e. has a negative determinant). This indicates the transform
    /// mirrors/reflections compared to orientation-preserving transforms.
    fn flips_direction(&self) -> bool;
}

impl AffineExt for Affine2D {
    fn id() -> Self {
        Self::IDENTITY
    }

    fn make(translation: impl Into<Point2D>, matrix: [impl Into<Point2D>; 2]) -> Self {
        let translation = translation.into();
        let [col0, col1] = matrix.map(Into::into);
        Self::new([col0.x, col0.y, col1.x, col1.y, translation.x, translation.y])
    }

    fn translated(translation: impl Into<Point2D>) -> Self {
        Self::translate(translation.into().to_vec2())
    }

    fn rotated(angle: f64) -> Self {
        Self::rotate(angle)
    }

    fn rotated_around(center: impl Into<Point2D>, angle: f64) -> Self {
        Self::rotate_about(angle, center.into())
    }

    fn scaled(scale: f64) -> Self {
        Self::scale(scale)
    }

    fn mirrored_along(base: impl Into<Point2D>, direction: impl Into<Point2D>) -> Self {
        Self::reflect(base.into(), direction.into().to_vec2())
    }

    fn mirror_along(self, point: impl Into<Point2D>, direction: impl Into<Point2D>) -> Self {
        self.pre_reflect(point.into(), direction.into().to_vec2())
    }

    fn apply(&self, point: &Point2D) -> Point2D {
        *self * *point
    }

    fn translation(&self) -> Point2D {
        let [_, _, _, _, e, f] = self.as_coeffs();
        Point2D::new(e, f)
    }

    fn matrix(&self) -> [Point2D; 2] {
        let [a, b, c, d, _, _] = self.as_coeffs();
        [Point2D::new(a, b), Point2D::new(c, d)]
    }

    fn scale_is_identity(&self) -> bool {
        let [a, b, c, d, _, _] = self.as_coeffs();
        (a, b, c, d) == (1.0, 0.0, 0.0, 1.0)
    }

    fn scale_is_uniform(&self) -> Option<f64> {
        let [a, b, c, d, _, _] = self.as_coeffs();
        (a == d && b == 0.0 && c == 0.0).then_some(a)
    }

    fn mat_is_only_scale(&self) -> Option<(f64, f64)> {
        let [a, b, c, d, _, _] = self.as_coeffs();
        (b == 0.0 && c == 0.0).then_some((a, d))
    }

    fn flips_direction(&self) -> bool {
        self.determinant() < 0.0
    }
}
