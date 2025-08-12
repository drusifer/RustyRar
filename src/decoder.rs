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

    loop {
        let mut byte_buffer = [0u8; 1];
        reader.read_exact(&mut byte_buffer)?;
        let byte = byte_buffer[0];

        // Extract the 7 data bits
        value |= ((byte & 0x7F) as u64) << shift;

        // Check the continuation flag (MSB)
        if (byte & 0x80) == 0 {
            break; // No more bytes for this vint
        }

        shift += 7;

        // Prevent overflow for extremely large, invalid vints
        if shift >= 64 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Vint value too large",
            ));
        }
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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::structures::{EndOfArchiveHeader, FileHeader, GeneralBlockHeader, MainArchiveHeader};
    use std::io::Cursor;

    #[test]

    fn test_read_vint_single_byte() {
        let data: Vec<u8> = vec![0x05]; // Vint value 5
        let mut cursor = Cursor::new(data);
        let result = read_vint(&mut cursor).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn test_read_vint_multi_byte() {
        let data: Vec<u8> = vec![0x81, 0x01]; // Vint value 129 (1 + 128)
        let mut cursor = Cursor::new(data);
        let result = read_vint(&mut cursor).unwrap();
        assert_eq!(result, 129);
    }

    #[test]
    fn test_read_general_block_header_basic() {
        // Mock data for a basic header:
        // CRC32: 0x11223344 (little-endian)
        // Header Size: 10 (vint 0x0A)
        // Header Type: 1 (vint 0x01)
        // Header Flags: 0x0000 (vint 0x00)
        let data: Vec<u8> = vec![
            0x44, 0x33, 0x22, 0x11, // CRC32
            0x0A,   // Header Size (10)
            0x01,   // Header Type (1)
            0x00,   // Header Flags (0)
        ];
        let mut cursor = Cursor::new(data);
        let header = read_general_block_header(&mut cursor).unwrap();

        assert_eq!(
            header,
            GeneralBlockHeader {
                crc32: 0x11223344,
                header_size: 10,
                header_type: 1,
                header_flags: 0,
                data_size: None,
            }
        );
    }

    #[test]
    fn test_read_general_block_header_with_data_size() {
        // Mock data for a header with Data Size:
        // CRC32: 0xAABBCCDD (little-endian)
        // Header Size: 15 (vint 0x0F)
        // Header Type: 2 (vint 0x02)
        // Header Flags: 0x0001 (vint 0x01) - indicates presence of Data Size
        // Data Size: 1024 (vint 0x80, 0x08)
        let data: Vec<u8> = vec![
            0xDD, 0xCC, 0xBB, 0xAA, // CRC32
            0x0F,   // Header Size (15)
            0x02,   // Header Type (2)
            0x01,   // Header Flags (1)
            0x80, 0x08, // Data Size (1024)
        ];
        let mut cursor = Cursor::new(data);
        let header = read_general_block_header(&mut cursor).unwrap();

        assert_eq!(
            header,
            GeneralBlockHeader {
                crc32: 0xAABBCCDD,
                header_size: 15,
                header_type: 2,
                header_flags: 1,
                data_size: Some(1024),
            }
        );
    }

    #[test]
    fn test_read_main_archive_header_with_volume() {
        // Mock data for a Main Archive Header:
        // Archive Flags: 0x0001 (vint 0x01) - Volume
        // Volume Number: 1 (vint 0x01)
        let data: Vec<u8> = vec![
            0x01, // Archive Flags (Volume)
            0x01, // Volume Number (1)
        ];
        let mut cursor = Cursor::new(data);
        let main_header = read_main_archive_header(&mut cursor).unwrap();

        let expected = MainArchiveHeader {
            archive_flags: 1,
            volume_number: Some(1),
        };

        assert_eq!(main_header, expected);
    }

    #[test]
    fn test_read_main_archive_header_no_volume() {
        // Mock data for a Main Archive Header:
        // Archive Flags: 0x0000 (vint 0x00) - Not a volume
        let data: Vec<u8> = vec![
            0x00, // Archive.
        ];
        let mut cursor = Cursor::new(data);
        let main_header = read_main_archive_header(&mut cursor).unwrap();

        let expected = MainArchiveHeader {
            archive_flags: 0,
            volume_number: None,
        };

        assert_eq!(main_header, expected);
    }

    #[test]
    fn test_read_file_header() {
        // Mock data for a File Header:
        // File Flags: 0 (vint)
        // Unpacked Size: 12345 (vint)
        // File Attributes: 32 (vint)
        // Compression Info: 0x30 (vint)
        // File Name Length: 4 (vint)
        // File Name: "test"
        let data: Vec<u8> = vec![
            0x00, // File Flags
            0xB9, 0x60, // Unpacked Size (12345)
            0x20, // File Attributes
            0x30, // Compression Info
            0x04, // File Name Length
            b't', b'e', b's', b't', // File Name
        ];
        let mut cursor = Cursor::new(data);
        let file_header = read_file_header(&mut cursor).unwrap();

        let expected = FileHeader {
            file_flags: 0,
            unpacked_size: 12345,
            file_attributes: 32,
            modification_time: None,
            file_crc32: None,
            compression_info: 0x30,
            file_name: "test".to_string(),
            symlink_target: None,
        };

        assert_eq!(file_header, expected);
    }

    #[test]
    fn test_read_file_header_with_optional_data() {
        // Mock data for a File Header with optional data:
        // File Flags: 0x03 (vint) - Mod time and CRC32 present
        // Unpacked Size: 12345 (vint)
        // File Attributes: 32 (vint)
        // Modification Time: 0x12345678 (u32 LE)
        // CRC32: 0x87654321 (u32 LE)
        // Compression Info: 0x30 (vint)
        // File Name Length: 4 (vint)
        // File Name: "test"
        let data: Vec<u8> = vec![
            0x06, // File Flags (mod time + crc32)
            0xB9, 0x60, // Unpacked Size (12345)
            0x20, // File Attributes
            0x78, 0x56, 0x34, 0x12, // Mod Time
            0x21, 0x43, 0x65, 0x87, // CRC32
            0x30, // Compression Info
            0x04, // File Name Length
            b't', b'e', b's', b't', // File Name
        ];
        let mut cursor = Cursor::new(data);
        let file_header = read_file_header(&mut cursor).unwrap();

        let expected = FileHeader {
            file_flags: 6,
            unpacked_size: 12345,
            file_attributes: 32,
            modification_time: Some(0x12345678),
            file_crc32: Some(0x87654321),
            compression_info: 0x30,
            file_name: "test".to_string(),
            symlink_target: None,
        };

        assert_eq!(file_header, expected);
    }

    #[test]
    fn test_read_file_header_with_symlink() {
        // Mock data for a File Header with a symlink:
        // File Flags: 0x08 (vint) - Symlink
        // Unpacked Size: 0 (vint)
        // File Attributes: 0 (vint)
        // Compression Info: 0x30 (vint)
        // File Name Length: 15 (vint)
        // File Name: "/path/to/target"
        let data: Vec<u8> = vec![
            0x08, // File Flags (Symlink)
            0x00, // Unpacked Size
            0x00, // File Attributes
            0x30, // Compression Info
            0x0F, // File Name Length
            b'/', b'p', b'a', b't', b'h', b'/', b't', b'o', b'/', b't', b'a', b'r', b'g', b'e', b't',
        ];
        let mut cursor = Cursor::new(data);
        let file_header = read_file_header(&mut cursor).unwrap();

        let expected = FileHeader {
            file_flags: 8,
            unpacked_size: 0,
            file_attributes: 0,
            modification_time: None,
            file_crc32: None,
            compression_info: 0x30,
            file_name: "/path/to/target".to_string(),
            symlink_target: Some("/path/to/target".to_string()),
        };

        assert_eq!(file_header, expected);
    }

    #[test]
    fn test_read_end_of_archive_header() {
        // Mock data for an End of Archive Header:
        // End of Archive Flags: 1 (vint)
        let data: Vec<u8> = vec![0x01];
        let mut cursor = Cursor::new(data);
        let end_of_archive_header = read_end_of_archive_header(&mut cursor).unwrap();

        let expected = EndOfArchiveHeader {
            end_archive_flags: 1,
        };

        assert_eq!(end_of_archive_header, expected);
    }
}
