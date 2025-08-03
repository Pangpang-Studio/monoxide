use std::{collections::BTreeMap, ops::Deref, sync::Arc};

use crate::{FontParamSettings, dsl::IntoOutline};

mod compound;
mod simple;

pub use compound::GlyphComponent;
pub use simple::OutlineExpr;

#[derive(Debug, Clone)]
pub struct FontContext {
    /// The default glyph, used for characters that do not have a glyph
    /// assigned.
    ///
    /// This glyph should usually be the tofu glyph, and additionally assigned
    /// to U+FFFD REPLACEMENT CHARACTER.
    ///
    /// Finalizing the context without setting this will result in an error.
    pub(crate) tofu: Option<Glyph>,
    /// The regular character mapping. This mapping is used when no other
    /// replacements override the characters.
    pub(crate) cmap: BTreeMap<char, Glyph>,
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
    pub fn set_tofu(&mut self, tofu: Glyph) {
        self.tofu = Some(tofu.clone());
        self.cmap.insert('\u{FFFD}', tofu);
    }

    /// Set the glyph of the given character. Returns the previous glyph
    /// if it was set, otherwise `None`.
    pub fn set_mapping(&mut self, ch: char, glyph: Glyph) -> Option<Glyph> {
        self.cmap.insert(ch, glyph)
    }

    pub fn settings(&self) -> &FontParamSettings {
        &self.settings
    }
}

/// An opaque glyph type that cannot be modified once built.
///
/// For building a [`Glyph`], see [`GlyphBuilder`], or [`Glyph::build()`].
#[derive(Debug, Clone, Default)]
pub struct Glyph(Arc<GlyphInner>);

impl Glyph {
    /// Create a new glyph using [`GlyphBuilder`].
    pub fn builder() -> GlyphBuilder {
        GlyphBuilder::new()
    }

    pub(crate) fn from_inner(inner: GlyphInner) -> Self {
        Glyph(Arc::new(inner))
    }

    pub(crate) fn inner(&self) -> &GlyphInner {
        &self.0
    }
}

impl Deref for Glyph {
    type Target = GlyphInner;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

#[derive(Debug, Clone, Default)]
pub struct GlyphInner {
    /// The outlines contained by this glyph.
    ///
    /// If both `outlines` and `subglyphs`
    pub(crate) outlines: Vec<Arc<OutlineExpr>>,

    /// The other glyphs that are inserted into this glyph.
    pub(crate) components: Vec<GlyphComponent>,

    /// The advance width of the glyph. If unset, uses the default advance width
    /// of the font.
    pub(crate) advance: Option<f64>,
}

/// The type to use for building a glyph.
#[derive(Debug, Clone, Default)]
pub struct GlyphBuilder {
    inner: GlyphInner,
}

impl GlyphBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn outline(mut self, outline: impl IntoOutline) -> Self {
        self.inner.outlines.push(outline.into_outline());
        self
    }

    pub fn outlines<I: IntoOutline>(mut self, outlines: impl IntoIterator<Item = I>) -> Self {
        for outline in outlines {
            self = self.outline(outline);
        }
        self
    }

    pub fn component<C: Into<GlyphComponent>>(mut self, component: C) -> Self {
        self.inner.components.push(component.into());
        self
    }

    pub fn components<I: Into<GlyphComponent>>(
        mut self,
        components: impl IntoIterator<Item = I>,
    ) -> Self {
        for component in components {
            self = self.component(component);
        }
        self
    }

    pub fn advance(mut self, advance: impl Into<Option<f64>>) -> Self {
        self.inner.advance = advance.into();
        self
    }

    pub fn build(self) -> Glyph {
        Glyph(Arc::new(self.inner))
    }
}
