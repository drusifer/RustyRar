// src/structures.rs

/// Represents the fixed 8-byte RAR 5.0 signature.
pub struct RarSignature {
    pub signature: [u8; 8],
}

/// Represents the general header found at the beginning of most RAR blocks.
#[derive(Debug, PartialEq, Eq)]
pub struct GeneralBlockHeader {
    pub crc32: u32,          // Checksum of the block header
    pub header_size: u64,    // Total size of the block header (variable-length integer)
    pub header_type: u64,    // Type of the block (variable-length integer)
    pub header_flags: u64,   // Bitmask of flags (variable-length integer)
    pub data_size: Option<u64>, // Size of the data area following the header (optional, variable-length integer)
}

/// Represents the Main Archive Header block.
#[derive(Debug, PartialEq, Eq)]
pub struct MainArchiveHeader {
    pub archive_flags: u64, // Bitmask of archive properties (variable-length integer)
    pub volume_number: Option<u64>, // Sequence number of the volume (optional, variable-length integer)
}

/// Represents a File Header block.
#[derive(Debug, PartialEq, Eq)]
pub struct FileHeader {
    pub file_flags: u64,     // Bitmask of file properties (variable-length integer)
    pub unpacked_size: u64,  // Original, uncompressed size of the file (variable-length integer)
    pub file_attributes: u64,// Host OS-specific file attributes (variable-length integer)
    pub modification_time: Option<u32>, // 32-bit Unix timestamp (optional)
    pub file_crc32: Option<u32>,     // Checksum of the uncompressed file data (optional)
    pub compression_info: u64, // Compression algorithm and host OS (variable-length integer)
    pub file_name: String,  // The file name, encoded in UTF-8
    pub symlink_target: Option<String>, // Symlink target path (optional)
}

/// Represents the End of Archive Header block.
#[derive(Debug, PartialEq, Eq)]
pub struct EndOfArchiveHeader {
    pub end_archive_flags: u64, // Bitmask of properties (variable-length integer)
}

#[derive(Debug, PartialEq, Eq)]
pub enum BlockHeader {
    Main(MainArchiveHeader),
    File(FileHeader),
    End(EndOfArchiveHeader),
}
