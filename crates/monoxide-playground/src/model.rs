use std::collections::HashMap;

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
pub struct GlyphDetail {
    overview: GlyphOverview,
    // TODO: [`monoxide-script::ast`] types mapped here
}
