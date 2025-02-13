use bytes::BufMut;

use crate::{hl::cmap as hl, model::encoding::NON_MACINTOSH_LANG_CODE};

#[derive(Debug, PartialEq, Eq)]
struct Segment {
    /// The ending character code for the segment, inclusive.
    end_code: u16,
    /// The starting character code for the segment.
    start_code: u16,
    /// The delta value to be added to the character code to get the glyph index.
    id_delta: i16,
    /// The offset into the glyph index array or 0 if the idDelta value is used.
    ///
    /// All segments emitted by this program will have this value set to 0
    /// because we don't use the glyph index array.
    id_range_offset: u16,
}

#[derive(Debug)]
pub struct Table {
    // format: u16 = 4,
    length: u16,
    language: u16,
    seg_count_x2: u16, // = seg_count * 2
    search_range: u16,
    entry_selector: u16,
    range_shift: u16,
    // end_code: Vec<u16>,          // of length seg_count
    // reserved_pad: u16 = 0
    // start_code: Vec<u16>,        // of length seg_count
    // id_delta: Vec<u16>,          // of length seg_count
    // id_range_offset: Vec<u16>,   // of length seg_count
    segments: Vec<Segment>,
    // The glyph index array is not used by us
    // glyph_index_array: Vec<u16>, // variable length
}

impl Table {
    /// Get the written size in byte of the table.
    pub fn byte_length(&self) -> usize {
        // As the length is pre-calculated, we can just return it.
        self.length as usize
    }

    /// Write the table to a buffer.
    pub fn write(&self, writer: &mut impl BufMut) {
        writer.put_u16(4);

        writer.put_u16(self.length);
        writer.put_u16(self.language);
        writer.put_u16(self.seg_count_x2);
        writer.put_u16(self.search_range);
        writer.put_u16(self.entry_selector);
        writer.put_u16(self.range_shift);
        for segment in &self.segments {
            writer.put_u16(segment.end_code);
        }
        writer.put_u16(0);
        for segment in &self.segments {
            writer.put_u16(segment.start_code);
        }
        for segment in &self.segments {
            writer.put_i16(segment.id_delta);
        }
        for segment in &self.segments {
            writer.put_u16(segment.id_range_offset);
        }
        // for glyph_index in &self.glyph_index_array {
        //     writer.put_u16(*glyph_index);
        // }
    }

    /// Convert a raw subtable to a format 4 subtable.
    pub fn from_raw(tbl: &hl::SubTable) -> Self {
        // As the input range list does not contain holes, the number of
        // segments is the same as the number of ranges.
        let seg_count = tbl.len() + 1; // +1 for the dummy segment, see below
        let seg_count_u16 = seg_count as u16;

        // Search parameters
        // segCountX2 is just 2 * segCount
        let seg_count_x2 = seg_count_u16 * 2;
        // searchRange is the next power of 2 larger than segCount, times 2
        let search_range = seg_count_u16.next_power_of_two() * 2;
        // entrySelector is log2 of the maximum power of 2 that leq segCount
        // that is to say, the number of bits needed to represent segCount
        let entry_selector = seg_count_u16.next_power_of_two().trailing_zeros() as u16;
        // rangeShift is segCount * 2 - searchRange
        let range_shift = seg_count_x2 - search_range;

        // Encode the segments
        let mut segments = vec![];
        for seq in tbl {
            // We can't encode any segment that is outside BMP.
            if seq.start_code > 0xffff {
                continue;
            }
            // We can encode the in-BMP part of a segment that starts within
            // the BMP and extends beyond it. This should be rare, if not
            // completely non-existent (because the end of BMP is full of
            // non-characters).
            //
            // Anyway, the following code will truncate the segment to the BMP
            // if by any chance it extends beyond it.
            let seq = if (seq.start_code + seq.len - 1) > 0xffff {
                &hl::SeqMapping {
                    start_code: seq.start_code,
                    len: 0x10000 - seq.start_code,
                    glyph_id: seq.glyph_id,
                }
            } else {
                seq
            };

            segments.push(Segment {
                end_code: (seq.start_code + seq.len - 1) as u16,
                start_code: (seq.start_code) as u16,
                id_delta: (seq.glyph_id as i16).wrapping_sub(seq.start_code as i16),
                id_range_offset: 0, // for we don't use the glyph index array
            });
        }
        // A final segment is needed to end the search.
        //
        // > For the search to terminate, the final startCode and endCode values
        // > must be 0xFFFF. This segment need not contain any valid mappings.
        // > (It can just map the single character code 0xFFFF to missingGlyph).
        // > However, the segment must be present.
        //
        // https://learn.microsoft.com/zh-cn/typography/opentype/spec/cmap#format-4-segment-mapping-to-delta-values
        segments.push(Segment {
            end_code: 0xFFFF,
            start_code: 0xFFFF,
            id_delta: 1,
            id_range_offset: 0,
        });

        let length = 8 * 2 // header & padding
            + (4 * 2) * seg_count_u16; // segments

        Self {
            length,
            language: NON_MACINTOSH_LANG_CODE,
            seg_count_x2,
            search_range,
            entry_selector,
            range_shift,
            segments,
        }
    }
}

