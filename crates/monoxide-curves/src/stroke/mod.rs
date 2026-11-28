//! Expand strokes into cubic Bézier outlines.

mod variable;
mod widths;

use kurbo::{BezPath, Cap, Join, ParamCurveArclen, PathEl, PathSeg, Stroke, StrokeOpts, Vec2};
use kurbo_se::{StrokeSide, StrokeStyle, stroke_aligned};
use monoxide_spiro::{SpiroCp, SpiroCpTy};

use crate::{
    CubicBezier, CubicSegment, SpiroCurve,
    convert::spiro_to_cube_with_indices,
    cube::CubicSegmentFull,
    debug::CurveDebugger,
    error::{Error, Result},
    point::Point2D,
};

const TOLERANCE: f64 = 1e-5;
const TANGENT_EPSILON: f64 = 1e-12;

#[derive(Clone, Copy, Debug)]
struct Knot {
    width: f64,
    alignment: f64,
    slant: f64,
    width_slope: f64,
    alignment_slope: f64,
    slant_slope: f64,
}

/// Stroke a single Spiro contour, preserving its per-knot stroke attributes.
///
/// The returned contours use clockwise outer boundaries and counterclockwise
/// holes, matching the rest of the font pipeline.
pub fn stroke_spiro(
    curve: &SpiroCurve,
    width: f64,
    dbg: &mut impl CurveDebugger,
) -> Result<Vec<CubicBezier>> {
    ensure_single_piece(&curve.points)?;
    validate_width(width)?;
    validate_attributes(curve)?;

    let closed = curve.points[0].ty != SpiroCpTy::Open;
    let (mut cubics, indices) = spiro_to_cube_with_indices(&curve.points)?;
    if cubics.len() != 1 || indices.iter().any(|idx| idx.curve_index != 0) {
        return Err(Error::internal(
            "a single Spiro contour must produce one cubic contour",
        ));
    }
    let mut cubic = cubics.remove(0);
    if closed
        && let Some(last) = cubic.segments.last()
        && last.last_point() != cubic.start
    {
        cubic.segments.push(CubicSegment::Line(cubic.start));
    }
    let mut indices: Vec<_> = indices.into_iter().map(|idx| idx.segment_index).collect();
    if !closed {
        indices.push(cubic.segment_count());
    }
    if indices.len() != curve.points.len()
        || indices.first() != Some(&0)
        || indices.windows(2).any(|pair| pair[0] > pair[1])
        || indices.iter().any(|&idx| idx > cubic.segment_count())
        || cubic.closed != closed
    {
        return Err(Error::internal("invalid Spiro knot indices"));
    }

    let path = cubic.to_kurbo();
    if !path.is_finite() {
        return Err(Error::InvalidStroke("non-finite curve coordinates"));
    }
    if width == 0.0 {
        return Ok(Vec::new());
    }

    let lengths: Vec<_> = cubic
        .segment_iter()
        .map(|seg| PathSeg::from(seg).arclen(TOLERANCE * 0.1))
        .collect();
    let mut cumulative = Vec::with_capacity(lengths.len() + 1);
    cumulative.push(0.0);
    for length in &lengths {
        cumulative.push(cumulative.last().copied().unwrap() + length);
    }
    let total_length = *cumulative.last().unwrap();
    if !total_length.is_finite() || total_length <= TANGENT_EPSILON {
        return Err(Error::InvalidStroke("curve has no drawable length"));
    }
    let positions: Vec<_> = indices.iter().map(|&idx| cumulative[idx]).collect();
    let factors = widths::interpolate(&curve.width_factors, &positions, total_length, closed, 1.0)?;
    let alignments = widths::interpolate(&curve.alignment, &positions, total_length, closed, 0.5)?;

    let mut knots = Vec::with_capacity(curve.points.len());
    for (idx, ((&factor, &alignment), &segment_index)) in
        factors.iter().zip(&alignments).zip(&indices).enumerate()
    {
        let incoming = find_tangent(&cubic, segment_index, false);
        let outgoing = find_tangent(&cubic, segment_index, true);
        let tangent = match (incoming, outgoing) {
            (Some(a), Some(b)) if a.dot(b) > 0.99 => Some(a),
            (Some(_), Some(_)) => None,
            (Some(a), None) | (None, Some(a)) => Some(a),
            (None, None) => return Err(Error::InvalidStroke("curve has no tangent")),
        };
        let mut slant = 0.0;
        if let (Some(tangent), Some(heading)) = (tangent, curve.tangents.get(&idx)) {
            let heading = heading.to_vec2().normalize();
            let dot = tangent.dot(heading);
            if dot.abs() < 1e-8 {
                return Err(Error::InvalidStroke(
                    "stroke heading is perpendicular to the curve",
                ));
            }
            slant = tangent.dot(heading.turn_90()) / dot;
        }
        let knot = Knot {
            width: width * factor,
            alignment,
            slant,
            width_slope: 0.0,
            alignment_slope: 0.0,
            slant_slope: 0.0,
        };
        let left_offset = knot.width * alignment;
        let right_offset = knot.width * (alignment - 1.0);
        if !knot.width.is_finite()
            || !knot.slant.is_finite()
            || !left_offset.is_finite()
            || !right_offset.is_finite()
            || !(left_offset * slant).is_finite()
            || !(right_offset * slant).is_finite()
        {
            return Err(Error::InvalidStroke("stroke attributes are too large"));
        }
        if let Some(tangent) = tangent {
            let cp = Point2D::new(curve.points[idx].x, curve.points[idx].y);
            let offset = -knot.width * (1.0 - alignment);
            dbg.line(
                cp,
                cp + offset * (tangent.turn_90() + slant * tangent),
                format_args!(""),
            );
        }
        knots.push(knot);
    }

    let width_slopes = widths::slopes(
        &knots.iter().map(|knot| knot.width).collect::<Vec<_>>(),
        &positions,
        total_length,
        closed,
    );
    let alignment_slopes = widths::slopes(&alignments, &positions, total_length, closed);
    let slant_slopes = widths::slopes(
        &knots.iter().map(|knot| knot.slant).collect::<Vec<_>>(),
        &positions,
        total_length,
        closed,
    );
    for (i, knot) in knots.iter_mut().enumerate() {
        knot.width_slope = width_slopes[i];
        knot.alignment_slope = alignment_slopes[i];
        knot.slant_slope = slant_slopes[i];
        if !knot.width_slope.is_finite()
            || !knot.alignment_slope.is_finite()
            || !knot.slant_slope.is_finite()
        {
            return Err(Error::InvalidStroke("stroke attributes are too large"));
        }
    }

    let first = knots[0];
    let uniform = knots.iter().all(|knot| {
        knot.width == first.width && knot.alignment == first.alignment && knot.slant.abs() < 1e-10
    });
    if uniform && matches!(first.alignment, 0.0 | 0.5 | 1.0) {
        return stroke_uniform(&cubic, first.width, first.alignment);
    }

    let outline = variable::stroke(&cubic, &indices, &lengths, &knots)?;
    path_to_contours(&outline)
}

