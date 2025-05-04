use monoxide_curves::{CubicBezier, point::Point2D};
use serde::Serialize;

/// Represent the overall character mapping of a font
#[derive(Serialize)]
pub struct FontOverview {
    glyphs: Vec<GlyphOverview>,
}

/// Represents the minimal information to represent a glyph
#[derive(Serialize)]
pub struct GlyphOverview {
    /// The index of the current glyph, to be used in other interfaces.
    id: usize,
    /// The character this glyph represents
    ch: char,
    /// The name of the glyph, if any
    name: Option<String>,
    /// The outline(s) of the current glyph
    outline: Vec<String>,
}

/// Represent the detail of a glyph, including the comptation tree, debug points, etc.
#[derive(Serialize)]
pub struct GlyphDetail {
    overview: GlyphOverview,
    // TODO: [`monoxide-script::ast`] types mapped here
    construction: Vec<SerializedGlyphConstruction>,
}

#[derive(Serialize)]
pub struct SerializedGlyphConstruction {
    id: usize,

    /// The method to construct the glyph
    #[serde(flatten)]
    kind: ConstructionKind,

    /// The resulting curve of the construction, if any
    result_curve: Option<CubicBezier<Point2D>>,

    /// Auxiliary points for debugging
    debug_points: Vec<DebugPoint>,

    /// Auxiliary lines for debugging
    debug_lines: Vec<DebugLine>,
}

/// A point for debugging
#[derive(Serialize)]
pub struct DebugPoint {
    kind: &'static str,
    #[serde(flatten)]
    at: Point2D,
}

/// A line for debugging
#[derive(Serialize)]
pub struct DebugLine {
    from: Point2D,
    to: Point2D,
    tag: String,
}

#[derive(Serialize)]
#[serde(tag = "t", rename_all = "kebab-case")]
pub enum ConstructionKind {
    Spiro { curve: Vec<SerializeSpiroPoint> },
    CubicBezier { curve: CubicBezier<Point2D> },
    Stroke { parent: usize, width: f64 },
    BooleanAdd { parents: Vec<usize> },
}

#[derive(Serialize)]
pub struct SerializeSpiroPoint {
    #[serde(flatten)]
    point: Point2D,
    ty: SerializeSpiroKind,
}

impl Into<SerializeSpiroPoint> for monoxide_spiro::SpiroCp {
    fn into(self) -> SerializeSpiroPoint {
        SerializeSpiroPoint {
            point: Point2D::new(self.x, self.y),
            ty: self.ty.into(),
        }
    }
}

/// A version of spiro control point type that is designed to be serialized.
#[derive(Serialize, Copy, Clone, PartialEq, Eq, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum SerializeSpiroKind {
    Corner,
    G4,
    G2,
    Flat,
    Curl,
    Anchor,
    Handle,
    Open,
    EndOpen,
}

impl Into<SerializeSpiroKind> for monoxide_spiro::SpiroCpTy {
    fn into(self) -> SerializeSpiroKind {
        match self {
            monoxide_spiro::SpiroCpTy::Corner => SerializeSpiroKind::Corner,
            monoxide_spiro::SpiroCpTy::G4 => SerializeSpiroKind::G4,
            monoxide_spiro::SpiroCpTy::G2 => SerializeSpiroKind::G2,
            monoxide_spiro::SpiroCpTy::Left => SerializeSpiroKind::Flat,
            monoxide_spiro::SpiroCpTy::Right => SerializeSpiroKind::Curl,
            monoxide_spiro::SpiroCpTy::Anchor => SerializeSpiroKind::Anchor,
            monoxide_spiro::SpiroCpTy::Handle => SerializeSpiroKind::Handle,
            monoxide_spiro::SpiroCpTy::Open => SerializeSpiroKind::Open,
            monoxide_spiro::SpiroCpTy::EndOpen => SerializeSpiroKind::EndOpen,
        }
    }
}
