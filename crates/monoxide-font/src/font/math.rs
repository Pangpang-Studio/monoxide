use std::ops::{Add, Mul, Sub};

/// Returns the weighted average of `z` and `w` based on the `ratio`.
/// The weight is put on `z`, i.e. it returns `w + (z - w) * ratio`.
pub fn mix<F, T: Add<T, Output = T> + Sub<T, Output = T> + Mul<F, Output = T> + Copy>(
    z: T,
    w: T,
    ratio: F,
) -> T {
    w + (z - w) * ratio
}
