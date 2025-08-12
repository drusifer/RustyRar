// src/decoder.rs

use crate::structures::{EndOfArchiveHeader, FileHeader, GeneralBlockHeader, MainArchiveHeader};
use std::io::{self, Read};

// A constant for the "Volume" flag in the main archive header.
const ARCHIVE_FLAG_VOLUME: u64 = 0x0001;

// File header flags
const FILE_FLAG_MODIFICATION_TIME: u64 = 0x02;
const FILE_FLAG_CRC32: u64 = 0x04;
const FILE_FLAG_SYMLINK: u64 = 0x08;

/// Reads a variable-length integer (vint) from a Read source.
///
/// RAR 5.0 uses a variable-length encoding where each byte's most significant bit
/// indicates if the next byte is part of the integer.
pub fn read_vint<R: Read>(reader: &mut R) -> Result<u64, io::Error> {
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
pub fn read_general_block_header<R: Read>(
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

/// Reads a Main Archive Header from a `Read` source.
pub fn read_main_archive_header<R: Read>(
    reader: &mut R,
) -> Result<MainArchiveHeader, io::Error> {
    let archive_flags = read_vint(reader)?;
    let volume_number = if (archive_flags & ARCHIVE_FLAG_VOLUME) != 0 {
        Some(read_vint(reader)?)
    } else {
        None
    };

    Ok(MainArchiveHeader {
        archive_flags,
        volume_number,
    })
}

/// Reads a File Header from a `Read` source.
pub fn read_file_header<R: Read>(reader: &mut R) -> Result<FileHeader, io::Error> {
    let file_flags = read_vint(reader)?;
    let unpacked_size = read_vint(reader)?;
    let file_attributes = read_vint(reader)?;

    let modification_time = if (file_flags & FILE_FLAG_MODIFICATION_TIME) != 0 {
        let mut time_bytes = [0u8; 4];
        reader.read_exact(&mut time_bytes)?;
        Some(u32::from_le_bytes(time_bytes))
    } else {
        None
    };

    let file_crc32 = if (file_flags & FILE_FLAG_CRC32) != 0 {
        let mut crc_bytes = [0u8; 4];
        reader.read_exact(&mut crc_bytes)?;
        Some(u32::from_le_bytes(crc_bytes))
    } else {
        None
    };

    let compression_info = read_vint(reader)?;
    let file_name_length = read_vint(reader)? as usize;

    let mut file_name_bytes = vec![0; file_name_length];
    reader.read_exact(&mut file_name_bytes)?;
    let file_name = String::from_utf8(file_name_bytes.clone())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 in file name"))?;

    let symlink_target = if (file_flags & FILE_FLAG_SYMLINK) != 0 {
        Some(
            String::from_utf8(file_name_bytes)
                .map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 in symlink target")
                })?,
        )
    } else {
        None
    };

    Ok(FileHeader {
        file_flags,
        unpacked_size,
        file_attributes,
        modification_time,
        file_crc32,
        compression_info,
        file_name,
        symlink_target,
    })
}

/// Reads an End Of Archive Header from a `Read` source.
pub fn read_end_of_archive_header<R: Read>(
    reader: &mut R,
) -> Result<EndOfArchiveHeader, io::Error> {
    let end_archive_flags = read_vint(reader)?;
    Ok(EndOfArchiveHeader {
        end_archive_flags,
    })
}
