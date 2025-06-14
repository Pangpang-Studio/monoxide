use std::ops::{Add, Mul, Range, Sub};

/// Returns the weighted average of `z` and `w` based on the `ratio`.
pub fn mix<F, T: Add<T, Output = T> + Sub<T, Output = T> + Mul<F, Output = T> + Copy>(
    z: T,
    w: T,
    ratio: F,
) -> T {
    w + (z - w) * ratio
}

pub fn crange(center: f64, radius: f64) -> Range<f64> {
    (center - radius)..(center + radius)
}
