//! Expand solved Bézier centerlines in font coordinates (one unit per em).

use itertools::Itertools;
use kurbo::{BezPath, ParamCurve, ParamCurveArclen, ParamCurveDeriv, PathSeg, Vec2};
use kurbo_se::{StrokeSide, StrokeStyle, stroke_aligned};
use linesweeper::topology::Topology;
use monoxide_spiro::{SpiroCp, SpiroCpTy};

use crate::{
    CubicBezier, SpiroCurve,
    convert::spiro_to_cube_with_indices,
    error::{Error, Result},
};

mod edge;
mod widths;

// Smaller than a unit on the default TrueType grid.
const TOLERANCE: f64 = 0.0001;
const MITER_LIMIT: f64 = 4.0;

fn ensure_single_piece(curve: &[SpiroCp]) -> Result<()> {
    use SpiroCpTy::*;

    let is_single_piece = match curve {
        [] | [_] => false, // No curve
        [
            SpiroCp { ty: Open, .. },
            open @ ..,
            SpiroCp { ty: EndOpen, .. },
        ] => !open.iter().any(|cp| matches!(cp.ty, Open | EndOpen)),
        [SpiroCp { ty: Open, .. }, ..] | [SpiroCp { ty: EndOpen, .. }, ..] => false,
        closed => !closed.iter().any(|cp| matches!(cp.ty, Open | EndOpen)),
    };
    if !is_single_piece {
        return Err(Error::SpiroBroken);
    }
    Ok(())
}

/// Resolve overlapping turns before the outlines are used in boolean operations
/// or rounded to the font grid. Both expanders emit positive-winding ink.
/// TrueType wants the outer boundary clockwise.
fn contours(path: BezPath) -> Result<Vec<CubicBezier>> {
    if path.is_empty() {
        return Ok(vec![]);
    }
    if !path.is_finite() {
        return Err(Error::InvalidStroke);
    }
    let topology = Topology::from_path(&path, TOLERANCE * 0.01)
        .map_err(|_| Error::internal("stroke produced an open contour"))?;
    if topology.has_normal_contours(()) {
        return Ok(path
            .reverse_subpaths()
            .subpaths()
            .map(CubicBezier::from_kurbo_elements)
            .collect());
    }
    Ok(topology
        .contours(|&w| w > 0)
        .contours()
        .map(|c| CubicBezier::from_kurbo(&c.path.reverse_subpaths()))
        .collect())
}

/// Stroke a Bézier contour with centered alignment, miter joins and butt caps.
pub fn stroke_bezier(curve: &CubicBezier, width: f64) -> Result<Vec<CubicBezier>> {
    aligned_bezier(curve, width, StrokeSide::Center)
}

fn aligned_bezier(curve: &CubicBezier, width: f64, side: StrokeSide) -> Result<Vec<CubicBezier>> {
    if !width.is_finite() || width < 0.0 {
        return Err(Error::InvalidStroke);
    }
    if width == 0.0 {
        return Ok(vec![]);
    }
    let path = curve.to_kurbo();
    if !path.is_finite() {
        return Err(Error::InvalidStroke);
    }
    let style = StrokeStyle::new(width).with_side(side);
    contours(stroke_aligned(path, &style, TOLERANCE))
}

