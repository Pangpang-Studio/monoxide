use bytes::BufMut;

use super::fword;

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

mod compound;
mod simple;

pub enum Glyph {
    Simple(simple::SimpleGlyph),
    Compound(compound::CompoundGlyph),
}

pub struct Table {
    pub glyphs: Vec<Glyph>,
}
