use std::sync::Arc;

use monoxide_curves::{CubicBezier, SpiroCurve, point::Point2D, stroke::TangentOverride};

#[derive(Debug, Clone)]
pub enum OutlineExpr {
    Bezier(CubicBezier<Point2D>),
    Spiro(SpiroCurve, TangentOverride),
    Stroked(Arc<OutlineExpr>, f64),
    // TODO: transformed, etc.
}

impl Default for OutlineExpr {
    fn default() -> Self {
        OutlineExpr::Bezier(CubicBezier::builder(Point2D::new(0., 0.)).build())
    }
}

impl OutlineExpr {
    pub fn stroked(self: Arc<Self>, width: f64) -> Arc<Self> {
        Arc::new(OutlineExpr::Stroked(self, width))
    }
}
