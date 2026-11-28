use std::ops::Range;

use kurbo::{
    BezPath, CubicBez, CurveFitSample, ParamCurve, ParamCurveArclen, ParamCurveDeriv,
    ParamCurveFit, PathEl, Point, Vec2, fit_to_bezpath, offset::offset_cubic,
};

use super::{Knot, TANGENT_EPSILON, TOLERANCE, segment_tangent};
use crate::{
    CubicBezier, CubicSegment,
    cube::CubicSegmentFull,
    error::{Error, Result},
};

pub(super) fn stroke(
    curve: &CubicBezier,
    indices: &[usize],
    lengths: &[f64],
    knots: &[Knot],
) -> Result<BezPath> {
    let mut edges = Edges::default();
    let span_count = if curve.closed {
        knots.len()
    } else {
        knots.len() - 1
    };
    for i in 0..span_count {
        let next = (i + 1) % knots.len();
        let from = indices[i];
        let to = if next == 0 {
            curve.segment_count()
        } else {
            indices[next]
        };
        let span_length: f64 = lengths[from..to].iter().sum();
        if span_length <= TANGENT_EPSILON {
            if knots[i].width != knots[next].width
                || knots[i].alignment != knots[next].alignment
                || knots[i].slant != knots[next].slant
            {
                return Err(Error::InvalidStroke(
                    "conflicting attributes on coincident knots",
                ));
            }
            continue;
        }

        let mut preceding_length = 0.0;
        for (index, &length) in lengths.iter().enumerate().take(to).skip(from) {
            if length <= TANGENT_EPSILON {
                continue;
            }
            let segment = curve
                .segment(index)
                .ok_or_else(|| Error::internal("missing stroke segment"))?;
            let start_tangent = segment_tangent(&segment, false)
                .ok_or(Error::InvalidStroke("curve has no start tangent"))?;
            let end_tangent = segment_tangent(&segment, true)
                .ok_or(Error::InvalidStroke("curve has no end tangent"))?;
            let left = offset(
                &segment,
                length,
                preceding_length,
                span_length,
                knots[i],
                knots[next],
                true,
            );
            let right = offset(
                &segment,
                length,
                preceding_length,
                span_length,
                knots[i],
                knots[next],
                false,
            );
            edges.push(
                &left,
                &right,
                segment.start,
                segment.rest.last_point(),
                start_tangent,
                end_tangent,
            )?;
            preceding_length += length;
        }
    }
    edges.finish(curve.closed)
}

fn offset(
    segment: &CubicSegmentFull,
    length: f64,
    preceding_length: f64,
    span_length: f64,
    from: Knot,
    to: Knot,
    left: bool,
) -> BezPath {
    let cubic = match segment.rest {
        CubicSegment::Line(end) => CubicBez::new(
            segment.start,
            segment.start.lerp(end, 1.0 / 3.0),
            segment.start.lerp(end, 2.0 / 3.0),
            end,
        ),
        CubicSegment::Curve(c1, c2, end) => CubicBez::new(segment.start, c1, c2, end),
    };

    if from.width == to.width
        && from.alignment == to.alignment
        && from.slant == 0.0
        && to.slant == 0.0
        && from.width_slope == 0.0
        && to.width_slope == 0.0
        && from.alignment_slope == 0.0
        && to.alignment_slope == 0.0
        && from.slant_slope == 0.0
        && to.slant_slope == 0.0
    {
        let distance = from.width
            * if left {
                from.alignment
            } else {
                from.alignment - 1.0
            };
        let mut path = BezPath::new();
        if distance == 0.0 {
            path.move_to(cubic.p0);
            match segment.rest {
                CubicSegment::Line(end) => path.line_to(end),
                CubicSegment::Curve(c1, c2, end) => path.curve_to(c1, c2, end),
            }
            return path;
        }
        if matches!(segment.rest, CubicSegment::Line(_)) {
            let normal = (cubic.p3 - cubic.p0).normalize().turn_90();
            path.move_to(cubic.p0 + distance * normal);
            path.line_to(cubic.p3 + distance * normal);
            return path;
        }
        let chord = cubic.p3 - cubic.p0;
        let non_collinear = (cubic.p1 - cubic.p0).cross(chord).abs()
            + (cubic.p2 - cubic.p0).cross(chord).abs()
            > TOLERANCE * chord.hypot();
        if non_collinear {
            offset_cubic(cubic, distance, TOLERANCE, &mut path);
            return path;
        }
    }

    let path = fit_to_bezpath(
        &VariableOffset {
            cubic,
            length,
            preceding_length,
            span_length,
            from,
            to,
            left,
        },
        TOLERANCE,
    );
    straighten_lines(&path)
}

