use super::ITable;

pub struct Table {
    pub ascender: i16,
    pub descender: i16,
    pub line_gap: i16,
    pub advance_width_max: u16,
    pub min_left_side_bearing: i16,
    pub min_right_side_bearing: i16,
    pub x_max_extent: i16,
    pub caret_slope_rise: i16,
    pub caret_slope_run: i16,
    pub caret_offset: i16,
    pub metric_data_format: i16,
    pub number_of_hmetrics: u16,
}

const MAJOR_VERSION: u16 = 1;
const MINOR_VERSION: u16 = 0;

impl ITable for Table {
    fn name(&self) -> &'static [u8; 4] {
        b"hhea"
    }

    fn write(&self, writer: &mut impl bytes::BufMut) {
        writer.put_u16(MAJOR_VERSION);
        writer.put_u16(MINOR_VERSION);
        writer.put_i16(self.ascender);
        writer.put_i16(self.descender);
        writer.put_i16(self.line_gap);
        writer.put_u16(self.advance_width_max);
        writer.put_i16(self.min_left_side_bearing);
        writer.put_i16(self.min_right_side_bearing);
        writer.put_i16(self.x_max_extent);
        writer.put_i16(self.caret_slope_rise);
        writer.put_i16(self.caret_slope_run);
        writer.put_i16(self.caret_offset);

        // Reserved
        for _ in 0..4 {
            writer.put_i16(0);
        }

        writer.put_i16(self.metric_data_format);
        writer.put_u16(self.number_of_hmetrics);
    }
}
