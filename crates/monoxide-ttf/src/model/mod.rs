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
    /// The name tag of the table.
    fn name(&self) -> &'static [u8; 4];

    /// Writes the table to the given buffer.
    ///
    /// This function does not write to a [`std::io::Write`] directly, because
    /// all tables are written to a buffer before being written to a file.
    fn write(&self, writer: &mut impl BufMut);
}

/// A version of `ITable` that can be used with dynamic dispatch, but only
/// writing to a `BytesMut`.
trait DynITable {
    fn name_dyn(&self) -> &'static [u8; 4];
    fn write_dyn(&self, writer: &mut BytesMut);
}

impl<T: ITable> DynITable for T {
    fn name_dyn(&self) -> &'static [u8; 4] {
        T::name(self)
    }

    fn write_dyn(&self, writer: &mut BytesMut) {
        T::write(self, writer)
    }
}

fn ttf_checksum(data: &[u8]) -> u32 {
    let mut sum = 0u32;
    let mut chunks = data.chunks_exact(4);
    for chunk in &mut chunks {
        sum = sum.wrapping_add(u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]));
    }
    let remaining = chunks.remainder();
    if !remaining.is_empty() {
        let mut buf = [0u8; 4];
        buf[..remaining.len()].copy_from_slice(remaining);
        sum = sum.wrapping_add(u32::from_be_bytes(buf));
    }
    sum
}

/// Tables for TrueType outlines.
pub struct TrueTypeTables {
    pub glyf: glyf::Table,
    pub loca: loca::Table,
    pub maxp: maxp::TableV1,
}

/// Tables for CFF2 outlines.
#[allow(deprecated)]
pub struct CFF2Tables {
    pub cff2: cff2::Table,
    pub maxp: maxp::TableV0_5,
}

pub enum Outline {
    TrueType(TrueTypeTables),
    CFF2(CFF2Tables),
}

pub struct FontFile {
    pub head: head::Table,
    pub hhea: hhea::Table,
    pub hmtx: hmtx::Table,
    pub cmap: cmap::Table,
    pub name: name::Table,
    pub os2: os2::Table,
    pub outline: Outline,
}

struct TableRecord {
    tag: [u8; 4],
    checksum: u32,
    offset: u32,
    length: u32,
}

impl FontFile {
    pub fn write(&self, w: impl std::io::Write) -> std::io::Result<()> {
        write_font_file(self, w)
    }
}

