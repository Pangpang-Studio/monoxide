use std::collections::BTreeMap;

use crate::error::{Error, Result};

/// Sample a sparse knot attribute by arc length. Open ends extend the nearest
/// value. On closed paths the interval from the last key to the first wraps
/// through the closing segment.
pub(super) fn at_knots(
    keys: &BTreeMap<usize, f64>,
    default: f64,
    distances: &[f64],
    perimeter: f64,
    closed: bool,
) -> Result<Vec<f64>> {
    let Some((&last, &last_value)) = keys.last_key_value() else {
        return Ok(vec![default; distances.len()]);
    };
    if last >= distances.len() || keys.values().any(|v| !v.is_finite()) {
        return Err(Error::InvalidStroke);
    }
    let (&first, &first_value) = keys.first_key_value().unwrap();
    Ok(distances
        .iter()
        .enumerate()
        .map(|(i, &s)| {
            let (s0, a) = keys.range(..=i).next_back().map_or(
                (
                    distances[last] - perimeter,
                    if closed { last_value } else { first_value },
                ),
                |(&k, &a)| (distances[k], a),
            );
            let (s1, b) = keys.range(i..).next().map_or(
                (
                    distances[first] + perimeter,
                    if closed { first_value } else { last_value },
                ),
                |(&k, &b)| (distances[k], b),
            );
            if s0 == s1 || a == b {
                a
            } else {
                a + (b - a) * ((s - s0) / (s1 - s0))
            }
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sparse_keys_stay_at_their_knots_across_the_seam() {
        // Accumulating the next span's length before sampling shifts each
        // key by one knot, especially at the wraparound on a closed path.
        let keys = BTreeMap::from([(1, 1.0), (3, 5.0)]);
        let distances = [0., 1., 2., 5., 6.];
        assert_eq!(
            at_knots(&keys, 0.5, &distances, 8., false).unwrap(),
            [1., 1., 2., 5., 5.]
        );
        assert_eq!(
            at_knots(&keys, 0.5, &distances, 8., true).unwrap(),
            [2., 1., 2., 5., 4.]
        );
    }
}
