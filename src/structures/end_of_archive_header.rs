// src/structures/end_of_archive_header.rs

use crate::structures::block::{Block, BaseBlock};
use std::io::{self, Read, Write};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct EndOfArchiveHeader {
    pub base: BaseBlock,
    // Add End of Archive Header specific fields here
}

impl Block for EndOfArchiveHeader {
    fn get_base(&self) -> &BaseBlock {
        &self.base
    }

    fn get_mut_base(&mut self) -> &mut BaseBlock {
        &mut self.base
    }

    fn header_type(&self) -> u64 {
        5
    }

    fn encode_data(&self, _writer: &mut dyn Write) -> io::Result<()> {
        // No specific data to encode for EndOfArchiveHeader
        Ok(())
    }

    fn decode_data(&mut self, _reader: &mut dyn Read) -> io::Result<()> {
        // No specific data to decode for EndOfArchiveHeader
        Ok(())
    }
}
