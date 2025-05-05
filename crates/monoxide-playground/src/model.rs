use monoxide_curves::{CubicBezier, point::Point2D};
use serde::Serialize;

/// Represent the overall character mapping of a font
#[derive(Serialize)]
pub struct FontOverview {
    pub glyphs: Vec<GlyphOverview>,
}

/// Represents the minimal information to represent a glyph
#[derive(Serialize)]
pub struct GlyphOverview {
    /// The index of the current glyph, to be used in other interfaces.
    pub id: usize,
    /// The character this glyph represents
    pub ch: Option<char>,
    /// The name of the glyph, if any
    pub name: Option<String>,
    /// The outline(s) of the current glyph
    pub outline: Vec<CubicBezier<Point2D>>,
}

/// A guideline in either horizontal or vertical direction, with a position
/// and tag
#[derive(Serialize)]
pub struct Guideline {
    pub pos: f64,
    pub label: Option<String>,
}

/// The guidelines of a glyph, including horizontal and vertical ones
#[derive(Serialize)]
pub struct Guidelines {
    pub h: Vec<Guideline>,
    pub v: Vec<Guideline>,
}

/// Represent the detail of a glyph, including the comptation tree, debug
/// points, etc.
#[derive(Serialize)]
pub struct GlyphDetail {
    pub overview: GlyphOverview,
    pub guidelines: Guidelines,
    pub construction: Vec<SerializedGlyphConstruction>,
    pub result_id: Option<usize>,
    pub errors: Vec<String>,
}

#[derive(Serialize)]
pub struct SerializedGlyphConstruction {
    pub id: usize,

    /// The method to construct the glyph
    pub kind: ConstructionKind,

    /// The resulting curve of the construction, if any
    pub result_curve: Option<Vec<CubicBezier<Point2D>>>,

    /// Auxiliary points for debugging
    pub debug_points: Vec<DebugPoint>,

    /// Auxiliary lines for debugging
    pub debug_lines: Vec<DebugLine>,
}

impl SerializedGlyphConstruction {
    pub fn new(id: usize, kind: ConstructionKind) -> Self {
        SerializedGlyphConstruction {
            id,
            kind,
            result_curve: None,
            debug_points: vec![],
            debug_lines: vec![],
        }
    }
}

/// A point for debugging
#[derive(Serialize)]
pub struct DebugPoint {
    pub kind: &'static str,
    pub tag: String,
    #[serde(flatten)]
    pub at: Point2D,
}

/// A line for debugging
#[derive(Serialize)]
pub struct DebugLine {
    pub from: Point2D,
    pub to: Point2D,
    pub tag: String,
}

#[derive(Serialize)]
#[serde(tag = "t", rename_all = "kebab-case")]
pub enum ConstructionKind {
    Spiro {
        curve: Vec<Vec<SerializeSpiroPoint>>,
    },
    CubicBezier {
        curve: Vec<CubicBezier<Point2D>>,
    },
    Stroke {
        parent: usize,
        width: f64,
        curve: Vec<Vec<SerializeSpiroPoint>>,
    },
    SpiroToBezier {
        parent: usize,
    },
    BooleanAdd {
        parents: Vec<usize>,
    },
    /// A placeholder when the construction is not yet complete.
    Placeholder,
}

#[derive(Serialize)]
pub struct SerializeSpiroPoint {
    #[serde(flatten)]
    pub point: Point2D,
    pub ty: SerializeSpiroKind,
}

impl From<monoxide_spiro::SpiroCp> for SerializeSpiroPoint {
    fn from(value: monoxide_spiro::SpiroCp) -> Self {
        SerializeSpiroPoint {
            point: Point2D::new(value.x, value.y),
            ty: value.ty.into(),
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

impl From<monoxide_spiro::SpiroCpTy> for SerializeSpiroKind {
    fn from(value: monoxide_spiro::SpiroCpTy) -> Self {
        match value {
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
