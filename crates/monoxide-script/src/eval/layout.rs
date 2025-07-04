use std::{
    collections::{BTreeMap, HashMap, HashSet},
    ops::Not,
};

use crate::{
    ast::{FontContext, Glyph, GlyphInner},
    eval::{HighEvalError, SerializedFontContext, SerializedGlyph, SerializedGlyphKind},
    util::RefId,
};

/// Lays out all glyphs referenced within a [`FontContext`] into a linear list.
pub fn layout_glyphs(cx: &FontContext) -> Result<SerializedFontContext, HighEvalError> {
    // We perform two DFSes.
    // 1. Split glyphs into their simple and compound parts
    // 2. Serialize them into a vector of glyphs
    if cx.tofu.is_none() {
        return Err(HighEvalError::TofuUnset);
    }

    let split_glyphs = split(cx);
    let mut ser = GlyphSerializer::new(cx, split_glyphs);
    ser.serialize();

    Ok(ser.build())
}

/// DFS into glyphs and split them into their simple and compound parts. Returns
/// a map of glyphs to their simple parts, if any.
///
/// After this function finishes, one will only need to check if the compound
/// part is empty to determine if the glyph is simple or compound.
fn split(cx: &FontContext) -> HashMap<RefId<GlyphInner>, Glyph> {
    let mut vis = HashSet::<RefId<GlyphInner>>::new();
    let mut res = HashMap::<RefId<GlyphInner>, Glyph>::new();
    let mut stack = Vec::<&GlyphInner>::new();

    stack.push(cx.tofu.as_ref().expect("Should be checked").inner());
    for glyph in cx.cmap.values() {
        stack.push(glyph.inner());
    }

    while let Some(glyph) = stack.pop() {
        if vis.contains(&glyph.into()) {
            continue;
        }
        vis.insert(glyph.into());

        if !glyph.outlines.is_empty() && !glyph.components.is_empty() {
            // This glyph needs to split into simple and compound parts.
            let new_glyph = Glyph::from_inner(GlyphInner {
                outlines: glyph.outlines.clone(),
                components: vec![],
                advance: glyph.advance,
            });
            res.insert(glyph.into(), new_glyph);
        }
    }

    res
}

struct GlyphSerializer<'a> {
    cx: &'a FontContext,
    split_glyphs: HashMap<RefId<'a, GlyphInner>, Glyph>,

    /// Map from glyph instances to their assigned IDs in the glyph list.
    map: HashMap<RefId<'a, GlyphInner>, usize>,
    /// Built `cmap` table
    cmap: BTreeMap<char, usize>,

    /// List of glyphs not yet transformed into [`SerializedGlyph`]
    glyphs: Vec<Glyph>,
    /// DFS stack
    stack: Vec<Glyph>,
}

impl<'a> GlyphSerializer<'a> {
    fn new(cx: &'a FontContext, split_glyphs: HashMap<RefId<'a, GlyphInner>, Glyph>) -> Self {
        Self {
            cx,
            split_glyphs,
            map: HashMap::new(),
            glyphs: Vec::new(),
            cmap: BTreeMap::new(),
            stack: Vec::new(),
        }
    }

    fn assign_id(&mut self, glyph: &'a Glyph) -> usize {
        let next_id = self.glyphs.len();
        if let Some(id) = self.map.get(&glyph.inner().into()) {
            return *id;
        }
        self.map.insert(glyph.inner().into(), next_id);
        self.glyphs.push(glyph.clone());
        next_id
    }

    fn serialize(&mut self) {
        // TOFU must be at index 0
        let tofu = self.cx.tofu.as_ref().expect("Should be checked");
        let tofu_id = self.assign_id(tofu);
        self.stack.push(tofu.clone());
        assert_eq!(tofu_id, 0);

        // The rest of the root glyphs need to be laid out continuously after TOFU.
        for (&ch, glyph) in &self.cx.cmap {
            let id = self.assign_id(glyph);
            self.cmap.insert(ch, id);
            self.stack.push(glyph.clone());
        }

        // Now we can start the DFS.
        while let Some(glyph) = self.stack.pop() {
            if glyph.inner().components.is_empty() {
                // Simple glyph
            } else {
                if let Some(simple_glyph) = self.split_glyphs.get(&glyph.inner().into()) {
                    self.stack.push(simple_glyph.clone());
                }
                self.stack.extend(glyph.inner().components.iter().cloned());
            }
        }
    }

    fn build(&self) -> SerializedFontContext {
        let ser_glyphs =
            self.glyphs
                .iter()
                .map(|glyph| {
                    let inner = glyph.inner();
                    let advance = inner.advance;
                    let kind = if inner.components.is_empty() {
                        SerializedGlyphKind::Simple(inner.outlines.clone())
                    } else {
                        let mut components = vec![];
                        if let Some(simple_glyph) = self.split_glyphs.get(&inner.into()) {
                            components.push(
                                *self
                                    .map
                                    .get(&simple_glyph.inner().into())
                                    .expect("Should be assigned"),
                            );
                        }
                        components.extend(inner.components.iter().map(|c| {
                            *self.map.get(&c.inner().into()).expect("Should be assigned")
                        }));
                        SerializedGlyphKind::Compound(components)
                    };
                    SerializedGlyph { kind, advance }
                })
                .collect();
        SerializedFontContext {
            glyph_list: ser_glyphs,
            cmap: self.cmap.clone(),
        }
    }
}
