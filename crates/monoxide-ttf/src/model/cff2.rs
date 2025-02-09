//! Implementation of the `CFF2` table.

pub mod inst;
pub mod num;

pub struct IndexData {
    // count: u32,
    pub offsize: u8,       // the bytes needed to store the offsets
    pub offsets: Vec<u32>, // of size count + 1
    pub data: Vec<u8>,
}

impl IndexData {
    pub fn verify(&self) -> Result<(), &'static str> {
        if self.offsets.len() != self.data.len() {
            return Err("offsets and data length mismatch");
        }
        Ok(())
    }
}

#[deprecated = "Not finished, do not use"]
pub struct Table {}
