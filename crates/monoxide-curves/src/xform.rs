//! Affine transform utilities.
//!
//! [`AffineExt`] adds a handful of domain-specific constructors and
//! introspection helpers on top of existing `kurbo` functionalities.

pub use kurbo::Affine as Affine2D;

use crate::point::Point2D;

/// Extension methods for [`Affine2D`] used throughout this crate and its
/// consumers, on top of what `kurbo` already provides.
pub trait AffineExt: Sized {
    /// Creates an affine transform from a translation and a column-major 2x2
    /// matrix.
    fn make(translation: impl Into<Point2D>, matrix: [impl Into<Point2D>; 2]) -> Self;

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
    fn make(translation: impl Into<Point2D>, matrix: [impl Into<Point2D>; 2]) -> Self {
        let translation = translation.into();
        let [col0, col1] = matrix.map(Into::into);
        Self::new([col0.x, col0.y, col1.x, col1.y, translation.x, translation.y])
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
