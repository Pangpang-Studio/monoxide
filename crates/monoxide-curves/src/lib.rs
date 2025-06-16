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
pub type SpiroCurve = Vec<monoxide_spiro::SpiroCp>;

/// Represents a point in space. Also represents a vector from origin to this
/// point for the convenience of calculation.
pub trait Point: PartialEq {
    type Scalar: Num;

    fn zero() -> Self;
    /// Unit vector on the given axis
    fn unit(axis: usize) -> Self;
    /// Returns a value with the given scalar on the given axis
    fn with_axis(&self, axis: usize, value: Self::Scalar) -> Self;

    fn mul_scalar(&self, scalar: Self::Scalar) -> Self;
    fn scale(&self, vector: &Self) -> Self;
    fn point_add(&self, other: &Self) -> Self;
    fn point_sub(&self, other: &Self) -> Self;
}

impl<N: Num + Copy> Point for (N, N) {
    type Scalar = N;

    fn zero() -> Self {
        (N::zero(), N::zero())
    }

    fn unit(axis: usize) -> Self {
        match axis {
            0 => (N::one(), N::zero()),
            1 => (N::zero(), N::one()),
            _ => panic!("Invalid axis for 2D point"),
        }
    }

    fn with_axis(&self, axis: usize, value: N) -> Self {
        match axis {
            0 => (value, self.1),
            1 => (self.0, value),
            _ => panic!("Invalid axis for 2D point"),
        }
    }

    fn mul_scalar(&self, scalar: N) -> Self {
        (self.0 * scalar, self.1 * scalar)
    }

    fn scale(&self, vector: &Self) -> Self {
        (self.0 * vector.0, self.1 * vector.1)
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

impl Point for f64 {
    type Scalar = f64;

    fn zero() -> Self {
        0.0
    }

    fn unit(axis: usize) -> Self {
        match axis {
            0 => 1.0,
            _ => panic!("Invalid axis for 1D point"),
        }
    }

    fn with_axis(&self, axis: usize, value: f64) -> Self {
        match axis {
            0 => value,
            _ => panic!("Invalid axis for 1D point"),
        }
    }

    fn mul_scalar(&self, scalar: f64) -> Self {
        self * scalar
    }

    fn scale(&self, vector: &Self) -> Self {
        self * vector
    }

    fn point_add(&self, other: &Self) -> Self {
        self + other
    }

    fn point_sub(&self, other: &Self) -> Self {
        self - other
    }
}

impl RealPoint for f64 {
    fn norm(&self) -> f64 {
        self.abs()
    }
}