/// Stroke one Spiro contour, interpolating its authored profile along the
/// solved centerline. Alignment is directional, including on closed contours.
pub fn stroke_spiro(curve: &SpiroCurve, width: f64) -> Result<Vec<CubicBezier>> {
    ensure_single_piece(&curve.points)?;
    if !width.is_finite() || width < 0.0 {
        return Err(Error::InvalidStroke);
    }
    if width == 0.0 {
        return Ok(vec![]);
    }
    let (cubics, markers) = spiro_to_cube_with_indices(&curve.points)?;
    let [cubic] = &cubics[..] else {
        return Err(Error::SpiroBroken);
    };
    let segments: Vec<_> = cubic.segment_iter().map(PathSeg::from).collect();
    let mut knots: Vec<_> = markers.iter().map(|m| m.segment_index).collect();
    if !cubic.closed {
        knots.push(segments.len());
    }
    if knots.len() != curve.len() || knots.first() != Some(&0) {
        return Err(Error::SpiroBroken);
    }
    if segments.iter().any(|seg| !seg.is_finite())
        || curve
            .tangents
            .iter()
            .any(|(&i, p)| i >= curve.len() || !p.is_finite())
    {
        return Err(Error::InvalidStroke);
    }
    let mut distance = vec![0.0];
    for seg in &segments {
        distance.push(distance.last().unwrap() + seg.arclen(TOLERANCE * 0.1));
    }
    let total = *distance.last().unwrap();
    if !total.is_finite() || curve.width_factors.values().any(|&w| w < 0.0) {
        return Err(Error::InvalidStroke);
    }
    let knot_distances: Vec<_> = knots.iter().map(|&k| distance[k]).collect();
    let at_knots =
        |keys, default| widths::at_knots(keys, default, &knot_distances, total, cubic.closed);
    let widths = at_knots(&curve.width_factors, crate::spiro::default_width_factor())?;
    if widths.iter().all(|&w| w == 0.0) {
        return Ok(vec![]);
    }
    let alignments = at_knots(&curve.alignment, crate::spiro::default_alignment())?;
    if curve.tangents.is_empty() && widths.iter().all_equal() && alignments.iter().all_equal() {
        // kurbo-se names sides in y-down coordinates. In the font's y-up
        // coordinates alignment 1 puts the whole stroke to the left.
        let side = match alignments[0] {
            0.0 => Some(StrokeSide::Left),
            0.5 => Some(StrokeSide::Center),
            1.0 => Some(StrokeSide::Right),
            _ => None,
        };
        if let Some(side) = side {
            return aligned_bezier(cubic, width * widths[0], side);
        }
    }
    let mut right = BezPath::new();
    let mut left = BezPath::new();
    if cubic.closed {
        knots.push(segments.len());
    }
    for (i, &[first, end]) in knots.array_windows().enumerate() {
        let j = (i + 1) % curve.len();
        let span = &segments[first..end];
        let length = distance[end] - distance[first];
        if length == 0.0 {
            continue;
        }
        let start_tangent = tangent(span[0], 0.0);
        let end_tangent = tangent(*span.last().unwrap(), 1.0);
        let before = first
            .checked_sub(1)
            .or_else(|| cubic.closed.then_some(segments.len() - 1));
        let after = if end == segments.len() {
            cubic.closed.then_some(0)
        } else {
            Some(end)
        };
        let skew0 = knot_skew(
            start_tangent,
            before.map(|k| tangent(segments[k], 1.0)),
            curve.tangents.get(&i),
        );
        let skew1 = knot_skew(
            end_tangent,
            after.map(|k| tangent(segments[k], 0.0)),
            curve.tangents.get(&j),
        );
        for (edge, side) in [(&mut left, 0.0), (&mut right, 1.0)] {
            let offset = |k: usize, skew| {
                let distance = width * widths[k] * (alignments[k] - side);
                Vec2::new(distance * skew, distance)
            };
            let start = offset(i, skew0);
            let finish = offset(j, skew1);
            let profile = edge::OffsetSpan {
                start,
                end: finish,
                length,
            };
            for (&seg, &[s0, s1]) in span.iter().zip(distance[first..=end].array_windows()) {
                if s0 != s1 {
                    profile.append(edge, seg, s0 - distance[first], s1 - distance[first])?;
                }
            }
        }
    }
    if right.is_empty() {
        return Ok(vec![]);
    }
    let cap = left.current_position().unwrap();
    let back = left.reverse_subpaths();
    if cubic.closed {
        right.close_path();
        right.extend(back);
    } else {
        right.line_to(cap);
        right.extend(back.into_iter().skip(1));
    }
    right.close_path();
    contours(right)
}

fn tangent(segment: PathSeg, t: f64) -> Vec2 {
    match segment {
        PathSeg::Line(line) => line.p1 - line.p0,
        _ => segment.to_cubic().deriv().eval(t).to_vec2(),
    }
    .normalize()
}

/// Tangential displacement per unit normal offset. At a corner the bisector
/// meets both offset edges at the miter. Beyond the miter limit the two
/// unextended ends are joined by a bevel.
fn knot_skew(tangent: Vec2, neighbor: Option<Vec2>, heading: Option<&kurbo::Point>) -> f64 {
    let direction = if let Some(other) = neighbor
        && tangent.dot(other) < 0.99
    {
        let bisector = (tangent + other).normalize();
        if !bisector.is_finite() || tangent.dot(bisector) < MITER_LIMIT.recip() {
            return 0.0;
        }
        bisector
    } else {
        heading.map_or(tangent, |p| p.to_vec2())
    };
    let dot = tangent.dot(direction);
    // A perpendicular heading cannot define a cap of finite length.
    if dot.abs() > 1e-8 {
        direction.cross(tangent) / dot
    } else {
        0.0
    }
}
