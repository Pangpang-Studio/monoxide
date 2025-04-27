use std::{collections::BTreeMap, rc::Rc};

use monoxide_curves::{point::Point2D, stroke::TangentOverride, CubicBezier, SpiroCurve};
use rquickjs::{
    class::{Trace, Tracer},
    JsLifetime,
};

#[rquickjs::class]
#[derive(Debug, Default, Clone)]
pub struct FontContext {
    pub glyphs: Vec<GlyphEntry>,
    pub cmap: BTreeMap<char, GlyphId>,
}

impl FontContext {
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
    pub outlines: Vec<Rc<OutlineExpr>>,
}

#[derive(Debug, Clone)]
pub enum OutlineExpr {
    Bezier(CubicBezier<Point2D>),
    Spiro(SpiroCurve, TangentOverride),
    Stroked(Rc<OutlineExpr>, f64),
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
