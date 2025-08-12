// tests/decoder_tests.rs

// The tests need to be in a separate file and not a module.
// We also need to import the functions and structs from the `app` crate.

use app::decoder::*;
use app::structures::general_block_header::GeneralBlockHeader;
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
fn test_read_vint_zero() {
    let data: Vec<u8> = vec![0x00];
    let mut cursor = Cursor::new(data);
    let result = read_vint(&mut cursor).unwrap();
    assert_eq!(result, 0);
}

#[test]
fn test_read_vint_too_long() {
    // A vint should not be longer than 10 bytes.
    let data: Vec<u8> = vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01];
    let mut cursor = Cursor::new(data);
    let result = read_vint(&mut cursor);
    assert!(result.is_err());
}

#[test]
fn test_general_block_header_basic() {
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
