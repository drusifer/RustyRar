// tests/archive_tests.rs

use app::archive::Archive;
use app::structures::{
    block::{Block, BaseBlock},
    base::BlockHeader,
    end_of_archive_header::EndOfArchiveHeader,
    file_header::FileHeader,
    general_block_header::GeneralBlockHeader,
    main_archive_header::MainArchiveHeader,
};
use std::io::Cursor;

fn build_block(block: &mut dyn Block) -> Vec<u8> {
    let mut buffer = Vec::new();
    block.encode(&mut buffer).unwrap();
    buffer
}

#[test]
fn test_archive_iteration() {
    let mut mock_data: Vec<u8> = vec![
        0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x01, 0x00, // Signature
    ];

    let mut main_header = MainArchiveHeader::default();
    mock_data.extend_from_slice(&build_block(&mut main_header));

    let mut file_header = FileHeader {
        file_name: "test".to_string(),
        ..Default::default()
    };
    mock_data.extend_from_slice(&build_block(&mut file_header));

    let mut end_header = EndOfArchiveHeader::default();
    mock_data.extend_from_slice(&build_block(&mut end_header));

    let mut archive = Archive::new(Cursor::new(mock_data)).unwrap();

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
    let mut main_header = MainArchiveHeader::default();
    mock_data.extend_from_slice(&build_block(&mut main_header));

    let mut file_header = FileHeader {
        file_name: "test".to_string(),
        ..Default::default()
    };
    mock_data.extend_from_slice(&build_block(&mut file_header));

    let mut end_header = EndOfArchiveHeader::default();
    mock_data.extend_from_slice(&build_block(&mut end_header));

    let archive = Archive::new(Cursor::new(mock_data)).unwrap();
    let blocks: Vec<_> = archive.map(Result::unwrap).collect();

    assert_eq!(blocks.len(), 3);
    assert!(matches!(blocks[0], BlockHeader::Main(_)));
    assert!(matches!(blocks[1], BlockHeader::File(_)));
    assert!(matches!(blocks[2], BlockHeader::End(_)));
}

#[test]
fn test_read_file_data() {
    let compressed_data = vec![1, 2, 3, 4, 5];
    let mut file_header = FileHeader {
        base: BaseBlock {
            general_header: GeneralBlockHeader {
                header_flags: 0x0001,
                data_size: Some(compressed_data.len() as u64),
                ..Default::default()
            },
        },
        file_name: "test_file.txt".to_string(),
        ..Default::default()
    };

    let mut mock_data: Vec<u8> = vec![
        0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x01, 0x00, // Signature
    ];
    mock_data.extend_from_slice(&build_block(&mut file_header));
    mock_data.extend_from_slice(&compressed_data);

    let mut archive = Archive::new(Cursor::new(mock_data)).unwrap();

    match archive.next() {
        Some(Ok(BlockHeader::File(header))) => {
            let decompressed_data = archive.read_file_data(&header).unwrap();
            assert_eq!(decompressed_data, compressed_data);
        }
        Some(Ok(other)) => {
            panic!("Expected a FileHeader, but got {:?}", other);
        }
        Some(Err(e)) => {
            panic!("Expected a FileHeader, but got an error: {}", e);
        }
        None => {
            panic!("Expected a FileHeader, but the archive was empty");
        }
    }
}
