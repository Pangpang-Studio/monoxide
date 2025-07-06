//! Affine transform utilities

use num_traits::real::Real;

use crate::IPoint2D;

/// Represents a 2D affine transformation.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Affine2D<P> {
    /// The translation vector, applied after scaling and rotation.
    trans: P,
    /// The scaling and rotation matrix, in column-major order.
    mat: [P; 2],
}

impl<P: IPoint2D<Scalar = S>, S: Real> Affine2D<P> {
    /// Creates an identity affine transformation.
    pub fn id() -> Self {
        Affine2D {
            trans: P::zero(),
            mat: [P::unit(0), P::unit(1)],
        }
    }

    pub fn make(translation: P, matrix: [P; 2]) -> Self {
        Affine2D {
            trans: translation,
            mat: matrix,
        }
    }

    pub fn translate(self, translation: P) -> Self {
        Affine2D {
            trans: self.trans.point_add(&translation),
            ..self
        }
    }

    pub fn rotate(self, angle: P::Scalar) -> Self {
        let cos = P::Scalar::cos(angle);
        let sin = P::Scalar::sin(angle);
        Affine2D {
            mat: [
                self.mat[0]
                    .mul_scalar(cos)
                    .point_sub(&self.mat[1].mul_scalar(sin)),
                self.mat[0]
                    .mul_scalar(sin)
                    .point_add(&self.mat[1].mul_scalar(cos)),
            ],
            ..self
        }
    }

    pub fn scale(self, scale: P::Scalar) -> Self {
        Affine2D {
            mat: [self.mat[0].mul_scalar(scale), self.mat[1].mul_scalar(scale)],
            ..self
        }
    }

    pub fn apply(&self, point: &P) -> P {
        let x = self.mat[0].dot(point);
        let y = self.mat[1].dot(point);
        P::make(x, y).point_add(&self.trans)
    }

    // Nice-to-have wrapper functions
    /// Create a transformation that translates the point by `translation`.
    pub fn translated(translation: P) -> Self {
        Self::id().translate(translation)
    }

    /// Create a transformation that rotates the point by `angle`.
    pub fn rotated(angle: P::Scalar) -> Self {
        Self::id().rotate(angle)
    }

    /// Create a transformation that scales the point by `scale`.
    pub fn scaled(scale: P::Scalar) -> Self {
        Self::id().scale(scale)
    }
}
