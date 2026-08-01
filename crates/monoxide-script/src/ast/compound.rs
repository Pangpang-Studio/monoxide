use monoxide_curves::xform::{Affine2D, AffineExt};

use crate::ast::Glyph;

/// Represents a component of a compound glyph, which is a reference to another
/// glyph.
#[derive(Debug, Clone)]
pub struct GlyphComponent {
    /// The component glyph to be included in the compound glyph.
    pub component: Glyph,
    /// The transformation applied to the component glyph.
    pub xform: Affine2D,
}

impl GlyphComponent {
    pub fn from_glyph(glyph: Glyph) -> Self {
        GlyphComponent {
            component: glyph,
            xform: Affine2D::id(),
        }
    }

    pub fn with_xform(mut self, xform: Affine2D) -> Self {
        self.xform = xform;
        self
    }
}

impl From<Glyph> for GlyphComponent {
    fn from(val: Glyph) -> Self {
        GlyphComponent {
            component: val,
            xform: Affine2D::id(),
        }
    }
}
