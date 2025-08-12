// src/block.rs

use crate::encoder::write_general_block_header;
use crate::structures::GeneralBlockHeader;
use std::io::{self, Read, Write};

/// The `Block` trait defines a common interface for all RAR block types.
/// It provides methods for encoding and decoding the block's specific data.
pub trait Block {
    fn get_base(&self) -> &BaseBlock;
    fn get_mut_base(&mut self) -> &mut BaseBlock;

    /// Encodes the specific data of the block to a writer.
    fn encode_data(&self, writer: &mut dyn Write) -> io::Result<()>;

    /// Decodes the specific data of the block from a reader.
    fn decode_data(&mut self, reader: &mut dyn Read) -> io::Result<()>;

    fn encode(&self, writer: &mut dyn Write) -> io::Result<()> {
        let mut specific_data = Vec::new();
        self.encode_data(&mut specific_data)?;

        let mut base = self.get_base().clone();
        base.general_header.header_size = specific_data.len() as u64;

        let mut general_header_data = Vec::new();
        write_general_block_header(&mut general_header_data, &base.general_header)?;

        writer.write_all(&general_header_data)?;
        writer.write_all(&specific_data)?;

        Ok(())
    }

    fn decode(general_header: GeneralBlockHeader, reader: &mut dyn Read) -> io::Result<Self>
    where
        Self: Sized + Default,
    {
        let mut block = Self::default();
        *block.get_mut_base() = BaseBlock { general_header };
        block.decode_data(reader)?;
        Ok(block)
    }
}

/// `BaseBlock` contains the `GeneralBlockHeader`, which is common to all block types.
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct BaseBlock {
    pub general_header: GeneralBlockHeader,
}

impl Block for BaseBlock {
    fn get_base(&self) -> &BaseBlock {
        self
    }

    fn get_mut_base(&mut self) -> &mut BaseBlock {
        self
    }

    fn encode_data(&self, _writer: &mut dyn Write) -> io::Result<()> {
        Ok(())
    }

    fn decode_data(&mut self, _reader: &mut dyn Read) -> io::Result<()> {
        Ok(())
    }
}
