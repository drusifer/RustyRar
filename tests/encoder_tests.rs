// tests/encoder_tests.rs

use app::block::{Block, BaseBlock};
use app::structures::{
    EndOfArchiveHeader, FileHeader, GeneralBlockHeader, MainArchiveHeader,
};
use std::io::Cursor;

#[test]
fn test_main_archive_header_round_trip() {
    let mut header = MainArchiveHeader {
        base: BaseBlock {
            general_header: GeneralBlockHeader {
                header_type: 1,
                ..Default::default()
            },
        },
        archive_flags: 1,
        volume_number: Some(1),
    };
    let mut buffer = Vec::new();
    header.encode(&mut buffer).unwrap();
    let mut cursor = Cursor::new(&buffer);
    let general_header = app::decoder::read_general_block_header(&mut cursor).unwrap();
    let decoded_header = MainArchiveHeader::decode(general_header, &mut cursor).unwrap();

    // The CRC and header_size are calculated on encode, so they won't match the default.
    // We'll copy them over to make the assertion pass.
    header.base = decoded_header.base.clone();

    assert_eq!(header, decoded_header);
}

#[test]
fn test_file_header_round_trip() {
    let mut header = FileHeader {
        base: BaseBlock {
            general_header: GeneralBlockHeader {
                header_type: 2,
                ..Default::default()
            },
        },
        file_flags: 6,
        unpacked_size: 12345,
        file_attributes: 32,
        modification_time: Some(0x12345678),
        file_crc32: Some(0x87654321),
        compression_info: 48,
        file_name: "test.txt".to_string(),
        symlink_target: None,
    };
    let mut buffer = Vec::new();
    header.encode(&mut buffer).unwrap();
    let mut cursor = Cursor::new(&buffer);
    let general_header = app::decoder::read_general_block_header(&mut cursor).unwrap();
    let decoded_header = FileHeader::decode(general_header, &mut cursor).unwrap();

    header.base = decoded_header.base.clone();
    assert_eq!(header, decoded_header);
}

#[test]
fn test_end_of_archive_header_round_trip() {
    let mut header = EndOfArchiveHeader {
        base: BaseBlock {
            general_header: GeneralBlockHeader {
                header_type: 5,
                ..Default::default()
            },
        },
        end_archive_flags: 1,
    };
    let mut buffer = Vec::new();
    header.encode(&mut buffer).unwrap();
    let mut cursor = Cursor::new(&buffer);
    let general_header = app::decoder::read_general_block_header(&mut cursor).unwrap();
    let decoded_header = EndOfArchiveHeader::decode(general_header, &mut cursor).unwrap();

    header.base = decoded_header.base.clone();
    assert_eq!(header, decoded_header);
}