fn straighten_lines(path: &BezPath) -> BezPath {
    let mut result = BezPath::new();
    let mut current = Point::ORIGIN;
    for el in path.iter() {
        match el {
            PathEl::MoveTo(p) => {
                result.move_to(p);
                current = p;
            }
            PathEl::CurveTo(c1, c2, end) => {
                let chord = end - current;
                let length_sq = chord.hypot2();
                let linear = if length_sq <= TOLERANCE.powi(2) {
                    current.distance(c1) <= TOLERANCE && current.distance(c2) <= TOLERANCE
                } else {
                    let a = c1 - current;
                    let b = c2 - current;
                    let t1 = a.dot(chord) / length_sq;
                    let t2 = b.dot(chord) / length_sq;
                    a.cross(chord).abs() <= TOLERANCE * length_sq.sqrt()
                        && b.cross(chord).abs() <= TOLERANCE * length_sq.sqrt()
                        && 0.0 <= t1
                        && t1 <= t2
                        && t2 <= 1.0
                };
                if linear {
                    if end != current {
                        result.line_to(end);
                    }
                } else {
                    result.curve_to(c1, c2, end);
                }
                current = end;
            }
            other => {
                if let Some(end) = other.end_point() {
                    current = end;
                }
                result.push(other);
            }
        }
    }
    result
}

struct VariableOffset {
    cubic: CubicBez,
    length: f64,
    preceding_length: f64,
    span_length: f64,
    from: Knot,
    to: Knot,
    left: bool,
}

impl VariableOffset {
    fn sample(&self, t: f64, sign: f64) -> (Point, Vec2) {
        let derivative = self.cubic.deriv();
        let velocity = derivative.eval(t).to_vec2();
        let speed = velocity.hypot();
        let tangent = if speed > TANGENT_EPSILON {
            velocity / speed
        } else {
            [
                nearby_parameter(t, sign),
                nearby_parameter(t, -sign),
                0.0,
                0.5,
                1.0,
            ]
            .into_iter()
            .map(|parameter| derivative.eval(parameter).to_vec2())
            .find(|candidate| candidate.hypot() > 0.0)
            .expect("a drawable cubic has a tangent")
            .normalize()
        };
        let tangent_derivative = if speed > TANGENT_EPSILON {
            let acceleration = derivative.deriv().eval(t).to_vec2();
            (acceleration - tangent * tangent.dot(acceleration)) / speed
        } else {
            Vec2::ZERO
        };
        let length = if t <= 0.0 {
            0.0
        } else if t >= 1.0 {
            self.length
        } else {
            self.cubic.subsegment(0.0..t).arclen(TOLERANCE * 0.1)
        };
        let progress = ((self.preceding_length + length) / self.span_length).clamp(0.0, 1.0);
        let interpolate = |a, b, slope_a, slope_b| {
            let (value, derivative) = hermite(a, b, slope_a, slope_b, self.span_length, progress);
            (value, derivative * speed / self.span_length)
        };
        let (width, width_derivative) = interpolate(
            self.from.width,
            self.to.width,
            self.from.width_slope,
            self.to.width_slope,
        );
        let (alignment, alignment_derivative) = interpolate(
            self.from.alignment,
            self.to.alignment,
            self.from.alignment_slope,
            self.to.alignment_slope,
        );
        let (slant, slant_derivative) = interpolate(
            self.from.slant,
            self.to.slant,
            self.from.slant_slope,
            self.to.slant_slope,
        );
        let factor = if self.left {
            alignment
        } else {
            alignment - 1.0
        };
        let distance = width * factor;
        let distance_derivative = width_derivative * factor + width * alignment_derivative;
        let direction = tangent.turn_90() + slant * tangent;
        let direction_derivative =
            tangent_derivative.turn_90() + slant * tangent_derivative + slant_derivative * tangent;
        (
            self.cubic.eval(t) + distance * direction,
            velocity + distance_derivative * direction + distance * direction_derivative,
        )
    }
}

