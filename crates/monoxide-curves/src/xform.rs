//! Affine transform utilities

use num_traits::real::Real;

use crate::IPoint2D;

/// Represents a 2D affine transformation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Affine2D<P> {
    /// The translation vector, applied after scaling and rotation.
    trans: P,
    /// The scaling and rotation matrix, in column-major order.
    mat: [P; 2],
}

impl<P: IPoint2D<Scalar = S> + Clone, S: Real> Affine2D<P> {
    /// Creates an identity affine transformation.
    pub fn id() -> Self {
        Affine2D {
            trans: P::zero(),
            mat: [P::unit(0), P::unit(1)],
        }
    }

    pub fn make(translation: impl Into<P>, matrix: [impl Into<P>; 2]) -> Self {
        Affine2D {
            trans: translation.into(),
            mat: matrix.map(Into::into),
        }
    }

    pub fn translation(&self) -> P {
        self.trans.clone()
    }

    pub fn matrix(&self) -> [P; 2] {
        self.mat.clone()
    }

    pub fn translate(self, translation: impl Into<P>) -> Self {
        Affine2D {
            trans: self.trans.point_add(&translation.into()),
            ..self
        }
    }

    pub fn rotate(self, angle: impl Into<P::Scalar>) -> Self {
        let angle = angle.into();
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

    pub fn scale(self, scale: impl Into<P::Scalar>) -> Self {
        let scale = scale.into();
        Affine2D {
            mat: [self.mat[0].mul_scalar(scale), self.mat[1].mul_scalar(scale)],
            ..self
        }
    }

    /// Mirror (reflect) points across the line defined by `point` and
    /// `direction`. The reflection is applied to the input of this
    /// transformation (i.e. the matrix is right-multiplied by the
    /// reflection matrix) and the translation is adjusted so the overall
    /// affine transformation equals: T ∘ Reflect_line(point, direction).
    pub fn mirror_along(self, point: impl Into<P>, direction: impl Into<P>) -> Self {
        let point = point.into();
        let direction = direction.into();

        // compute squared length of direction
        let len_sq = direction.x() * direction.x() + direction.y() * direction.y();
        if len_sq.is_zero() {
            // degenerate direction: no-op
            return self;
        }

        // unit direction u
        let inv_len = len_sq.sqrt().recip();
        let u = direction.mul_scalar(inv_len);
        let ux = u.x();
        let uy = u.y();

        // reflection matrix R = 2*u*u^T - I
        let two = P::Scalar::from(2).unwrap();
        let one = P::Scalar::from(1).unwrap();
        let r00 = two * ux * ux - one;
        let r01 = two * ux * uy;
        let r10 = r01;
        let r11 = two * uy * uy - one;

        // columns of R (needed to multiply rows of self.mat by R)
        let rcol0 = P::make(r00, r10);
        let rcol1 = P::make(r01, r11);

        // new matrix = self.mat * R
        let new_mat = [
            P::make(self.mat[0].dot(&rcol0), self.mat[0].dot(&rcol1)),
            P::make(self.mat[1].dot(&rcol0), self.mat[1].dot(&rcol1)),
        ];

        // translation: A*(p0 - R*p0) + t
        let p0 = point;
        let rp0 = P::make(r00 * p0.x() + r01 * p0.y(), r10 * p0.x() + r11 * p0.y());
        let diff = p0.point_sub(&rp0);
        let add = P::make(self.mat[0].dot(&diff), self.mat[1].dot(&diff));
        let new_trans = self.trans.point_add(&add);

        Affine2D {
            mat: new_mat,
            trans: new_trans,
        }
    }

    pub fn rotate_around(self, center: impl Into<P>, angle: impl Into<P::Scalar>) -> Self {
        let center = center.into();
        let angle = angle.into();

        let arm = self.trans.point_sub(&center);
        self.translate(arm.mul_scalar(-P::Scalar::one()))
            .rotate(angle)
            .translate(Self::rotated(angle).apply(&arm))
    }

    pub fn apply(&self, point: &P) -> P {
        let x = self.mat[0].dot(point);
        let y = self.mat[1].dot(point);
        P::make(x, y).point_add(&self.trans)
    }

    // Nice-to-have wrapper functions
    /// Create a transformation that translates the point by `translation`.
    pub fn translated(translation: impl Into<P>) -> Self {
        Self::id().translate(translation)
    }

    /// Create a transformation that rotates the point by `angle`.
    pub fn rotated(angle: impl Into<P::Scalar>) -> Self {
        Self::id().rotate(angle)
    }

    /// Create a transformation that rotates the point by `angle` around `center`.
    pub fn rotated_around(center: impl Into<P>, angle: impl Into<P::Scalar>) -> Self {
        Self::id().rotate_around(center, angle)
    }

    /// Create a transformation that scales the point by `scale`.
    pub fn scaled(scale: impl Into<P::Scalar>) -> Self {
        Self::id().scale(scale)
    }

    /// Create a transformation that reflects the point along the line that
    /// crosses `base` and goes in `direction`.
    pub fn mirrored_along(base: impl Into<P>, direction: impl Into<P>) -> Self {
        Self::id().mirror_along(base, direction)
    }

    // Info to simplify the transformation in pipeline stages
    /// Returns whether the transform does not scale the point.
    pub fn scale_is_identity(&self) -> bool {
        self.mat[0] == P::unit(0) && self.mat[1] == P::unit(1)
    }

    /// Returns `Some(scale)` if the transformation's matrix part
    /// only contains a uniform scaling operation, `None` otherwise.
    pub fn scale_is_uniform(&self) -> Option<P::Scalar> {
        if self.mat[0].x() == self.mat[1].y()
            && self.mat[0].y().is_zero()
            && self.mat[1].x().is_zero()
        {
            Some(self.mat[0].x())
        } else {
            None
        }
    }

    /// Returns `Some((scale_x, scale_y))` if the transformation's matrix part
    /// only contains a scaling operation, `None` otherwise. This function does
    /// not check the translation part.
    pub fn mat_is_only_scale(&self) -> Option<(P::Scalar, P::Scalar)> {
        let scale_x = self.mat[0].x();
        let scale_y = self.mat[1].y();
        if self.mat[0].y().is_zero() && self.mat[1].x().is_zero() {
            Some((scale_x, scale_y))
        } else {
            None
        }
    }

    /// Returns true if the linear part of the transform flips orientation
    /// (i.e. has a negative determinant). This indicates the transform
    /// mirrors/reflections compared to orientation-preserving transforms.
    pub fn flips_direction(&self) -> bool {
        let m00 = self.mat[0].x();
        let m01 = self.mat[0].y();
        let m10 = self.mat[1].x();
        let m11 = self.mat[1].y();
        let det = m00 * m11 - m01 * m10;
        det < P::Scalar::zero()
    }
}

impl<P: IPoint2D<Scalar = S> + Clone, S: Real> Default for Affine2D<P> {
    fn default() -> Self {
        Self::id()
    }
}
