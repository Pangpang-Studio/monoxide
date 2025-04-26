//! Operations and types related to Bezier curves.
pub mod convert;
pub mod cube;
pub mod debug;
pub mod point;
pub mod quad;
pub mod stroke;
pub use cube::{CubicBezier, CubicSegment};
use num_traits::{Num, real::Real};
pub use quad::{QuadBezier, QuadSegment};

/// Represents a spiro curve.
pub type SpiroCurve = Vec<spiro::SpiroCP>;

/// Represents a point in space.
pub trait Point: PartialEq {
    type Scalar: Num;

    fn mul_scalar(&self, scalar: Self::Scalar) -> Self;
    fn point_add(&self, other: &Self) -> Self;
    fn point_sub(&self, other: &Self) -> Self;
}

impl<N: Num + Copy> Point for (N, N) {
    type Scalar = N;

    fn mul_scalar(&self, scalar: N) -> Self {
        (self.0 * scalar, self.1 * scalar)
    }

    fn point_add(&self, other: &Self) -> Self {
        (self.0 + other.0, self.1 + other.1)
    }

    fn point_sub(&self, other: &Self) -> Self {
        (self.0 - other.0, self.1 - other.1)
    }
}

/// A point with real coordinates.
pub trait RealPoint: Point<Scalar: Real> {
    fn norm(&self) -> Self::Scalar;
}

impl<N: Real + Copy> RealPoint for (N, N) {
    fn norm(&self) -> N {
        self.0.hypot(self.1)
    }
}
