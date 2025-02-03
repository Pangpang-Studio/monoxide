//! Model of the various tables in a TrueType font file.
//!
//! These are rather low-level structures that approximate the binary format of
//! the tables in an OpenType font file. For high-level structures that can be
//! used to generate these tables, see the [`crate::hl`] module.

use bytes::BufMut;
pub mod cmap;
pub mod head;
pub mod hhea;

/// The trait implemented by all tables in a TrueType font file.
pub trait ITable {
    fn name(&self) -> &'static [u8; 4];
    fn write(&self, writer: &mut impl BufMut);
}
