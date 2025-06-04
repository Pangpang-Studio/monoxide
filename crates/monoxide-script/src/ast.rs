use std::{collections::BTreeMap, sync::Arc};

use monoxide_curves::{point::Point2D, stroke::TangentOverride, CubicBezier, SpiroCurve};

use crate::FontParamSettings;

#[derive(Debug, Clone)]
pub struct FontContext {
    pub glyphs: Vec<GlyphEntry>,
    pub cmap: BTreeMap<char, GlyphId>,
    pub settings: FontParamSettings,
}

impl FontContext {
    pub fn new(settings: FontParamSettings) -> Self {
        FontContext {
            glyphs: Vec::new(),
            cmap: BTreeMap::new(),
            settings,
        }
    }

    pub fn get_char_glyph_id(&self, c: char) -> Option<GlyphId> {
        self.cmap.get(&c).copied()
    }

    pub fn get_glyph(&self, id: GlyphId) -> Option<&GlyphEntry> {
        self.glyphs.get(id.0)
    }

    pub fn add_glyph(&mut self, v: GlyphEntry) -> GlyphId {
        let id = self.glyphs.len();
        self.glyphs.push(v);
        GlyphId(id)
    }

    pub fn assign_char(&mut self, char: char, glyph_id: GlyphId) {
        self.cmap.insert(char, glyph_id);
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
    pub fn new(outlines: impl IntoIterator<Item = Arc<OutlineExpr>>) -> Self {
        Self::with_advance(outlines, None)
    }

    pub fn with_advance(
        outlines: impl IntoIterator<Item = Arc<OutlineExpr>>,
        advance: impl Into<Option<f64>>,
    ) -> Self {
        Self {
            outlines: outlines.into_iter().collect(),
            advance: advance.into(),
        }
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
    pub components: Vec<usize>,
}