fn nearby_parameter(t: f64, sign: f64) -> f64 {
    let direction = if (t <= 0.0 && sign < 0.0) || (t >= 1.0 && sign > 0.0) {
        -sign
    } else {
        sign
    };
    let available = if direction > 0.0 { 1.0 - t } else { t };
    let nearby = t + direction * (available * 0.5).min(1e-6);
    if nearby == t {
        (t - direction * 1e-6).clamp(0.0, 1.0)
    } else {
        nearby
    }
}

fn hermite(a: f64, b: f64, slope_a: f64, slope_b: f64, length: f64, t: f64) -> (f64, f64) {
    let t2 = t * t;
    let t3 = t2 * t;
    let ma = slope_a * length;
    let mb = slope_b * length;
    let value = (2.0 * t3 - 3.0 * t2 + 1.0) * a
        + (t3 - 2.0 * t2 + t) * ma
        + (-2.0 * t3 + 3.0 * t2) * b
        + (t3 - t2) * mb;
    let derivative = (6.0 * t2 - 6.0 * t) * a
        + (3.0 * t2 - 4.0 * t + 1.0) * ma
        + (-6.0 * t2 + 6.0 * t) * b
        + (3.0 * t2 - 2.0 * t) * mb;
    (value, derivative)
}

impl ParamCurveFit for VariableOffset {
    fn sample_pt_tangent(&self, t: f64, sign: f64) -> CurveFitSample {
        let (p, mut tangent) = self.sample(t, sign);
        if tangent.hypot() <= TANGENT_EPSILON {
            tangent = self.sample(nearby_parameter(t, sign), sign).1;
        }
        CurveFitSample { p, tangent }
    }

    fn sample_pt_deriv(&self, t: f64) -> (Point, Vec2) {
        self.sample(t, if t < 0.5 { 1.0 } else { -1.0 })
    }

    fn break_cusp(&self, _range: Range<f64>) -> Option<f64> {
        None
    }
}

#[derive(Default)]
struct Edges {
    left: BezPath,
    right: BezPath,
    first_tangent: Vec2,
    last_tangent: Vec2,
    first_center: Point,
    last_center: Point,
}

impl Edges {
    fn push(
        &mut self,
        left: &BezPath,
        right: &BezPath,
        start: Point,
        end: Point,
        start_tangent: Vec2,
        end_tangent: Vec2,
    ) -> Result<()> {
        let left_start = start_point(left)?;
        let right_start = start_point(right)?;
        if self.left.is_empty() {
            self.left.extend(left.iter());
            self.right.extend(right.iter());
            self.first_center = start;
            self.first_tangent = start_tangent;
        } else {
            join(
                &mut self.left,
                left_start,
                start,
                self.last_tangent,
                start_tangent,
            )?;
            join(
                &mut self.right,
                right_start,
                start,
                self.last_tangent,
                start_tangent,
            )?;
            self.left.extend(left.iter().skip(1));
            self.right.extend(right.iter().skip(1));
        }
        self.last_center = end;
        self.last_tangent = end_tangent;
        Ok(())
    }

