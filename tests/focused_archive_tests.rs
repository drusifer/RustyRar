// tests/focused_archive_tests.rs

use app::archive::Archive;
use app::structures::{
    block::Block,
    base::BlockHeader,
    file_header::FileHeader,
};
use std::io::Cursor;

fn build_block<B: Block>(block: &mut B) -> Vec<u8> {
    let mut buffer = Vec::<u8>::new();
    block.encode(&mut buffer).unwrap();
    buffer
}

#[test]
fn test_read_single_file_header_block() {
    let mut mock_data: Vec<u8> = vec![
        0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x01, 0x00, // Signature
    ];

    let mut file_header = FileHeader {
        file_name: "test".to_string(),
        ..Default::default()
    };
    mock_data.extend_from_slice(&build_block(&mut file_header));

    let mut archive = Archive::new(Cursor::new(mock_data)).unwrap();

    // The first and only block should be the file header
    let block = archive.next().unwrap().unwrap();
    assert!(matches!(block, BlockHeader::File(_)));

    // No more blocks
    assert!(archive.next().is_none());
}
