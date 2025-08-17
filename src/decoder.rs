// src/decoder.rs

use crate::structures::general_block_header::GeneralBlockHeader;
use std::io::{self, Read};
use log::debug;

/// Reads a variable-length integer (vint) from a Read source.
///
/// RAR 5.0 uses a variable-length encoding where each byte's most significant bit
/// indicates if the next byte is part of the integer.
pub fn read_vint<R: Read + ?Sized>(reader: &mut R) -> Result<u64, io::Error> {
    let mut value: u64 = 0;
    let mut shift: u32 = 0;

    for i in 0.. {
        if i >= 10 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Vint value too large",
            ));
        }

        let mut byte_buffer = [0u8; 1];
        reader.read_exact(&mut byte_buffer)?;
        let byte = byte_buffer[0];
        debug!("[read_vint] Read byte: {:#04x}", byte);

        // Extract the 7 data bits
        value |= ((byte & 0x7F) as u64) << shift;

        // Check the continuation flag (MSB)
        if (byte & 0x80) == 0 {
            break;
        }

        shift += 7;
    }

    debug!("[read_vint] Decoded value: {}", value);
    Ok(value)
}

/// Reads and parses a GeneralBlockHeader from a Read source.
pub fn read_general_block_header<R: Read + ?Sized>(
    reader: &mut R,
) -> Result<GeneralBlockHeader, io::Error> {
    debug!("[read_general_block_header] Reading CRC32...");
    let crc32 = {
        let mut crc_bytes = [0u8; 4];
        reader.read_exact(&mut crc_bytes)?;
        u32::from_le_bytes(crc_bytes)
    };
    debug!("[read_general_block_header] CRC32: {:#010x}", crc32);

    debug!("[read_general_block_header] Reading header size...");
    let header_size = read_vint(reader)?;
    debug!("[read_general_block_header] Header size: {}", header_size);

    debug!("[read_general_block_header] Reading header type...");
    let header_type = read_vint(reader)?;
    debug!("[read_general_block_header] Header type: {}", header_type);

    debug!("[read_general_block_header] Reading header flags...");
    let header_flags = read_vint(reader)?;
    debug!("[read_general_block_header] Header flags: {:#06x}", header_flags);

    let data_size = if (header_flags & 0x0001) != 0 {
        debug!("[read_general_block_header] Reading data size...");
        Some(read_vint(reader)?)
    } else {
        None
    };
    if let Some(ds) = data_size {
        debug!("[read_general_block_header] Data size: {}", ds);
    } else {
        debug!("[read_general_block_header] Data size: None");
    }

    Ok(GeneralBlockHeader {
        crc32,
        header_size,
        header_type,
        header_flags,
        data_size,
    })
}