/// Stroke a cubic Bézier contour with a centered, butt-capped stroke.
pub fn stroke_bezier(curve: &CubicBezier, width: f64) -> Result<Vec<CubicBezier>> {
    validate_width(width)?;
    stroke_uniform(curve, width, 0.5)
}

fn stroke_uniform(curve: &CubicBezier, width: f64, alignment: f64) -> Result<Vec<CubicBezier>> {
    let path = curve.to_kurbo();
    if !path.is_finite() {
        return Err(Error::InvalidStroke("non-finite curve coordinates"));
    }
    if width == 0.0 {
        return Ok(Vec::new());
    }
    let outline = if alignment == 0.5 {
        let style = Stroke::new(width)
            .with_join(Join::Miter)
            .with_caps(Cap::Butt);
        kurbo::stroke(path.iter(), &style, &StrokeOpts::default(), TOLERANCE)
    } else {
        // kurbo-se names sides in Y-down coordinates; our font uses Y-up.
        let side = if alignment == 0.0 {
            StrokeSide::Left
        } else {
            StrokeSide::Right
        };
        let style = StrokeStyle::new(width).with_side(side);
        stroke_aligned(path, &style, TOLERANCE)
    };
    path_to_contours(&outline.reverse_subpaths())
}

fn path_to_contours(path: &BezPath) -> Result<Vec<CubicBezier>> {
    if !path.is_finite() {
        return Err(Error::InvalidStroke(
            "stroke produced non-finite coordinates",
        ));
    }
    let mut contours = Vec::new();
    for subpath in path.subpaths() {
        let Some(PathEl::MoveTo(start)) = subpath.first().copied() else {
            return Err(Error::internal("stroke contour has no starting point"));
        };
        let mut builder = CubicBezier::builder(start);
        let mut current = start;
        for el in &subpath[1..] {
            match *el {
                PathEl::MoveTo(_) => return Err(Error::internal("unexpected stroke subpath")),
                PathEl::LineTo(end) => {
                    builder.line_to(end);
                    current = end;
                }
                PathEl::QuadTo(control, end) => {
                    let cubic = kurbo::QuadBez::new(current, control, end).raise();
                    builder.curve_to(cubic.p1, cubic.p2, cubic.p3);
                    current = end;
                }
                PathEl::CurveTo(c1, c2, end) => {
                    builder.curve_to(c1, c2, end);
                    current = end;
                }
                PathEl::ClosePath => {
                    builder.close();
                }
            }
        }
        let contour = builder.build();
        if !contour.segments.is_empty() {
            contours.push(contour);
        }
    }
    Ok(contours)
}

