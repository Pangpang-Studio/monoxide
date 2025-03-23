use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{Point, RealPoint};

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Point2D { x, y }
    }

    pub fn normalize(self) -> Self {
        let norm = self.norm();
        Self::new(self.x / norm, self.y / norm)
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn normal_left(self) -> Self {
        Self::new(-self.y, self.x)
    }

    pub fn normal_right(self) -> Self {
        Self::new(self.y, -self.x)
    }
}

impl Point for Point2D {
    type Scalar = f64;

    fn mul_scalar(&self, scalar: Self::Scalar) -> Self {
        Self::new(self.x * scalar, self.y * scalar)
    }

    fn point_add(&self, other: &Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }

    fn point_sub(&self, other: &Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl RealPoint for Point2D {
    fn norm(&self) -> Self::Scalar {
        self.x.hypot(self.y)
    }
}

impl Add for Point2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Point2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f64> for Point2D {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self::new(self.x * scalar, self.y * scalar)
    }
}

impl Mul<Point2D> for f64 {
    type Output = Point2D;

    fn mul(self, point: Point2D) -> Point2D {
        Point2D::new(point.x * self, point.y * self)
    }
}

impl Div<f64> for Point2D {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Self::new(self.x / scalar, self.y / scalar)
    }
}

impl Neg for Point2D {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl From<(f64, f64)> for Point2D {
    fn from((x, y): (f64, f64)) -> Self {
        Point2D::new(x, y)
    }
}

impl From<Point2D> for (f64, f64) {
    fn from(point: Point2D) -> Self {
        (point.x, point.y)
    }
}

impl From<spiro::SpiroCP> for Point2D {
    fn from(spiro_cp: spiro::SpiroCP) -> Self {
        Point2D::new(spiro_cp.x, spiro_cp.y)
    }
}
impl From<&spiro::SpiroCP> for Point2D {
    fn from(spiro_cp: &spiro::SpiroCP) -> Self {
        Point2D::new(spiro_cp.x, spiro_cp.y)
    }
}
