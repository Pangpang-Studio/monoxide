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
    // Fast path: if there's nothing to interpolate, we should not bother to
    // calculate the curve length.
    //
    // We have a couple things to check to ensure we don't need to interpolate.
    //
    // For width factors, it should be either:
    // - completely unset (everything is 1.0),
    // - only one point set (everything is the same as that point), or
    // - all points set (just use their given values).
    let width_fast_path_res = width_fast_path(curve);
    // For alignments, it's either:
    // - completely unset (all are middle-aligned), or
    // - No `Interpolate` points in the map
    let alignment_fast_path_res = alignment_fast_path(curve);

    // Early return for fast paths
    if let (Some(width_factors), Some(alignments)) = (width_fast_path_res, alignment_fast_path_res)
    {
        return SolvedStrokeAttrs {
            width_factors,
            alignments,
        };
    }

    todo!()
}

fn width_fast_path(curve: &SpiroCurve) -> Option<Vec<f64>> {
    if curve.width_factors.is_empty() {
        Some(vec![default_width_factor(); curve.len()])
    } else if curve.width_factors.len() == 1 {
        let (_, &w) = curve
            .width_factors
            .iter()
            .next()
            .expect("We checked the length");
        Some(vec![w; curve.len()])
    } else if curve.width_factors.len() == curve.len() {
        let v = (0..curve.len())
            .map(|x| {
                curve
                    .width_factors
                    .get(&x)
                    .copied()
                    .expect("We checked the length")
            })
            .collect();
        Some(v)
    } else {
        None
    }
}

fn alignment_fast_path(curve: &SpiroCurve) -> Option<Vec<f64>> {
    let no_alignment_interp = curve
        .alignment
        .values()
        .all(|x| !matches!(x, StrokeAlignment::Interpolate));
    if !no_alignment_interp {
        return None;
    }

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

    Some(res)
}
