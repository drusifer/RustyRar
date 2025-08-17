// tests/encoder_tests.rs

use app::encoder::write_vint;

#[test]
fn test_write_vint() {
    let mut buffer = Vec::new();
    write_vint(&mut buffer, 0x01).unwrap();
    assert_eq!(buffer, vec![0x01]);

    buffer.clear();

    write_vint(&mut buffer, 0x80).unwrap();
    assert_eq!(buffer, vec![0x80, 0x01]);

    buffer.clear();

    write_vint(&mut buffer, 0x3FFF).unwrap();
    assert_eq!(buffer, vec![0xFF, 0x7F]);

    buffer.clear();

    write_vint(&mut buffer, 0x4000).unwrap();
    assert_eq!(buffer, vec![0x80, 0x80, 0x01]);
}
