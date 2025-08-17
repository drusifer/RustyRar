// src/encoder.rs

use crate::structures::base::BlockHeader;
use crate::structures::block::Block;
use std::io::{self, Write};
use log::debug;

pub fn write_vint(writer: &mut dyn Write, mut value: u64) -> io::Result<()> {
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value > 0 {
            byte |= 0x80;
        }
        writer.write_all(&[byte])?;
        if value == 0 {
            break;
        }
    }
    Ok(())
}

pub fn encode_block(block: &BlockHeader, writer: &mut dyn Write) -> io::Result<()> {
    debug!("[encode_block] Encoding block: {:?}", block);
    match block {
        BlockHeader::Main(header) => header.encode(writer),
        BlockHeader::File(header) => header.encode(writer),
        BlockHeader::End(header) => header.encode(writer),
    }
}
