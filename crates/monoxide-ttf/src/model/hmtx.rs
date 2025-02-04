use super::{fword, ufword, ITable};

#[derive(Debug)]
pub struct LongHorizontalMetric {
    pub advance_width: ufword,
    pub left_side_bearing: fword,
}

#[derive(Debug)]
pub struct Table {
    /// The horizontal metrics for each glyph in the font, up to
    /// `num_h_metrics` entries.
    pub metrics: Vec<LongHorizontalMetric>,
    /// LSB for glyphs larger than `num_h_metrics`.
    pub left_side_bearings: Vec<fword>,
}

impl ITable for Table {
    fn name(&self) -> &'static [u8; 4] {
        b"hmtx"
    }

    fn write(&self, writer: &mut impl bytes::BufMut) {
        for metric in &self.metrics {
            writer.put_u16(metric.advance_width);
            writer.put_i16(metric.left_side_bearing);
        }

        for &bearing in &self.left_side_bearings {
            writer.put_i16(bearing);
        }
    }
}
