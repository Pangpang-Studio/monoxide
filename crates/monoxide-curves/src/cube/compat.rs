use flo_curves::{
    Geo,
    bezier::path::{BezierPath, BezierPathFactory},
};

use crate::point::Point2D;

use super::{CubicBezier, CubicSegment};

impl Geo for CubicBezier<Point2D> {
    type Point = Point2D;
}

pub struct FloCurvesPointIter<'a> {
    it: std::slice::Iter<'a, CubicSegment<Point2D>>,
}

impl Iterator for FloCurvesPointIter<'_> {
    type Item = (Point2D, Point2D, Point2D);

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(|x| match x {
            CubicSegment::Line(p) => (*p, *p, *p),
            CubicSegment::Curve(p1, p2, p3) => (*p1, *p2, *p3),
        })
    }
}

impl BezierPath for CubicBezier<Point2D> {
    type PointIter<'a> = FloCurvesPointIter<'a>;

    fn start_point(&self) -> Self::Point {
        self.start
    }

    fn points(&self) -> Self::PointIter<'_> {
        FloCurvesPointIter {
            it: self.segments.iter(),
        }
    }
}

impl BezierPathFactory for CubicBezier<Point2D> {
    fn from_points<FromIter: IntoIterator<Item = (Self::Point, Self::Point, Self::Point)>>(
        start_point: Self::Point,
        points: FromIter,
    ) -> Self {
        let points = points
            .into_iter()
            .map(|(p1, p2, p3)| {
                if p1 == p2 && p2 == p3 {
                    CubicSegment::Line(p1)
                } else {
                    CubicSegment::Curve(p1, p2, p3)
                }
            })
            .collect::<Vec<_>>();
        let closed = points.last().is_some_and(|p| p.last_point() == start_point);
        CubicBezier {
            start: start_point,
            segments: points,
            closed,
        }
    }
}
