use std::{collections::BTreeMap, sync::Arc};

use monoxide_curves::{CubicBezier, SpiroCurve, point::Point2D, stroke::TangentOverride};

use crate::{FontParamSettings, dsl::IntoOutline};

#[derive(Debug, Clone)]
pub struct FontContext {
    /// The default glyph, used for characters that do not have a glyph
    /// assigned.
    ///
    /// This glyph should usually be the tofu glyph, and additionally assigned
    /// to U+FFFD REPLACEMENT CHARACTER.
    ///
    /// Finalizing the context without setting this will result in an error.
    pub(crate) tofu: Option<Arc<GlyphEntry>>,
    /// The regular character mapping. This mapping is used when no other
    /// replacements override the characters.
    pub(crate) cmap: BTreeMap<char, Arc<GlyphEntry>>,
    pub(crate) settings: FontParamSettings,
}

impl FontContext {
    pub fn new(settings: FontParamSettings) -> Self {
        Self {
            tofu: None,
            cmap: BTreeMap::new(),
            settings,
        }
    }

    /// Set the default glyph for the font. Also sets the tofu glyph to U+FFFD.
    pub fn set_tofu(&mut self, tofu: Arc<GlyphEntry>) {
        self.tofu = Some(tofu.clone());
        self.cmap.insert('\u{FFFD}', tofu);
    }

    /// Set the glyph of the given character. Returns the previous glyph
    /// if it was set, otherwise `None`.
    pub fn set_mapping(&mut self, ch: char, glyph: Arc<GlyphEntry>) -> Option<Arc<GlyphEntry>> {
        self.cmap.insert(ch, glyph)
    }

    pub fn settings(&self) -> &FontParamSettings {
        &self.settings
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlyphId(pub usize);

#[derive(Debug, Clone)]
pub enum GlyphEntry {
    Simple(SimpleGlyph),
    Compound(CompoundGlyph),
}

impl From<SimpleGlyph> for GlyphEntry {
    fn from(glyph: SimpleGlyph) -> Self {
        GlyphEntry::Simple(glyph)
    }
}

impl From<CompoundGlyph> for GlyphEntry {
    fn from(glyph: CompoundGlyph) -> Self {
        GlyphEntry::Compound(glyph)
    }
}

#[derive(Debug, Clone, Default)]
pub struct SimpleGlyph {
    pub outlines: Vec<Arc<OutlineExpr>>,
    /// The advance width of the glyph. If unset, uses the default advance width
    /// of the font.
    pub advance: Option<f64>,
}

impl SimpleGlyph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn outline(mut self, outline: impl IntoOutline) -> Self {
        self.outlines.push(outline.into_outline());
        self
    }

    pub fn outlines<I: IntoOutline>(mut self, outlines: impl IntoIterator<Item = I>) -> Self {
        for outline in outlines {
            self = self.outline(outline);
        }
        self
    }

    pub fn advance(mut self, advance: impl Into<Option<f64>>) -> Self {
        self.advance = advance.into();
        self
    }
}

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

#[derive(Debug, Clone, Default)]
pub struct CompoundGlyph {
    /// Index into the glyphs array of the font context.
    ///
    /// TODO: compound glyphs can transform their components
    pub components: Vec<Arc<GlyphEntry>>,
}
