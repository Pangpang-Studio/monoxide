use std::collections::BTreeMap;

use crate::error::{Error, Result};

pub(super) fn interpolate(
    values: &BTreeMap<usize, f64>,
    positions: &[f64],
    total_length: f64,
    closed: bool,
    default: f64,
) -> Result<Vec<f64>> {
    if values.is_empty() {
        return Ok(vec![default; positions.len()]);
    }

    positions
        .iter()
        .enumerate()
        .map(|(index, &position)| {
            if let Some(&value) = values.get(&index) {
                return Ok(value);
            }
            let previous = values.range(..index).next_back().or_else(|| {
                if closed {
                    values.last_key_value()
                } else {
                    None
                }
            });
            let next = values.range(index + 1..).next().or_else(|| {
                if closed {
                    values.first_key_value()
                } else {
                    None
                }
            });
            match (previous, next) {
                (Some((&prev, &a)), Some((&next, &b))) => {
                    let start = positions[prev] - if prev > index { total_length } else { 0.0 };
                    let end = positions[next] + if next < index { total_length } else { 0.0 };
                    if end <= start {
                        if a == b {
                            return Ok(a);
                        }
                        return Err(Error::InvalidStroke(
                            "conflicting attributes on coincident knots",
                        ));
                    }
                    Ok(a + (b - a) * ((position - start) / (end - start)))
                }
                (Some((_, &value)), None) | (None, Some((_, &value))) => Ok(value),
                (None, None) => Err(Error::internal("missing stroke attributes")),
            }
        })
        .collect()
}

/// Shape-preserving derivatives with respect to arc length.
pub(super) fn slopes(values: &[f64], positions: &[f64], total: f64, closed: bool) -> Vec<f64> {
    let n = values.len();
    let secant = |a: usize, b: usize, length: f64| {
        if length > super::TANGENT_EPSILON {
            (values[b] - values[a]) / length
        } else {
            0.0
        }
    };
    (0..n)
        .map(|i| {
            if !closed && i == 0 {
                return secant(0, 1, positions[1] - positions[0]);
            }
            if !closed && i == n - 1 {
                return secant(n - 2, n - 1, positions[n - 1] - positions[n - 2]);
            }
            let prev = if i == 0 { n - 1 } else { i - 1 };
            let next = (i + 1) % n;
            let before = if i == 0 {
                positions[0] + total - positions[prev]
            } else {
                positions[i] - positions[prev]
            };
            let after = if next == 0 {
                positions[0] + total - positions[i]
            } else {
                positions[next] - positions[i]
            };
            let a = secant(prev, i, before);
            let b = secant(i, next, after);
            if a == 0.0 || b == 0.0 || a.signum() != b.signum() {
                return 0.0;
            }
            let wa = 2.0 * after + before;
            let wb = after + 2.0 * before;
            (wa + wb) / (wa / a + wb / b)
        })
        .collect()
}
