//! Operations and types related to Bezier curves.
pub mod cube;
pub mod quad;
pub use quad::{QuadBezier, QuadSegment};

use num_traits::Num;

pub trait Point: PartialEq {
    type Scalar: Num;

    fn mul_scalar(&self, scalar: Self::Scalar) -> Self;
    fn point_add(&self, other: &Self) -> Self;
}

impl<N: Num + Copy> Point for (N, N) {
    type Scalar = N;

    fn mul_scalar(&self, scalar: N) -> Self {
        (self.0 * scalar, self.1 * scalar)
    }

    fn point_add(&self, other: &Self) -> Self {
        (self.0 + other.0, self.1 + other.1)
    }
}
