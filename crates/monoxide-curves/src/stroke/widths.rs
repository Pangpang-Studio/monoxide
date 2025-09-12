use std::{cell::LazyCell, collections::BTreeMap};

use flo_curves::bezier::curve_length;
use itertools::Itertools;

use crate::{
    CubicBezier,
    point::Point2D,
    spiro::{SpiroCurve, default_alignment, default_width_factor},
};

pub struct SolvedStrokeAttrs {
    /// Width factor relative to the default width of the stroke.
    pub width_factors: Vec<f64>,
    /// Represents the position of the control point within the normal, left=0,
    /// right=1. See [`StrokeAlignment`].
    pub alignments: Vec<f64>,
}

/// Determine the width factors and alignments for each point of the curve.
///
/// The reason why these two operations are merged into one is because we need
/// to calculate the length of each curve segments to do interpolation. Having
/// both calculations in one function allows us to avoid calculating the
/// length of the curve twice.
///
/// Additionally, in many cases, the attributes do not need to be interpolated
/// at all, so we can avoid the overhead of calculating the curve length
/// entirely.
pub fn solve_stroke_attrs(
    curve: &SpiroCurve,
    cubic: &CubicBezier<Point2D>,
    indices: &[usize],
) -> SolvedStrokeAttrs {
    // Lazy-evaluated curve length information.
    //
    // Calculating the curve length can be quite expensive, so we should avoid
    // doing it unless we actually need to interpolate the width factors or
    // alignments.
    //
    // The calculated length is for each segment of the spiro curve.
    let curve_lengths = LazyCell::new(|| calc_curve_lengths(curve, cubic, indices));

    let width_factors = populate_with_interpolation(
        &curve.width_factors,
        curve.len(),
        default_width_factor(),
        curve.is_closed,
        &curve_lengths,
    );
    let alignments = populate_with_interpolation(
        &curve.alignment,
        curve.len(),
        default_alignment(),
        curve.is_closed,
        &curve_lengths,
    );

    SolvedStrokeAttrs {
        width_factors,
        alignments,
    }
}

fn calc_curve_lengths(
    curve: &SpiroCurve,
    cubic: &CubicBezier<Point2D>,
    indices: &[usize],
) -> Vec<f64> {
    let max_error = 0.001;
    let mut lengths = Vec::with_capacity(curve.len());
    for window in indices.windows(2) {
        let &[from, to] = window else {
            panic!("window size mismatch")
        };
        let mut acc = 0.0;
        for seg in from..to {
            let seg_length = curve_length(&cubic.segment(seg).unwrap(), max_error);
            acc += seg_length;
        }
        lengths.push(acc);
    }
    // last segment
    {
        let mut acc = 0.0;
        for seg in indices.last().copied().unwrap_or(0)..curve.len() {
            let seg_length = curve_length(&cubic.segment(seg).unwrap(), max_error);
            acc += seg_length;
        }
        lengths.push(acc);
    }
    assert_eq!(lengths.len(), curve.len(), "length calculation mismatch");
    lengths
}

/// Given a map with indices and their corresponding values, this function
/// interpolates the values for every point in a curve of given length,
/// linearly, by the actual curve length of each segment.
//
// This function can be futher generalized to support any type representing a
// real vector space, i.e. implementing `Add`, `Sub`, `Mul<f64>`.
fn populate_with_interpolation(
    map: &BTreeMap<usize, f64>,
    len: usize,
    default: f64,
    is_closed: bool,
    curve_lengths: &LazyCell<Vec<f64>, impl FnOnce() -> Vec<f64>>,
) -> Vec<f64> {
    if map.is_empty() {
        return vec![default; len];
    }
    let mut res = vec![];
    res.reserve_exact(len);

    // 0..first_index
    let (&first_key, &first_val) = map.first_key_value().expect("checked for empty map");
    if is_closed {
        // Fill with placeholders; see actual filling in last_index
        res.extend(std::iter::repeat_n(0.0, first_key));
    } else {
        for _ in 0..(first_key) {
            res.push(first_val)
        }
    }

    // first_index..last_index
    // Only handles the start of each segment in the loop
    for ((&idx1, &width1), (&idx2, &width2)) in map.iter().tuple_windows() {
        assert!(idx2 > idx1);
        if idx2 - idx1 == 1 {
            res.push(width1);
        } else if width1 == width2 {
            res.extend(std::iter::repeat_n(width1, idx2 - idx1))
        } else {
            LazyCell::force(curve_lengths);
            let lengths = &curve_lengths[idx1..idx2];
            let total_len: f64 = lengths.iter().copied().sum();
            let mut sum = 0.0;
            for l in lengths {
                sum += l;
                res.push(width1 + (width2 - width1) * (sum / total_len));
            }
        }
    }

    // last_index..
    let (&last_key, &last_val) = map.last_key_value().expect("checked for empty map");
    if is_closed {
        if first_key == 0 && last_key == len - 1 {
            res.push(last_val);
        } else if last_val == first_val {
            // If the last value is the same as the first value, we can just
            // fill the rest with that value.
            res.extend(std::iter::repeat_n(last_val, len - res.len()));
            for x in &mut res[..first_key] {
                *x = last_val;
            }
        } else {
            // This fills both last_index.. and 0..first_index
            let lengths_tail = &curve_lengths[last_key..];
            let lengths_head = &curve_lengths[..first_key];
            let total_len: f64 = lengths_tail.iter().chain(lengths_head).copied().sum();
            let mut sum = 0.0;
            for l in lengths_tail {
                sum += l;
                res.push(last_val + (first_val - last_val) * (sum / total_len));
            }
            for (idx, &l) in lengths_head.iter().enumerate() {
                sum += l;
                res[idx] = last_val + (first_val - last_val) * (sum / total_len);
            }
        }
    } else {
        for _ in last_key..len {
            res.push(last_val);
        }
    }

    res
}
