use std::{collections::BTreeMap, sync::Arc};

use monoxide_curves::{point::Point2D, stroke::TangentOverride, CubicBezier, SpiroCurve};
use rquickjs::{
    class::{Trace, Tracer},
    JsLifetime,
};

use crate::FontParamSettings;

#[rquickjs::class]
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

unsafe impl JsLifetime<'_> for FontContext {
    type Changed<'to> = Self;
}

impl Trace<'_> for FontContext {
    fn trace(&self, _cx: Tracer) {
        // No need to trace, as we don't have any JS references in this struct.
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlyphId(pub usize);

#[derive(Debug, Clone)]
pub enum GlyphEntry {
    Simple(SimpleGlyph),
    Compound(CompoundGlyph),
}

#[derive(Debug, Clone, Default)]
pub struct SimpleGlyph {
    pub outlines: Vec<Arc<OutlineExpr>>,
    /// The advance width of the glyph. If unset, uses the default advance width
    /// of the font.
    pub advance: Option<f64>,
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

#[derive(Debug, Clone, Default)]
pub struct CompoundGlyph {
    /// Index into the glyphs array of the font context.
    pub components: Vec<usize>,
}
