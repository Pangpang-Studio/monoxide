use std::cell::LazyCell;

use flo_curves::bezier::curve_length;

use crate::{
    CubicBezier,
    point::Point2D,
    spiro::{SpiroCurve, StrokeAlignment, default_alignment, default_width_factor},
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
    let curve_lengths = LazyCell::new(|| {
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
    });

    // Fast path: if there's nothing to interpolate, we should not bother to
    // calculate the curve length.
    //
    // We have a couple things to check to ensure we don't need to interpolate.
    //
    // For width factors, it should be either:
    // - completely unset (everything is 1.0),
    // - only one point set (everything is the same as that point), or
    // - all points set (just use their given values).
    let width_factors = width_fast_path(curve, &curve_lengths);
    // For alignments, it's either:
    // - completely unset (all are middle-aligned), or
    // - No `Interpolate` points in the map
    let alignments = alignment_fast_path(curve, &curve_lengths);

    SolvedStrokeAttrs {
        width_factors,
        alignments,
    }
}

fn width_fast_path<F: FnOnce() -> Vec<f64>>(
    curve: &SpiroCurve,
    curve_lengths: &LazyCell<Vec<f64>, F>,
) -> Vec<f64> {
    if curve.width_factors.is_empty() {
        vec![default_width_factor(); curve.len()]
    } else if curve.width_factors.len() == 1 {
        let (_, &w) = curve
            .width_factors
            .iter()
            .next()
            .expect("We checked the length");
        vec![w; curve.len()]
    } else if curve.width_factors.len() == curve.len() {
        (0..curve.len())
            .map(|x| {
                curve
                    .width_factors
                    .get(&x)
                    .copied()
                    .expect("We checked the length")
            })
            .collect()
    } else {
        todo!()
    }
}

fn alignment_fast_path<F: FnOnce() -> Vec<f64>>(
    curve: &SpiroCurve,
    curve_lengths: &LazyCell<Vec<f64>, F>,
) -> Vec<f64> {
    let no_alignment_interp = curve
        .alignment
        .values()
        .all(|x| !matches!(x, StrokeAlignment::Interpolate));
    if no_alignment_interp {
        let mut last = default_alignment();
        let mut res = vec![];
        res.reserve_exact(curve.len());
        for i in 0..curve.len() {
            let alignment = if let Some(alignment) = curve.alignment.get(&i) {
                match alignment {
                    &StrokeAlignment::Aligned(v) => v,
                    StrokeAlignment::Interpolate => unreachable!("checked for no interpolation"),
                }
            } else {
                last
            };
            res.push(alignment);
            last = alignment;
        }
        res
    } else {
        todo!()
    }
}
