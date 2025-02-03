//! Model of the various tables in a TrueType font file.
//!
//! These are rather low-level structures that approximate the binary format of
//! the tables in an OpenType font file. For high-level structures that can be
//! used to generate these tables, see the [`crate::hl`] module.

use bytes::BufMut;
pub mod cmap;

/// The trait implemented by all tables in a TrueType font file.
pub trait ITable {
    fn name_raw(&self) -> &'static str;
    fn write(&self, writer: &mut impl BufMut);

    fn name(&self) -> [u8; 4] {
        assert!(self.name_raw().len() == 4);
        let mut name = [0u8; 4];
        name.copy_from_slice(self.name_raw().as_bytes());
        name
    }
}
