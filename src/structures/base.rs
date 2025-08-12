// src/structures/base.rs

use crate::structures::{EndOfArchiveHeader, FileHeader, MainArchiveHeader};

/// Represents the fixed 8-byte RAR 5.0 signature.
pub struct RarSignature {
    pub signature: [u8; 8],
}

#[derive(Debug, PartialEq, Eq)]
pub enum BlockHeader {
    Main(MainArchiveHeader),
    File(FileHeader),
    End(EndOfArchiveHeader),
}