/// This is a test taken from the OpenType specification that demonstrates the
/// format 4 subtable layout and encoding.
///
/// You can find it at the end of
/// https://learn.microsoft.com/zh-cn/typography/opentype/spec/cmap#format-4-segment-mapping-to-delta-values
#[test]
fn test_opentype_spec_example() {
    /*
        As an example, the variant part of the table to map characters 10-20, 30-90,
        and 153-480 onto a contiguous range of glyph indices may look like this:

        segCountX2: 	8
        searchRange: 	8
        entrySelector: 	2
        rangeShift: 	0
        endCode: 	    20 90 480 0xffff
        reservedPad: 	0
        startCode: 	    10 30 153 0xffff
        idDelta: 	    -9 -18 -80 1
        idRangeOffset: 	0 0 0 0

        This table yields the following mappings:

        10 ⇒ 10 - 9 = 1
        20 ⇒ 20 - 9 = 11
        30 ⇒ 30 - 18 = 12
        90 ⇒ 90 - 18 = 72
        153 ⇒ 153 - 80 = 73
        480 ⇒ 480 - 80 = 400
        0xffff ⇒ 0

        Note that the delta values could be reworked so as to reorder the segments.
    */
    let raw_tbl = vec![
        hl::SeqMapping {
            start_code: 10,
            len: 11,
            glyph_id: 1,
        },
        hl::SeqMapping {
            start_code: 30,
            len: 61,
            glyph_id: 12,
        },
        hl::SeqMapping {
            start_code: 153,
            len: 328,
            glyph_id: 73,
        },
    ];

    let table = Table::from_raw(&raw_tbl);
    assert_eq!(table.seg_count_x2, 8);
    assert_eq!(table.search_range, 8);
    assert_eq!(table.entry_selector, 2);
    assert_eq!(table.range_shift, 0);
    assert_eq!(table.segments.len(), 4);
    assert_eq!(
        &table.segments[0],
        &Segment {
            end_code: 20,
            start_code: 10,
            id_delta: -9,
            id_range_offset: 0,
        }
    );
    assert_eq!(
        &table.segments[1],
        &Segment {
            end_code: 90,
            start_code: 30,
            id_delta: -18,
            id_range_offset: 0,
        }
    );
    assert_eq!(
        &table.segments[2],
        &Segment {
            end_code: 480,
            start_code: 153,
            id_delta: -80,
            id_range_offset: 0,
        }
    );
    assert_eq!(
        &table.segments[3],
        &Segment {
            end_code: 0xFFFF,
            start_code: 0xFFFF,
            id_delta: 1,
            id_range_offset: 0,
        }
    );
}
