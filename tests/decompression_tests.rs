// tests/decompression_tests.rs

use app::archive::Archive;
use app::structures::base::BlockHeader;
use std::fs::File;

#[test]
fn test_uncompressed_rar_archive() {
    let file = File::open("tests/assets/uncompressed.rar").unwrap();
    let mut archive = Archive::new(file).unwrap();

    let mut found_file = false;
    while let Some(block) = archive.next() {
        match block.unwrap() {
            BlockHeader::File(file_header) => {
                if file_header.file_name == "uncompressed_test.txt" {
                    found_file = true;
                    let decompressed_data = archive.read_file_data(&file_header).unwrap();
                    let original_content = "This is an uncompressed test file.\n";
                    assert_eq!(decompressed_data, original_content.as_bytes());
                }
            }
            _ => {}
        }
    }

    assert!(found_file, "The test file was not found in the archive.");
}
