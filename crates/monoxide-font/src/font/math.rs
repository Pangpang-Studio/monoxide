use std::ops::{Add, Mul, Sub};

/// Returns the weighted average of `z` and `w` based on the `ratio`.
pub fn mix<F, T: Add<T, Output = T> + Sub<T, Output = T> + Mul<F, Output = T> + Copy>(
    z: T,
    w: T,
    ratio: F,
) -> T {
    w + (z - w) * ratio
}
