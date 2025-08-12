// tests/archive_tests.rs

use app::archive::Archive;
use app::block::{Block, BaseBlock};
use app::structures::{
    BlockHeader, EndOfArchiveHeader, FileHeader, GeneralBlockHeader, MainArchiveHeader,
};
use std::io::Cursor;

fn build_block<B: Block>(block: &B) -> Vec<u8> {
    let mut buffer = Vec::new();
    block.encode(&mut buffer).unwrap();
    buffer
}

#[test]
fn test_archive_iteration() {
    let mut mock_data: Vec<u8> = vec![
        0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x01, 0x00, // Signature
    ];

    let main_header = MainArchiveHeader {
        base: BaseBlock {
            general_header: GeneralBlockHeader {
                header_type: 1,
                ..Default::default()
            },
        },
        archive_flags: 0,
        volume_number: None,
    };
    mock_data.extend_from_slice(&build_block(&main_header));

    let file_header = FileHeader {
        base: BaseBlock {
            general_header: GeneralBlockHeader {
                header_type: 2,
                ..Default::default()
            },
        },
        file_flags: 0,
        unpacked_size: 12345,
        file_attributes: 32,
        modification_time: None,
        file_crc32: None,
        compression_info: 48,
        file_name: "test".to_string(),
        symlink_target: None,
    };
    mock_data.extend_from_slice(&build_block(&file_header));

    let end_header = EndOfArchiveHeader {
        base: BaseBlock {
            general_header: GeneralBlockHeader {
                header_type: 5,
                ..Default::default()
            },
        },
        end_archive_flags: 0,
    };
    mock_data.extend_from_slice(&build_block(&end_header));

    let mut archive = Archive::new(Cursor::new(mock_data));

    // First block should be the main header
    let block = archive.next().unwrap().unwrap();
    assert!(matches!(block, BlockHeader::Main(_)));

    // Second block should be the file header
    let block = archive.next().unwrap().unwrap();
    assert!(matches!(block, BlockHeader::File(_)));

    // Third block should be the end of archive
    let block = archive.next().unwrap().unwrap();
    assert!(matches!(block, BlockHeader::End(_)));

    // No more blocks
    assert!(archive.next().is_none());
}

#[test]
fn test_archive_iterator_trait() {
    let mut mock_data: Vec<u8> = vec![
        0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x01, 0x00, // Signature
    ];
    let main_header = MainArchiveHeader {
        base: BaseBlock {
            general_header: GeneralBlockHeader {
                header_type: 1,
                ..Default::default()
            },
        },
        archive_flags: 0,
        volume_number: None,
    };
    mock_data.extend_from_slice(&build_block(&main_header));

    let file_header = FileHeader {
        base: BaseBlock {
            general_header: GeneralBlockHeader {
                header_type: 2,
                ..Default::default()
            },
        },
        file_flags: 0,
        unpacked_size: 12345,
        file_attributes: 32,
        modification_time: None,
        file_crc32: None,
        compression_info: 48,
        file_name: "test".to_string(),
        symlink_target: None,
    };
    mock_data.extend_from_slice(&build_block(&file_header));

    let end_header = EndOfArchiveHeader {
        base: BaseBlock {
            general_header: GeneralBlockHeader {
                header_type: 5,
                ..Default::default()
            },
        },
        end_archive_flags: 0,
    };
    mock_data.extend_from_slice(&build_block(&end_header));

    let archive = Archive::new(Cursor::new(mock_data));
    let blocks: Vec<_> = archive.map(Result::unwrap).collect();

    assert_eq!(blocks.len(), 3);
    assert!(matches!(blocks[0], BlockHeader::Main(_)));
    assert!(matches!(blocks[1], BlockHeader::File(_)));
    assert!(matches!(blocks[2], BlockHeader::End(_)));
}
