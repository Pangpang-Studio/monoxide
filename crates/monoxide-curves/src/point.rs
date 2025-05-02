use std::ops::{Add, Div, Mul, Neg, Sub};

use monoxide_spiro::SpiroCp;

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

impl From<SpiroCp> for Point2D {
    fn from(spiro_cp: SpiroCp) -> Self {
        Point2D::new(spiro_cp.x, spiro_cp.y)
    }
}
impl From<&SpiroCp> for Point2D {
    fn from(spiro_cp: &SpiroCp) -> Self {
        Point2D::new(spiro_cp.x, spiro_cp.y)
    }
}

impl flo_curves::Coordinate for Point2D {
    fn from_components(components: &[f64]) -> Self {
        Point2D::new(components[0], components[1])
    }

    fn origin() -> Self {
        Point2D::new(0.0, 0.0)
    }

    fn len() -> usize {
        2
    }

    fn get(&self, index: usize) -> f64 {
        match index {
            0 => self.x,
            1 => self.y,
            _ => panic!("Invalid index"),
        }
    }

    fn from_biggest_components(p1: Self, p2: Self) -> Self {
        let x = p1.x.max(p2.x);
        let y = p1.y.max(p2.y);
        Point2D::new(x, y)
    }

    fn from_smallest_components(p1: Self, p2: Self) -> Self {
        let x = p1.x.min(p2.x);
        let y = p1.y.min(p2.y);
        Point2D::new(x, y)
    }
}

impl flo_curves::Coordinate2D for Point2D {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }
}
