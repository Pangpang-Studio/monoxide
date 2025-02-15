//! Generate the `maxp` table from the `glyf` table.

use crate::model::{cff2, glyf, maxp};

/// Convert a `glyf` table to a `maxp` table version 1.
pub fn glyf_to_maxp(glyf: &glyf::Table) -> maxp::TableV1 {
    let mut maxp = maxp::TableV1 {
        n_glyphs: glyf.glyphs.len() as u16,
        max_component_depth: 0,
        ..Default::default()
    };

    for glyph in &glyf.glyphs {
        match glyph {
            glyf::Glyph::Simple(glyph) => {
                maxp.max_points = maxp.max_points.max(glyph.n_points() as u16);
                maxp.max_contours = maxp.max_contours.max(glyph.n_contours() as u16);
                // TODO: zones, twilight points, storage, func defs, instruction defs, stack elements
                maxp.max_size_of_instructions = maxp
                    .max_size_of_instructions
                    .max(glyph.instructions.len() as u16);
            }
            glyf::Glyph::Compound(glyph) => {
                maxp.max_points = maxp.max_points.max(glyph.n_points(&glyf.glyphs) as u16);
                maxp.max_contours = maxp.max_contours.max(glyph.n_contours(&glyf.glyphs) as u16);
                // TODO: instruction stuff, see above
                maxp.max_size_of_instructions = maxp
                    .max_size_of_instructions
                    .max(glyph.instructions.len() as u16);
                maxp.max_component_elements = maxp
                    .max_component_elements
                    .max(glyph.components.len() as u16);
                maxp.max_component_depth = maxp
                    .max_component_depth
                    .max(glyph.depth(&glyf.glyphs) as u16);
            }
        }
    }

    maxp
}

#[allow(deprecated)]
pub fn cff2_to_maxp(_cff2: &cff2::Table) -> maxp::TableV0_5 {
    todo!("CFF2 is not implemented yet")
}
