//! CFF2 numeral encodings

/*
Numerical value encoding table
https://learn.microsoft.com/zh-cn/typography/opentype/spec/cff2#encoded-number-formats

Initial byte b0 	Range           Value calculation 	            Size in bytes 	Usage
32 to 246 	        -107 to +107    b0 - 139 	                    1 	            both
247 to 250 	        108 to 1131     (b0 - 247) * 256 + b1 + 108 	2 	            both
251 to 254 	        -1131 to -108   -(b0 - 251) * 256 - b1 - 108 	2 	            both
28 	                (rest int16)    interpret b1 and b2 as int16 	3 	            both
255 	            (rest f16dot16) interpret b1 to b4 as Fixed 	5 	            CharString only
29 	                (rest int32)    interpret b1 to b4 as int32 	5 	            DICT only
30 	                (any)           following bytes are binary coded decimal	unlimited 	DICT only
*/

use bytes::BufMut;
pub use fixed::types::I16F16 as f16dot16;
use rust_decimal::Decimal;

/// Write a number in the range between -1131 and 1131 in CFF2 encoding format.
/// If the number is out of range, return false.
fn write_n1131_to_1131(value: i16, writer: &mut impl bytes::BufMut) -> bool {
    match value {
        -107..=107 => {
            writer.put_u8((value + 139) as u8);
        }
        108..=1131 => {
            writer.put_u8(((value - 108) / 256 + 247) as u8);
            writer.put_u8(((value - 108) % 256) as u8);
        }
        -1131..=-108 => {
            writer.put_u8((-(value + 108) / 256 + 251) as u8);
            writer.put_u8((-(value + 108) % 256) as u8);
        }
        _ => {
            return false;
        }
    }
    true
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Cff2Int16(pub i16);

impl Cff2Int16 {
    pub fn write(&self, writer: &mut impl bytes::BufMut) {
        let value = self.0;
        if !write_n1131_to_1131(value, writer) {
            writer.put_u8(28);
            writer.put_i16(value);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Cff2Int32(pub i32);

impl Cff2Int32 {
    pub fn write(&self, writer: &mut impl bytes::BufMut) {
        let value = self.0;
        if !write_n1131_to_1131(value as i16, writer) {
            writer.put_u8(29);
            writer.put_i32(value);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Cff2Fixed(pub f16dot16);

fn f16dot16_int_part(val: f16dot16) -> i16 {
    (val.int().to_bits() >> 16) as i16
}

impl Cff2Fixed {
    pub fn write(&self, writer: &mut impl bytes::BufMut) {
        let is_int = self.0.frac() == 0;
        if is_int && write_n1131_to_1131(f16dot16_int_part(self.0), writer) {
            return;
        }
        writer.put_u8(255);
        writer.put_i32(self.0.to_bits());
    }
}

/*
Nibble value 	Nibble value (hex) 	Represents in ASCII
0 to 9 	        0 to 9 	            0 to 9
10 	            a 	                . (decimal point)
11 	            b 	                E
12 	            c 	                E-
13 	            d 	                (reserved)
14 	            e 	                - (minus)
15 	            f 	                end of number

The following regular expression (using POSIX ERE or Perl Compatible RE syntax) validates a binary coded decimal value represented as ASCII:

    -?([1-9][0-9]*|0)?(\.[0-9]*)?(E-?[1-9][0-9]*)?
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Cff2Decimal(pub Decimal);

impl Cff2Decimal {
    pub fn write(&self, writer: &mut impl bytes::BufMut) {
        use std::io::Write;

        // All decimals should first write byte 30 to indicate it's a binary
        // encoded decimal, before writing the actual value.
        writer.put_u8(30);

        let mut w = NibbleWriter::new(writer);
        let dec = &self.0;

        if dec.is_zero() {
            w.write_nibble(0);
            w.flush();
        } else {
            let neg = dec.is_sign_negative();
            let mantissa = dec.mantissa().unsigned_abs();
            let scale = dec.scale();

            if neg {
                w.write_nibble(0xe);
            }
            // Write the mantissa and exponent part.
            // Decimal points can't appear on its own, so if the mantissa is
            // only 1 digit, we need to skip the decimal point.
            if mantissa < 10 {
                w.write_nibble(mantissa as u8);
            } else {
                // Serialize the mantissa into bytes, and then write them.
                // A u128 can serialize to at most 39 decimal digits.

                let mut buf = [0u8; 39];
                let buf_len = buf.len();
                let mut write_buf = &mut buf[..];
                write!(&mut write_buf, "{mantissa}").unwrap();
                let written_len = buf_len - write_buf.len();
                for &digit in &buf[..written_len] {
                    w.write_nibble(digit - b'0');
                }
            }
            // Write the scale part.
            // Due to the way Decimal works, the scale is always negative.
            if scale > 0 {
                w.write_nibble(0xc);

                // A u32 scale can serialize to at most 10 decimal digits.
                let mut buf = [0u8; 10];
                let buf_len = buf.len();
                let mut write_buf = &mut buf[..];
                write!(&mut write_buf, "{scale}").unwrap();
                let written_len = buf_len - write_buf.len();
                for &digit in &buf[..written_len] {
                    w.write_nibble(digit - b'0');
                }
            }

            w.flush();
        }
    }
}

/// A writer for writing nibbles to a buffer. The most significant (high) nibble
/// is written first, followed by the least significant (low) nibble.
struct NibbleWriter<T> {
    writer: T,
    /// The current nibble to write.
    curr: u8,
    /// Whether the next nibble to write is the high nibble.
    is_high: bool,
}

impl<T: BufMut> NibbleWriter<T> {
    fn new(writer: T) -> Self {
        Self {
            writer,
            curr: 0,
            is_high: true,
        }
    }

    fn write_nibble(&mut self, nibble: u8) {
        debug_assert!(nibble < 16, "nibble must be in the range 0..=15");
        if self.is_high {
            self.curr = nibble << 4;
        } else {
            self.curr |= nibble;
            self.writer.put_u8(self.curr);
        }
        self.is_high = !self.is_high;
    }

    /// Flush self with one or more `f` nibbles that pad to a whole byte.
    fn flush(&mut self) {
        if self.is_high {
            self.writer.put_u8(0xff);
        } else {
            self.write_nibble(0xf);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cff2Number {
    Int16(Cff2Int16),
    Int32(Cff2Int32),
    Fixed(Cff2Fixed),
    Decimal(Cff2Decimal),
}
