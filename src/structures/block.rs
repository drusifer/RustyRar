// src/structures/block.rs

use crate::structures::general_block_header::GeneralBlockHeader;
use std::io::{self, Read, Write};
use crate::encoder::write_vint;

/// The `Block` trait defines a common interface for all RAR block types.
/// It provides methods for encoding and decoding the block's specific data.
pub trait Block {
    fn get_base(&self) -> &BaseBlock;
    fn get_mut_base(&mut self) -> &mut BaseBlock;
    fn header_type(&self) -> u64;

    /// Encodes the specific data of the block to a writer.
    fn encode_data(&self, writer: &mut dyn Write) -> io::Result<()>;

    /// Decodes the specific data of the block from a reader.
    fn decode_data(&mut self, reader: &mut dyn Read) -> io::Result<()>;

    fn encode(&self, writer: &mut dyn Write) -> io::Result<()> {
        let mut temp_buffer = Vec::new();
        let temp_writer = &mut temp_buffer;

        // Write header type and flags first
        write_vint(temp_writer, self.header_type())?;
        write_vint(temp_writer, self.get_base().general_header.header_flags)?;

        // Write data size if it exists
        if let Some(data_size) = self.get_base().general_header.data_size {
            write_vint(temp_writer, data_size)?;
        }

        // Write the specific data of the block
        self.encode_data(temp_writer)?;

        // Now we can calculate the header size
        let header_size = temp_buffer.len() as u64;

        // Write the CRC32 (dummy value for now), header size, and the rest of the header
        writer.write_all(&[0,0,0,0])?; // Dummy CRC32
        write_vint(writer, header_size)?;
        writer.write_all(&temp_buffer)?;

        Ok(())
    }

    fn decode(general_header: GeneralBlockHeader, reader: &mut dyn Read) -> io::Result<Self>
    where
        Self: Sized + Default,
    {
        let mut limited_reader = reader.take(general_header.header_size);
        let mut block = Self::default();
        *block.get_mut_base() = BaseBlock { general_header };
        block.decode_data(&mut limited_reader)?;
        Ok(block)
    }
}

/// `BaseBlock` contains the `GeneralBlockHeader`, which is common to all block types.
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct BaseBlock {
    pub general_header: GeneralBlockHeader,
}
