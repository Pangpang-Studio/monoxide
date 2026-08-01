//! The 2D point type used throughout this crate.
//!
//! [`Point2D`] is simply [`kurbo::Point`]. This module only adds a handful of
//! small convenience helpers (via [`Point2DExt`]) that are used by the
//! stroking code and by consumers of this crate, but that don't exist on
//! `kurbo`'s own `Point`/`Vec2` types.

pub use kurbo::Point as Point2D;

/// Small extension methods for [`Point2D`] that are not provided by `kurbo`
/// itself.
pub trait Point2DExt: Sized {
    /// The unit vector on the X axis, i.e. `(1, 0)`.
    fn unit_x() -> Self;
    /// The unit vector on the Y axis, i.e. `(0, 1)`.
    fn unit_y() -> Self;

    /// Returns a copy of this point with the X coordinate replaced.
    fn with_x(self, x: f64) -> Self;
    /// Returns a copy of this point with the Y coordinate replaced.
    fn with_y(self, y: f64) -> Self;

    /// Returns a copy of this point with the X coordinate negated.
    fn neg_x(self) -> Self;
    /// Returns a copy of this point with the Y coordinate negated.
    fn neg_y(self) -> Self;

    /// Normalizes the vector from the origin to this point to unit length.
    fn normalize(self) -> Self;
}

impl Point2DExt for Point2D {
    fn unit_x() -> Self {
        Self::new(1.0, 0.0)
    }

    fn unit_y() -> Self {
        Self::new(0.0, 1.0)
    }

    fn with_x(self, x: f64) -> Self {
        Self::new(x, self.y)
    }

    fn with_y(self, y: f64) -> Self {
        Self::new(self.x, y)
    }

    fn neg_x(self) -> Self {
        Self::new(-self.x, self.y)
    }

    fn neg_y(self) -> Self {
        Self::new(self.x, -self.y)
    }

    fn normalize(self) -> Self {
        self.to_vec2().normalize().to_point()
    }
}
