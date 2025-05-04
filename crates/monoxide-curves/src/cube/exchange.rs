//! Data exchange format for the CubicBezier path

use serde::{Deserialize, Serialize};

use super::CubicSegment;

#[derive(Clone, Debug, PartialEq, Copy, Serialize, Deserialize)]
#[serde(tag = "t")]
pub enum SerdeForCubicSegment<P> {
    Line { p2: P },
    Curve { c1: P, c2: P, p2: P },
}

impl<P> From<SerdeForCubicSegment<P>> for CubicSegment<P> {
    fn from(val: SerdeForCubicSegment<P>) -> Self {
        match val {
            SerdeForCubicSegment::Line { p2 } => CubicSegment::Line(p2),
            SerdeForCubicSegment::Curve { c1, c2, p2 } => CubicSegment::Curve(c1, c2, p2),
        }
    }
}

impl<P> From<CubicSegment<P>> for SerdeForCubicSegment<P> {
    fn from(seg: CubicSegment<P>) -> Self {
        match seg {
            CubicSegment::Line(p2) => SerdeForCubicSegment::Line { p2 },
            CubicSegment::Curve(c1, c2, p2) => SerdeForCubicSegment::Curve { c1, c2, p2 },
        }
    }
}
