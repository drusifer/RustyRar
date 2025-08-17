// src/structures/file_header.rs

use crate::decoder::read_vint;
use crate::encoder::write_vint;
use crate::structures::block::{Block, BaseBlock};
use crate::decompression::{DecompressionError, self};
use std::io::{self, Read, Write};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct FileHeader {
    pub base: BaseBlock,
    pub compression_method: u64,
    pub os_type: u64,
    pub file_crc32: u32,
    pub file_time: u64,
    pub file_name: String,
}

impl Block for FileHeader {
    fn get_base(&self) -> &BaseBlock {
        &self.base
    }

    fn get_mut_base(&mut self) -> &mut BaseBlock {
        &mut self.base
    }

    fn header_type(&self) -> u64 {
        2
    }

    fn encode_data(&self, writer: &mut dyn Write) -> io::Result<()> {
        println!("[FileHeader::encode_data] Encoding file header for '{}'", self.file_name);
        write_vint(writer, self.compression_method)?;
        write_vint(writer, self.os_type)?;
        writer.write_all(&self.file_crc32.to_le_bytes())?;
        write_vint(writer, self.file_time)?;
        write_vint(writer, self.file_name.len() as u64)?;
        writer.write_all(self.file_name.as_bytes())?;
        Ok(())
    }

    fn decode_data(&mut self, reader: &mut dyn Read) -> io::Result<()> {
        println!("[FileHeader::decode_data] Decoding file header...");
        self.compression_method = read_vint(reader)?;
        println!("[FileHeader::decode_data] Compression method: {}", self.compression_method);
        self.os_type = read_vint(reader)?;
        println!("[FileHeader::decode_data] OS type: {}", self.os_type);
        let mut crc32_bytes = [0u8; 4];
        reader.read_exact(&mut crc32_bytes)?;
        self.file_crc32 = u32::from_le_bytes(crc32_bytes);
        println!("[FileHeader::decode_data] File CRC32: {:#010x}", self.file_crc32);
        self.file_time = read_vint(reader)?;
        println!("[FileHeader::decode_data] File time: {}", self.file_time);
        let file_name_len = read_vint(reader)? as usize;
        println!("[FileHeader::decode_data] File name length: {}", file_name_len);
        let mut file_name_bytes = vec![0u8; file_name_len];
        reader.read_exact(&mut file_name_bytes)?;
        self.file_name = String::from_utf8(file_name_bytes)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        println!("[FileHeader::decode_data] File name: {}", self.file_name);
        Ok(())
    }
}

impl FileHeader {
    pub fn decompress(&self, compressed_data: &[u8]) -> Result<Vec<u8>, DecompressionError> {
        println!("[FileHeader::decompress] Decompressing file data...");
        decompression::decompress_data(compressed_data, self.compression_method)
    }
}
