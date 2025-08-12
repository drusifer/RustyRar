// src/encoder.rs

use crate::structures::general_block_header::GeneralBlockHeader;
use crc::{Crc, CRC_32_ISO_HDLC};
use std::io::{self, Write};

pub fn write_vint<W: Write + ?Sized>(writer: &mut W, mut value: u64) -> io::Result<()> {
    if value == 0 {
        return writer.write_all(&[0]);
    }

    let mut vint = Vec::new();
    while value > 0 {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value > 0 {
            byte |= 0x80;
        }
        vint.push(byte);
    }
    writer.write_all(&vint)
}

pub fn write_general_block_header<W: Write + ?Sized>(
    writer: &mut W,
    header: &GeneralBlockHeader,
) -> io::Result<()> {
    let mut header_data = Vec::new();
    write_vint(&mut header_data, header.header_size)?;
    write_vint(&mut header_data, header.header_type)?;
    write_vint(&mut header_data, header.header_flags)?;
    if let Some(data_size) = header.data_size {
        write_vint(&mut header_data, data_size)?;
    }

    let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    let mut digest = crc.digest();
    digest.update(&header_data);
    let crc_bytes = digest.finalize().to_le_bytes();

    writer.write_all(&crc_bytes)?;
    writer.write_all(&header_data)
}
