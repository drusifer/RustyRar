// src/decompression.rs

use crate::structures::file_header::FileHeader;

#[derive(Debug)]
pub enum DecompressionError {
    InvalidFormat,
    // Other potential errors can be added here
}

pub trait Decompressor {
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, DecompressionError>;
}

// A dummy decompressor for initial testing and integration.
pub struct DummyDecompressor;

impl Decompressor for DummyDecompressor {
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, DecompressionError> {
        // For now, it simply returns a copy of the input data.
        Ok(data.to_vec())
    }
}

pub fn get_decompressor(_compression_method: u64) -> Box<dyn Decompressor> {
    // For now, we only have the dummy decompressor.
    // In the future, this function will inspect the compression_method
    // to determine which decompressor to return.
    Box::new(DummyDecompressor)
}

pub fn decompress_data(compressed_data: &[u8], compression_method: u64) -> Result<Vec<u8>, DecompressionError> {
    let decompressor = get_decompressor(compression_method);
    decompressor.decompress(compressed_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_decompressor_returns_input_as_is() {
        let decompressor = DummyDecompressor;
        let test_data = vec![10, 20, 30, 40, 50];
        let result = decompressor.decompress(&test_data).unwrap();
        assert_eq!(result, test_data);
    }

    #[test]
    fn get_decompressor_returns_dummy_decompressor() {
        let decompressor = get_decompressor(0);
        let test_data = vec![1, 2, 3];
        let result = decompressor.decompress(&test_data).unwrap();
        assert_eq!(result, test_data);
    }
}
