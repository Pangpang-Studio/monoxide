use std::sync::Arc;

use monoxide_curves::{CubicBezier, SpiroCurve, point::Point2D, xform::Affine2D};

#[derive(Debug, Clone)]
pub enum OutlineExpr {
    Bezier(CubicBezier<Point2D>),
    Spiro(SpiroCurve),
    Stroked(Arc<OutlineExpr>, f64),
    Transformed(Arc<OutlineExpr>, Affine2D<Point2D>),
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

    pub fn transformed(self: Arc<Self>, xform: Affine2D<Point2D>) -> Arc<Self> {
        Arc::new(OutlineExpr::Transformed(self, xform))
    }
}
