// src/structures/end_of_archive_header.rs

use crate::structures::block::{Block, BaseBlock};
use crate::decoder::read_vint;
use crate::encoder::write_vint;
use std::io::{self, Read, Write};

/// Represents the End of Archive Header block.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct EndOfArchiveHeader {
    pub base: BaseBlock,
    pub end_archive_flags: u64, // Bitmask of properties (variable-length integer)
}

impl Block for EndOfArchiveHeader {
    fn get_base(&self) -> &BaseBlock {
        &self.base
    }

    fn get_mut_base(&mut self) -> &mut BaseBlock {
        &mut self.base
    }

    fn encode_data(&self, writer: &mut dyn Write) -> io::Result<()> {
        write_vint(writer, self.end_archive_flags)
    }

    fn decode_data(&mut self, reader: &mut dyn Read) -> io::Result<()> {
        self.end_archive_flags = read_vint(reader)?;
        Ok(())
    }
}
