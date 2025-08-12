// src/structures/general_block_header.rs

/// Represents the general header found at the beginning of most RAR blocks.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct GeneralBlockHeader {
    pub crc32: u32,          // Checksum of the block header
    pub header_size: u64,    // Total size of the block header (variable-length integer)
    pub header_type: u64,    // Type of the block (variable-length integer)
    pub header_flags: u64,   // Bitmask of flags (variable-length integer)
    pub data_size: Option<u64>, // Size of the data area following the header (optional, variable-length integer)
}
