// src/archive.rs

use crate::structures::block::Block;
use crate::decoder::read_general_block_header;
use crate::structures::{
    base::BlockHeader, end_of_archive_header::EndOfArchiveHeader, file_header::FileHeader,
    main_archive_header::MainArchiveHeader,
};
use std::io::{self, Read};

pub struct Archive<R: Read> {
    reader: R,
    first_block: bool,
}

impl<R: Read> Archive<R> {
    pub fn new(reader: R) -> Self {
        Archive {
            reader,
            first_block: true,
        }
    }
}

impl<R: Read> Iterator for Archive<R> {
    type Item = io::Result<BlockHeader>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_block {
            // Skip the 8-byte signature
            let mut signature = [0u8; 8];
            if self.reader.read_exact(&mut signature).is_err() {
                return Some(Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Could not read signature",
                )));
            }
            self.first_block = false;
        }

        let general_header = match read_general_block_header(&mut self.reader) {
            Ok(header) => header,
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return None,
            Err(e) => return Some(Err(e)),
        };

        match general_header.header_type {
            1 => {
                // Main Archive Header
                match MainArchiveHeader::decode(general_header, &mut self.reader) {
                    Ok(main_header) => Some(Ok(BlockHeader::Main(main_header))),
                    Err(e) => Some(Err(e)),
                }
            }
            2 => {
                // File Header
                match FileHeader::decode(general_header, &mut self.reader) {
                    Ok(file_header) => Some(Ok(BlockHeader::File(file_header))),
                    Err(e) => Some(Err(e)),
                }
            }
            5 => {
                // End of Archive
                match EndOfArchiveHeader::decode(general_header, &mut self.reader) {
                    Ok(end_header) => Some(Ok(BlockHeader::End(end_header))),
                    Err(e) => Some(Err(e)),
                }
            }
            _ => Some(Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unknown block type",
            ))),
        }
    }
}
