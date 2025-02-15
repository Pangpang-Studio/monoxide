//! The `DSIG` table.

use bitflags::bitflags;

use super::ITable;

pub const VERSION: u32 = 0x00000001;

#[derive(Clone, Copy, Debug, Default)]
pub struct Flags(u16);
bitflags! {
    impl Flags: u16 {
        const NO_RESIGN = 0b0000_0001;
    }
}

#[derive(Debug, Default)]
pub struct Table {
    // version: u32 = 0x00000001,
    // length: u32,
    pub flags: Flags,
    // TODO: add actual signature data
}

impl ITable for Table {
    fn name(&self) -> &'static [u8; 4] {
        b"DSIG"
    }

    fn write(&self, writer: &mut impl bytes::BufMut) {
        writer.put_u32(VERSION);
        writer.put_u16(0); // length
        writer.put_u16(self.flags.bits());
        // TODO: add actual signature data
    }
}
