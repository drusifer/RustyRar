// src/structures/main_archive_header.rs

use crate::structures::block::{Block, BaseBlock};
use std::io::{self, Write, Read};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct MainArchiveHeader {
    pub base: BaseBlock,
    // Add Main Archive Header specific fields here
}

impl Block for MainArchiveHeader {
    fn get_base(&self) -> &BaseBlock {
        &self.base
    }

    fn get_mut_base(&mut self) -> &mut BaseBlock {
        &mut self.base
    }

    fn header_type(&self) -> u64 {
        1
    }

    fn encode_data(&self, _writer: &mut dyn Write) -> io::Result<()> {
        // No specific data to encode for MainArchiveHeader
        Ok(())
    }

    fn decode_data(&mut self, _reader: &mut dyn Read) -> io::Result<()> {
        // No specific data to decode for MainArchiveHeader
        Ok(())
    }
}
