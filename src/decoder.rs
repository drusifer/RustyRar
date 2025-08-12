// src/decoder.rs

use crate::structures::{GeneralBlockHeader};
use std::io::{self, Read};

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

        // Extract the 7 data bits
        value |= ((byte & 0x7F) as u64) << shift;

        // Check the continuation flag (MSB)
        if (byte & 0x80) == 0 {
            break;
        }

        shift += 7;
    }

    Ok(value)
}

/// Reads and parses a GeneralBlockHeader from a Read source.
pub fn read_general_block_header<R: Read + ?Sized>(
    reader: &mut R,
) -> Result<GeneralBlockHeader, io::Error> {
    let crc32 = {
        let mut crc_bytes = [0u8; 4];
        reader.read_exact(&mut crc_bytes)?;
        u32::from_le_bytes(crc_bytes)
    };

    let header_size = read_vint(reader)?;
    let header_type = read_vint(reader)?;
    let header_flags = read_vint(reader)?;

    let data_size = if (header_flags & 0x0001) != 0 {
        Some(read_vint(reader)?)
    } else {
        None
    };

    Ok(GeneralBlockHeader {
        crc32,
        header_size,
        header_type,
        header_flags,
        data_size,
    })
}
