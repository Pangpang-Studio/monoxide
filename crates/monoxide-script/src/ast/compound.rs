use monoxide_curves::{point::Point2D, xform::Affine2D};

use crate::ast::Glyph;

/// Represents a component of a compound glyph, which is a reference to another
/// glyph.
#[derive(Debug, Clone)]
pub struct GlyphComponent {
    /// The component glyph to be included in the compound glyph.
    pub component: Glyph,
    /// The transformation applied to the component glyph.
    pub xform: Affine2D<Point2D>,
}
