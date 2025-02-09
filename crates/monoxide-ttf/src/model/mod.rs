//! Model of the various tables in a TrueType font file.
//!
//! These are rather low-level structures that approximate the binary format of
//! the tables in an OpenType font file. For high-level structures that can be
//! used to generate these tables, see the [`crate::hl`] module.

use bytes::{BufMut, BytesMut};
pub mod cff2;
pub mod cmap;
pub mod glyf;
pub mod head;
pub mod hhea;
pub mod hmtx;
pub mod loca;
pub mod maxp;
pub mod name;
pub mod os2;

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

/// A version of `ITable` that can be used with dynamic dispatch, but only
/// writing to a `BytesMut`.
pub trait DynITable {
    fn name(&self) -> &'static [u8; 4];
    fn write(&self, writer: &mut BytesMut);
}

impl<T: ITable> DynITable for T {
    fn name(&self) -> &'static [u8; 4] {
        T::name(self)
    }

    fn write(&self, writer: &mut BytesMut) {
        T::write(self, writer)
    }
}

/// Tables for TrueType outlines.
pub struct TrueTypeTables {
    pub glyf: glyf::Table,
    pub loca: loca::Table,
}

/// Tables for CFF2 outlines.
pub struct CFF2Tables {
    pub cff2: cff2::Table,
}

pub struct FontFile {
    // required tables
    pub head: head::Table,
    pub maxp: maxp::Table,
    pub hhea: hhea::Table,
    pub hmtx: hmtx::Table,
    pub cmap: cmap::Table,
    pub name: name::Table,
    pub os2: os2::Table,

    // optional tables
    /// Tables for TrueType outlines.
    pub truetype: Option<TrueTypeTables>,

    /// Tables for CFF2 outlines.
    pub cff2: Option<CFF2Tables>,
}
