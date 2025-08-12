use crate::block::{Block, BaseBlock};
use crate::decoder::{read_vint};
use crate::encoder::{write_vint};
use std::io::{self, Read, Write};

/// Represents the fixed 8-byte RAR 5.0 signature.
pub struct RarSignature {
    pub signature: [u8; 8],
}

/// Represents the general header found at the beginning of most RAR blocks.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct GeneralBlockHeader {
    pub crc32: u32,          // Checksum of the block header
    pub header_size: u64,    // Total size of the block header (variable-length integer)
    pub header_type: u64,    // Type of the block (variable-length integer)
    pub header_flags: u64,   // Bitmask of flags (variable-length integer)
    pub data_size: Option<u64>, // Size of the data area following the header (optional, variable-length integer)
}

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

/// Represents a File Header block.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct FileHeader {
    pub base: BaseBlock,
    pub file_flags: u64,     // Bitmask of file properties (variable-length integer)
    pub unpacked_size: u64,  // Original, uncompressed size of the file (variable-length integer)
    pub file_attributes: u64,// Host OS-specific file attributes (variable-length integer)
    pub modification_time: Option<u32>, // 32-bit Unix timestamp (optional)
    pub file_crc32: Option<u32>,     // Checksum of the uncompressed file data (optional)
    pub compression_info: u64, // Compression algorithm and host OS (variable-length integer)
    pub file_name: String,  // The file name, encoded in UTF-8
    pub symlink_target: Option<String>, // Symlink target path (optional)
}

impl Block for FileHeader {
    fn get_base(&self) -> &BaseBlock {
        &self.base
    }

    fn get_mut_base(&mut self) -> &mut BaseBlock {
        &mut self.base
    }

    fn encode_data(&self, writer: &mut dyn Write) -> io::Result<()> {
        write_vint(writer, self.file_flags)?;
        write_vint(writer, self.unpacked_size)?;
        write_vint(writer, self.file_attributes)?;
        if let Some(modification_time) = self.modification_time {
            writer.write_all(&modification_time.to_le_bytes())?;
        }
        if let Some(file_crc32) = self.file_crc32 {
            writer.write_all(&file_crc32.to_le_bytes())?;
        }
        write_vint(writer, self.compression_info)?;
        write_vint(writer, self.file_name.len() as u64)?;
        writer.write_all(self.file_name.as_bytes())?;
        if let Some(symlink_target) = &self.symlink_target {
            writer.write_all(symlink_target.as_bytes())?;
        }
        Ok(())
    }

    fn decode_data(&mut self, reader: &mut dyn Read) -> io::Result<()> {
        self.file_flags = read_vint(reader)?;
        self.unpacked_size = read_vint(reader)?;
        self.file_attributes = read_vint(reader)?;
        if (self.file_flags & 0x02) != 0 {
            let mut time_bytes = [0u8; 4];
            reader.read_exact(&mut time_bytes)?;
            self.modification_time = Some(u32::from_le_bytes(time_bytes));
        }
        if (self.file_flags & 0x04) != 0 {
            let mut crc_bytes = [0u8; 4];
            reader.read_exact(&mut crc_bytes)?;
            self.file_crc32 = Some(u32::from_le_bytes(crc_bytes));
        }
        self.compression_info = read_vint(reader)?;
        let file_name_length = read_vint(reader)? as usize;
        let mut file_name_bytes = vec![0; file_name_length];
        reader.read_exact(&mut file_name_bytes)?;
        self.file_name = String::from_utf8(file_name_bytes.clone()).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 in file name")
        })?;
        if (self.file_flags & 0x08) != 0 {
            self.symlink_target = Some(
                String::from_utf8(file_name_bytes)
                    .map_err(|_| {
                        io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 in symlink target")
                    })?,
            );
        }
        Ok(())
    }
}

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

#[derive(Debug, PartialEq, Eq)]
pub enum BlockHeader {
    Main(MainArchiveHeader),
    File(FileHeader),
    End(EndOfArchiveHeader),
}
