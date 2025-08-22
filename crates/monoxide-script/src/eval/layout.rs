use std::collections::{BTreeMap, HashMap, HashSet};

use by_address::ByAddress;
use monoxide_curves::xform::Affine2D;
use petgraph::prelude::DiGraphMap;

use crate::{
    ast::{FontContext, Glyph, GlyphInner},
    eval::{
        HighEvalError, SerializedComponent, SerializedFontContext, SerializedGlyph,
        SerializedGlyphKind,
    },
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
fn split(cx: &FontContext) -> HashMap<ByAddress<Glyph>, Glyph> {
    let mut vis = HashSet::<ByAddress<Glyph>>::new();
    let mut res = HashMap::<ByAddress<Glyph>, Glyph>::new();
    let mut stack = Vec::<&Glyph>::new();

    stack.push(cx.tofu.as_ref().expect("Should be checked"));
    for glyph in cx.cmap.values() {
        stack.push(glyph);
    }

    while let Some(glyph) = stack.pop() {
        if vis.contains(ByAddress::from_ref(glyph)) {
            continue;
        }
        vis.insert(glyph.clone().into());

        if !glyph.outlines.is_empty() && !glyph.components.is_empty() {
            // This glyph needs to split into simple and compound parts.
            let new_glyph = Glyph::from_inner(GlyphInner {
                outlines: glyph.outlines.clone(),
                components: vec![],
                advance: glyph.advance,
            });
            res.insert(glyph.clone().into(), new_glyph);
        }
    }

    res
}

struct GlyphSerializer<'a> {
    cx: &'a FontContext,
    split_glyphs: HashMap<ByAddress<Glyph>, Glyph>,

    /// Map from glyph instances to their assigned IDs in the glyph list.
    map: HashMap<ByAddress<Glyph>, usize>,
    /// Built `cmap` table
    cmap: BTreeMap<char, usize>,

    /// List of glyphs not yet transformed into [`SerializedGlyph`]
    glyphs: Vec<Glyph>,
    /// DFS stack
    stack: Vec<Glyph>,
}

impl<'a> GlyphSerializer<'a> {
    fn new(cx: &'a FontContext, split_glyphs: HashMap<ByAddress<Glyph>, Glyph>) -> Self {
        Self {
            cx,
            split_glyphs,
            map: HashMap::new(),
            glyphs: Vec::new(),
            cmap: BTreeMap::new(),
            stack: Vec::new(),
        }
    }

    fn assign_id(&mut self, glyph: &Glyph) -> usize {
        let next_id = self.glyphs.len();
        if let Some(id) = self.map.get(ByAddress::from_ref(glyph)) {
            return *id;
        }
        self.map.insert(glyph.clone().into(), next_id);
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
            self.assign_id(&glyph);
            if glyph.inner().components.is_empty() {
                // Simple glyph
            } else {
                if let Some(simple_glyph) = self.split_glyphs.get(ByAddress::from_ref(&glyph)) {
                    self.stack.push(simple_glyph.clone());
                }
                for cmp in &glyph.inner().components {
                    self.stack.push(cmp.component.clone());
                }
            }
        }
    }

    fn build(&self) -> SerializedFontContext {
        let mut glyph_list = Vec::new();
        let mut glyph_map = DiGraphMap::new();
        for (idx, glyph) in self.glyphs.iter().enumerate() {
            let serialized = self.convert_to_serialized_glyph(glyph);

            glyph_map.add_node(idx);
            match &serialized.kind {
                SerializedGlyphKind::Simple(_) => {}
                SerializedGlyphKind::Compound(components) => {
                    for it in components {
                        glyph_map.add_edge(idx, it.index, ());
                    }
                }
            }
            glyph_list.push(serialized);
        }

        SerializedFontContext {
            glyph_list,
            cmap: self.cmap.clone(),
            glyph_map,
        }
    }

    fn convert_to_serialized_glyph(&self, glyph: &Glyph) -> SerializedGlyph {
        let inner = glyph.inner();
        let advance = inner.advance;
        let kind = if inner.components.is_empty() {
            SerializedGlyphKind::Simple(inner.outlines.clone())
        } else {
            let mut components = vec![];

            if let Some(simple_glyph) = self.split_glyphs.get(ByAddress::from_ref(glyph)) {
                let index = *self
                    .map
                    .get(ByAddress::from_ref(simple_glyph))
                    .expect("Should be assigned");
                let xform = Affine2D::id();
                components.push(SerializedComponent { index, xform });
            }

            components.extend(inner.components.iter().map(|c| {
                let index = *self
                    .map
                    .get(ByAddress::from_ref(&c.component))
                    .expect("Should be assigned");
                let xform = c.xform;
                SerializedComponent { index, xform }
            }));
            SerializedGlyphKind::Compound(components)
        };
        SerializedGlyph { kind, advance }
    }
}