fn validate_width(width: f64) -> Result<()> {
    if !width.is_finite() || width < 0.0 {
        return Err(Error::InvalidStroke(
            "stroke width must be finite and non-negative",
        ));
    }
    Ok(())
}

fn validate_attributes(curve: &SpiroCurve) -> Result<()> {
    if curve
        .points
        .iter()
        .any(|p| !p.x.is_finite() || !p.y.is_finite())
    {
        return Err(Error::InvalidStroke("non-finite curve coordinates"));
    }
    if curve
        .width_factors
        .iter()
        .any(|(&idx, &value)| idx >= curve.len() || !value.is_finite() || value < 0.0)
    {
        return Err(Error::InvalidStroke("invalid stroke width factor"));
    }
    if curve
        .alignment
        .iter()
        .any(|(&idx, &value)| idx >= curve.len() || !value.is_finite())
    {
        return Err(Error::InvalidStroke("invalid stroke alignment"));
    }
    if curve.tangents.iter().any(|(&idx, tangent)| {
        idx >= curve.len()
            || !tangent.x.is_finite()
            || !tangent.y.is_finite()
            || tangent.to_vec2().hypot() <= TANGENT_EPSILON
    }) {
        return Err(Error::InvalidStroke("invalid stroke heading"));
    }
    Ok(())
}

fn ensure_single_piece(curve: &[SpiroCp]) -> Result<()> {
    use SpiroCpTy::*;

    let single = match curve {
        [] | [_] => false,
        [
            SpiroCp { ty: Open, .. },
            middle @ ..,
            SpiroCp { ty: EndOpen, .. },
        ] => !middle.iter().any(|cp| matches!(cp.ty, Open | EndOpen)),
        [
            SpiroCp {
                ty: Open | EndOpen, ..
            },
            ..,
        ] => false,
        closed => !closed.iter().any(|cp| matches!(cp.ty, Open | EndOpen)),
    };
    if single {
        Ok(())
    } else {
        Err(Error::SpiroBroken)
    }
}

fn segment_tangent(segment: &CubicSegmentFull, at_end: bool) -> Option<Vec2> {
    let candidates = match segment.rest {
        CubicSegment::Line(end) => [end - segment.start; 3],
        CubicSegment::Curve(c1, c2, end) if at_end => [end - c2, end - c1, end - segment.start],
        CubicSegment::Curve(c1, c2, end) => {
            [c1 - segment.start, c2 - segment.start, end - segment.start]
        }
    };
    candidates
        .into_iter()
        .find(|v| v.hypot() > TANGENT_EPSILON)
        .map(Vec2::normalize)
}

fn find_tangent(curve: &CubicBezier, index: usize, forward: bool) -> Option<Vec2> {
    let n = curve.segment_count();
    for step in 0..n {
        let segment_index = if forward {
            let i = index + step;
            if i >= n && !curve.closed {
                break;
            }
            i % n
        } else {
            if step >= index && !curve.closed {
                break;
            }
            (index + n - 1 - step) % n
        };
        if let Some(tangent) = segment_tangent(&curve.segment(segment_index)?, !forward) {
            return Some(tangent);
        }
    }
    None
}

#[cfg(test)]
mod tests;
