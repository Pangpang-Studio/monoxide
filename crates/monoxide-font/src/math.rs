use std::ops::{Add, Mul, Sub};

/// Returns the weighted average of `z` and `w` based on the `ratio`.
/// The weight is put on `z`, i.e. it returns `w + (z - w) * ratio`.
pub fn mix<F, T, D>(z: T, w: T, ratio: F) -> T
where
    T: Sub<T, Output = D> + Copy,
    D: Mul<F>,
    T: Add<<D as Mul<F>>::Output, Output = T>,
{
    w + (z - w) * ratio
}
