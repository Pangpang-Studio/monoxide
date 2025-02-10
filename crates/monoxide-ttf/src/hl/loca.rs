//! Generate the `loca` table from the `glyf` table.

// Although we want to generate the `loca` table directly from the `glyf` table,
// there's actually no better way to do this than just serialize the `glyf`
// table and look at the offsets of each glyph data.
//
// To do this, we use [`crate::util::SizeOnlyBufWriter`] to help us with this.
// This type behaves just like a `BufMut`, but it directly discard all the data
// written to it and only keeps track of the size of the buffer. With sufficient
// compiler optimizations, all unnecessary memory reads and writes should be
// skipped, and we can be left with the bare minimum of information we need
// to get the total size of each glyph data.

use crate::{
    model::{glyf, loca},
    util::SizeOnlyBufWriter,
};

/// Generate the `loca` table from an existing `glyf` table.
pub fn glyf_to_loca(glyf: &glyf::Table) -> loca::Table {
    let mut offsets = Vec::with_capacity(glyf.glyphs.len());
    let mut w = SizeOnlyBufWriter::new();
    for it in &glyf.glyphs {
        it.write(&mut w);
        offsets.push(w.size() as u32);
    }
    loca::Table { offsets }
}