    fn finish(mut self, closed: bool) -> Result<BezPath> {
        if self.left.is_empty() {
            return Err(Error::InvalidStroke("curve has no drawable segments"));
        }
        if closed {
            if self.first_center.distance(self.last_center) > TOLERANCE {
                return Err(Error::internal(
                    "closed stroke does not return to its start",
                ));
            }
            let left_start = start_point(&self.left)?;
            let right_start = start_point(&self.right)?;
            join(
                &mut self.left,
                left_start,
                self.first_center,
                self.last_tangent,
                self.first_tangent,
            )?;
            join(
                &mut self.right,
                right_start,
                self.first_center,
                self.last_tangent,
                self.first_tangent,
            )?;
            self.left.close_path();
            self.right.close_path();
            let mut outline = self.left;
            outline.extend(self.right.reverse_subpaths().iter());
            Ok(outline)
        } else {
            let mut outline = self.left;
            let reversed = self.right.reverse_subpaths();
            outline.line_to(start_point(&reversed)?);
            outline.extend(reversed.iter().skip(1));
            outline.close_path();
            Ok(outline)
        }
    }
}

fn start_point(path: &BezPath) -> Result<Point> {
    match path.elements().first() {
        Some(PathEl::MoveTo(point)) => Ok(*point),
        _ => Err(Error::internal("offset curve has no starting point")),
    }
}

fn join(
    path: &mut BezPath,
    next: Point,
    center: Point,
    incoming: Vec2,
    outgoing: Vec2,
) -> Result<()> {
    let previous = path
        .elements()
        .last()
        .and_then(PathEl::end_point)
        .ok_or_else(|| Error::internal("offset curve has no endpoint"))?;
    if previous.distance(next) <= TOLERANCE * 0.01 {
        return Ok(());
    }
    let cross = incoming.cross(outgoing);
    let dot = incoming.dot(outgoing);
    if dot > 0.0 && cross.abs() <= 1e-6 {
        path.line_to(next);
        return Ok(());
    }
    let distance = (previous - center).dot(incoming.turn_90());
    if cross * distance < 0.0 && cross.abs() > 1e-12 {
        let t = (next - previous).cross(outgoing) / cross;
        let miter = previous + t * incoming;
        let next_distance = (next - center).dot(outgoing.turn_90()).abs();
        let limit = 4.0 * distance.abs().max(next_distance);
        if miter.distance(center) <= limit + TOLERANCE {
            path.line_to(miter);
        }
    } else if cross * distance > 0.0 {
        path.line_to(center);
    }
    path.line_to(next);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stationary_endpoints_have_finite_outward_tangent_samples() {
        let knot = Knot {
            width: 1.0,
            alignment: 0.5,
            slant: 0.0,
            width_slope: 0.0,
            alignment_slope: 0.0,
            slant_slope: 0.0,
        };
        let offset = VariableOffset {
            cubic: CubicBez::new((0.0, 0.0), (0.0, 0.0), (2.0, 0.0), (2.0, 0.0)),
            length: 2.0,
            preceding_length: 0.0,
            span_length: 2.0,
            from: knot,
            to: knot,
            left: true,
        };

        for (t, sign) in [(0.0, -1.0), (1e-14, -1.0), (1.0 - 1e-14, 1.0), (1.0, 1.0)] {
            let sample = offset.sample_pt_tangent(t, sign);
            assert!(sample.p.is_finite());
            assert!(sample.tangent.is_finite() && sample.tangent.hypot() > 0.0);
        }

        let segment = CubicSegmentFull {
            start: Point::ORIGIN,
            rest: CubicSegment::Curve(Point::ORIGIN, Point::new(2.0, 0.0), Point::new(2.0, 0.0)),
        };
        let path = super::offset(
            &segment,
            2.0,
            0.0,
            2.0,
            knot,
            Knot { width: 2.0, ..knot },
            true,
        );
        assert!(path.is_finite());
        assert!(path.elements().len() >= 2);
    }
}
