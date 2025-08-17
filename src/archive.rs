// src/archive.rs

use crate::decoder::read_general_block_header;
use crate::structures::base::BlockHeader;
use crate::structures::block::Block;
use crate::structures::end_of_archive_header::EndOfArchiveHeader;
use crate::structures::file_header::FileHeader;
use crate::structures::main_archive_header::MainArchiveHeader;
use std::io::{self, Read};
use log::debug;

pub struct Archive<R: Read> {
    reader: R,
    current_block_data_size: u64,
}

impl<R: Read> Archive<R> {
    pub fn new(mut reader: R) -> io::Result<Self> {
        let mut signature = [0u8; 8];
        reader.read_exact(&mut signature)?;
        // TODO: Add signature validation.

        Ok(Archive {
            reader,
            current_block_data_size: 0,
        })
    }

    pub fn read_file_data(&mut self, file_header: &FileHeader) -> io::Result<Vec<u8>> {
        if let Some(compressed_size) = file_header.base.general_header.data_size {
            let mut compressed_data = vec![0; compressed_size as usize];
            self.reader.read_exact(&mut compressed_data)?;
            self.current_block_data_size = 0; // Data is consumed.

            file_header
                .decompress(&compressed_data)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Decompression error: {:?}", e)))
        } else {
            Ok(Vec::new())
        }
    }

    fn advance_reader(&mut self) -> io::Result<()> {
        if self.current_block_data_size > 0 {
            debug!("[advance_reader] Skipping {} bytes", self.current_block_data_size);
            let mut limited_reader = self.reader.by_ref().take(self.current_block_data_size);
            io::copy(&mut limited_reader, &mut io::sink())?;
            self.current_block_data_size = 0;
        }
        Ok(())
    }
}

impl<R: Read> Iterator for Archive<R> {
    type Item = io::Result<BlockHeader>;

    fn next(&mut self) -> Option<Self::Item> {
        debug!("[next] Advancing reader...");
        if let Err(e) = self.advance_reader() {
            return Some(Err(e));
        }

        debug!("[next] Reading general block header...");
        let general_header = match read_general_block_header(&mut self.reader) {
            Ok(header) => {
                debug!("[next] Read general block header: {:?}", header);
                header
            }
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                debug!("[next] EOF reached.");
                return None;
            }
            Err(e) => {
                debug!("[next] Error reading general block header: {}", e);
                return Some(Err(e));
            }
        };

        if let Some(data_size) = general_header.data_size {
            debug!("[next] Setting current_block_data_size to {}", data_size);
            self.current_block_data_size = data_size;
        }

        let block_header_result = match general_header.header_type {
            1 => MainArchiveHeader::decode(general_header, &mut self.reader)
                .map(BlockHeader::Main),
            2 => FileHeader::decode(general_header, &mut self.reader)
                .map(BlockHeader::File),
            5 => EndOfArchiveHeader::decode(general_header, &mut self.reader)
                .map(BlockHeader::End),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unknown block type",
            )),
        };

        match block_header_result {
            Ok(block_header) => {
                debug!("[next] Successfully decoded block header.");
                Some(Ok(block_header))
            }
            Err(e) => {
                debug!("[next] Error decoding block header: {}", e);
                Some(Err(e))
            }
        }
    }
}
