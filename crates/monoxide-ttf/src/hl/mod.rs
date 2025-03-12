//! High-level data structures to generate various tables within a TrueType font file.
//!
//! These data structures will be mapped to the low-level structures in
//! [`crate::model`] so that they can be written to the binary format.
pub mod cmap;
pub mod glyf;
pub mod loca;
pub mod maxp;
