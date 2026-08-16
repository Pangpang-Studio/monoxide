use std::sync::Arc;

use linesweeper::BinaryOp;
use monoxide_curves::{CubicBezier, SpiroCurve, point::Point2D, xform::Affine2D};

#[derive(Debug, Clone)]
pub enum OutlineExpr {
    Bezier(CubicBezier),
    Spiro(SpiroCurve),
    Bool(BinaryOp, Arc<OutlineExpr>, Arc<OutlineExpr>),
    Stroked(Arc<OutlineExpr>, f64),
    Transformed(Arc<OutlineExpr>, Affine2D),
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

    pub fn transformed(self: Arc<Self>, xform: Affine2D) -> Arc<Self> {
        Arc::new(OutlineExpr::Transformed(self, xform))
    }

    pub fn bool(self: Arc<Self>, other: Arc<Self>, op: BinaryOp) -> Arc<Self> {
        Arc::new(OutlineExpr::Bool(op, self, other))
    }
}
