// src/structures/main_archive_header.rs

use crate::structures::block::{Block, BaseBlock};
use crate::decoder::read_vint;
use crate::encoder::write_vint;
use std::io::{self, Read, Write};

/// Represents the Main Archive Header block.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct MainArchiveHeader {
    pub base: BaseBlock,
    pub archive_flags: u64, // Bitmask of archive properties (variable-length integer)
    pub volume_number: Option<u64>, // Sequence number of the volume (optional, variable-length integer)
}

impl Block for MainArchiveHeader {
    fn get_base(&self) -> &BaseBlock {
        &self.base
    }

    fn get_mut_base(&mut self) -> &mut BaseBlock {
        &mut self.base
    }

    fn encode_data(&self, writer: &mut dyn Write) -> io::Result<()> {
        write_vint(writer, self.archive_flags)?;
        if let Some(volume_number) = self.volume_number {
            write_vint(writer, volume_number)?;
        }
        Ok(())
    }

    fn decode_data(&mut self, reader: &mut dyn Read) -> io::Result<()> {
        self.archive_flags = read_vint(reader)?;
        if (self.archive_flags & 0x0001) != 0 {
            self.volume_number = Some(read_vint(reader)?);
        }
        Ok(())
    }
}
