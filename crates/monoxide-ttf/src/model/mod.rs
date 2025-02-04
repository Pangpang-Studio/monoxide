//! Model of the various tables in a TrueType font file.
//!
//! These are rather low-level structures that approximate the binary format of
//! the tables in an OpenType font file. For high-level structures that can be
//! used to generate these tables, see the [`crate::hl`] module.

use bytes::BufMut;
pub mod cmap;
pub mod head;
pub mod hhea;
pub mod hmtx;

#[allow(non_camel_case_types)]
/// A signed 16-bit number describing number of font design units.
pub type fword = i16;

#[allow(non_camel_case_types)]
/// An unsigned 16-bit number describing number of font design units.
pub type ufword = u16;

#[allow(non_camel_case_types)]
/// A signed fix-point number of 14 fractional bits.
pub type f2dot14 = fixed::types::I2F14;

/// The trait implemented by all tables in a TrueType font file.
pub trait ITable {
    fn name(&self) -> &'static [u8; 4];
    fn write(&self, writer: &mut impl BufMut);
}
