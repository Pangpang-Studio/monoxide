use bytes::BufMut;
use thiserror::Error;

use super::{ITable, fword};

#[derive(Debug, Clone, Default)]
pub struct GlyphCommon {
    pub x_min: fword,
    pub y_min: fword,
    pub x_max: fword,
    pub y_max: fword,
}

impl GlyphCommon {
    pub fn write(&self, w: &mut impl BufMut) {
        w.put_i16(self.x_min);
        w.put_i16(self.y_min);
        w.put_i16(self.x_max);
        w.put_i16(self.y_max);
    }
}

pub mod compound;
pub mod simple;

#[derive(Debug)]
pub enum Glyph {
    Simple(simple::SimpleGlyph),
    Compound(compound::CompoundGlyph),
}

#[derive(Debug, Error)]
pub enum GlyphVerifyErrorKind {
    #[error("{0}")]
    Simple(#[from] simple::SimpleGlyphVerifyError),
    #[error("{0}")]
    Compound(#[from] compound::CompoundGlyphVerifyError),
}

#[derive(Debug, Error)]
#[error("Glyph verification failed: {kind}, at glyph index {index}")]
pub struct GlyphVerifyError {
    kind: GlyphVerifyErrorKind,
    index: usize,
}

impl Glyph {
    pub fn common(&self) -> &GlyphCommon {
        match self {
            Glyph::Simple(simple_glyph) => &simple_glyph.common,
            Glyph::Compound(compound_glyph) => &compound_glyph.common,
        }
    }

    pub fn n_points(&self, glyphs: &[Glyph]) -> usize {
        match self {
            Glyph::Simple(g) => g.n_points(),
            Glyph::Compound(g) => g.n_points(glyphs),
        }
    }

    pub fn n_contours(&self, glyphs: &[Glyph]) -> usize {
        match self {
            Glyph::Simple(g) => g.n_contours(),
            Glyph::Compound(g) => g.n_contours(glyphs),
        }
    }

    pub fn depth(&self, glyphs: &[Glyph]) -> usize {
        match self {
            Glyph::Simple(_) => 1,
            Glyph::Compound(g) => g.depth(glyphs),
        }
    }

    pub fn verify(&self) -> Result<(), GlyphVerifyErrorKind> {
        match self {
            Glyph::Simple(g) => g.verify().map_err(|x| x.into()),
            Glyph::Compound(g) => g.verify().map_err(|x| x.into()),
        }
    }

    pub fn write(&self, w: &mut impl BufMut) {
        match self {
            Glyph::Simple(g) => g.write(w),
            Glyph::Compound(g) => g.write(w),
        }
    }
}

pub struct Table {
    pub glyphs: Vec<Glyph>,
}

impl Table {
    pub fn verify(&self) -> Result<(), GlyphVerifyError> {
        for (ix, g) in self.glyphs.iter().enumerate() {
            if let Err(e) = g.verify() {
                return Err(GlyphVerifyError { kind: e, index: ix });
            }
        }
        Ok(())
    }
}

impl ITable for Table {
    fn name(&self) -> &'static [u8; 4] {
        b"glyf"
    }

    fn write(&self, w: &mut impl BufMut) {
        for g in &self.glyphs {
            g.write(w);
        }
    }
}