fn write_font_file(font: &FontFile, mut w: impl std::io::Write) -> std::io::Result<()> {
    let version = match font.outline {
        Outline::TrueType(_) => [0x00, 0x01, 0x00, 0x00],
        Outline::CFF2(_) => b"OTTO".to_owned(),
    };
    // The `head` requires special treatment because it needs to be
    // rewritten after the checksum is calculated. All other tables can
    // now be treated as opaque blobs.
    let mut tables_except_header = Vec::<&dyn DynITable>::new();
    {
        tables_except_header.push(&font.hhea);
        tables_except_header.push(&font.hmtx);
        tables_except_header.push(&font.cmap);
        tables_except_header.push(&font.name);
        tables_except_header.push(&font.os2);
        match &font.outline {
            Outline::TrueType(tables) => {
                tables_except_header.push(&tables.glyf);
                tables_except_header.push(&tables.loca);
                tables_except_header.push(&tables.maxp);
            }
            Outline::CFF2(_) => {
                todo!("CFF2 tables are not implemented yet");
                // tables_except_header.push(Box::new(tables.cff2));
                // tables_except_header.push(Box::new(tables.maxp));
            }
        }
    }
    let n_table_records = tables_except_header.len() + 1;
    /*
    header layout:
        version: u32
        n_tables: u16
        search_range: u16 = ((2**floor(log2(numTables))) * 16 (Maximum power of 2 less than or equal to numTables * 16)
        entry_selector: u16 = log2(searchRange / 16)
        range_shift: u16 = numTables * 16 - searchRange
        ... table records ...
     */
    let header_size = 12 + n_table_records * 16;

    let search_range = (1 << (n_table_records as f32).log2().floor() as u32) * 16;
    let entry_selector = (n_table_records as f32).log2().floor() as u16;
    let range_shift = (n_table_records as u16 * 16) - search_range as u16;

    // Write the header
    let mut header_buffer = Vec::new();
    header_buffer.extend_from_slice(&version);
    header_buffer.extend_from_slice(&(n_table_records as u16).to_be_bytes());
    header_buffer.extend_from_slice(&(search_range as u16).to_be_bytes());
    header_buffer.extend_from_slice(&entry_selector.to_be_bytes());
    header_buffer.extend_from_slice(&range_shift.to_be_bytes());

    // Calculate the checksum of the whole font file.
    //
    // Since the checksum is calculated using wrapping add, it can be calculated
    // in a different order from the actual writing order. Additionally, we are
    // padding all tables to 4 bytes, so we can directly add the checksum
    // calculated from `ttf_checksum`.
    let mut font_cksum = 0u32;
    font_cksum = font_cksum.wrapping_add(ttf_checksum(&header_buffer));

    // Serialize all tables
    let head_ser = {
        let mut head_buf = BytesMut::new();
        font.head.write(&mut head_buf);
        head_buf.freeze()
    };
    let tables_ser = tables_except_header
        .iter()
        .map(|table| {
            let mut buf = BytesMut::new();
            table.write_dyn(&mut buf);
            buf.freeze()
        })
        .collect::<Vec<_>>();

    // We first do a virtual allocation of all tables to calculate the offsets,
    // before we write the actual data.
    let mut table_records = Vec::with_capacity(n_table_records);
    let mut offset = header_size; // current write offset

    // To assist debugging, we write "__{tag}__" before the beginning of each table
    let debug_data_len = 8;

    // Head table record
    {
        offset += debug_data_len;
        let head_cksum = ttf_checksum(&head_ser);
        table_records.push(TableRecord {
            tag: *font.head.name(),
            checksum: head_cksum,
            offset: offset as u32,
            length: head_ser.len() as u32,
        });

        offset += head_ser.len();
        offset = offset.next_multiple_of(4); // pad to 4 bytes

        font_cksum = font_cksum.wrapping_add(head_cksum);
    }
    for (table, ser) in tables_except_header.iter().zip(tables_ser.iter()) {
        offset += debug_data_len;

        let cksum = ttf_checksum(ser);
        table_records.push(TableRecord {
            tag: *table.name_dyn(),
            checksum: cksum,
            offset: offset as u32,
            length: ser.len() as u32,
        });

        offset += ser.len();
        offset = offset.next_multiple_of(4); // pad to 4 bytes

        font_cksum = font_cksum.wrapping_add(cksum);
    }

    let mut table_records_ser = Vec::new();
    for record in &table_records {
        table_records_ser.extend_from_slice(&record.tag);
        table_records_ser.extend_from_slice(&record.checksum.to_be_bytes());
        table_records_ser.extend_from_slice(&record.offset.to_be_bytes());
        table_records_ser.extend_from_slice(&record.length.to_be_bytes());
    }
    font_cksum = font_cksum.wrapping_add(ttf_checksum(&table_records_ser));

    // As we said, the head table needs to be rewritten with the checksum added.
    let cksum_adjustment = 0xB1B0AFBAu32.wrapping_sub(font_cksum);
    let new_head = head::Table {
        checksum_adjustment: cksum_adjustment,
        ..font.head.clone()
    };
    // And re-serialize it
    drop(head_ser); // We don't need the old head anymore
    let head_ser = {
        let mut head_buf = BytesMut::new();
        new_head.write(&mut head_buf);
        head_buf.freeze()
    };

    fn write_ser(mut w: impl std::io::Write, name: [u8; 4], ser: &[u8]) -> std::io::Result<()> {
        write!(w, "__{}__", std::str::from_utf8(&name).unwrap())?;

        w.write_all(ser)?;
        pad_to_4_bytes(ser.len(), &mut w)?;

        Ok(())
    }

    // Noice, we can finally write the font file
    // Do offset checks for tables and table records
    let mut actual_offset = 0;

    w.write_all(&header_buffer)?;
    actual_offset += header_buffer.len();
    debug_assert_eq!(header_buffer.len(), 12, "header size mismatch");

    w.write_all(&table_records_ser)?;
    actual_offset += table_records_ser.len();
    debug_assert_eq!(
        table_records_ser.len(),
        n_table_records * 16,
        "table records size mismatch"
    );
    debug_assert_eq!(
        actual_offset, header_size,
        "header (including table records) size mismatch"
    );

    pad_to_4_bytes(actual_offset, &mut w)?;

    actual_offset += 8; // debug info

    assert_table_invariants(actual_offset, &table_records[0], "head");
    write_ser(&mut w, *new_head.name(), &head_ser)?;
    actual_offset += head_ser.len().next_multiple_of(4);

    for (ser, tbl) in tables_ser.iter().zip(table_records.iter().skip(1)) {
        actual_offset += 8; // debug info
        let table_tag_string = std::str::from_utf8(&tbl.tag).unwrap();

        assert_table_invariants(actual_offset, tbl, table_tag_string);

        write_ser(&mut w, tbl.tag, ser)?;
        actual_offset += ser.len().next_multiple_of(4);
    }

    Ok(())
}

fn assert_table_invariants(actual_offset: usize, tbl: &TableRecord, table_tag_string: &str) {
    debug_assert_eq!(
        actual_offset % 4,
        0,
        "table {} should be aligned to 4 bytes, offset={}",
        table_tag_string,
        actual_offset
    );
    debug_assert_eq!(
        actual_offset, tbl.offset as usize,
        "offset mismatch for table {}",
        table_tag_string
    );
}

fn pad_to_4_bytes(curr_len: usize, w: &mut impl std::io::Write) -> std::io::Result<()> {
    let pad = [0u8; 4];
    let need_to_pad = (4 - curr_len % 4) % 4;
    w.write_all(&pad[..need_to_pad])?;

    debug_assert_eq!(curr_len + need_to_pad, curr_len.next_multiple_of(4));

    Ok(())
}
