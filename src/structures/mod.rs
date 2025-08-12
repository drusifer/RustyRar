// src/structures/mod.rs

pub mod block;
pub mod base;
pub mod general_block_header;
pub mod main_archive_header;
pub mod file_header;
pub mod end_of_archive_header;

pub use block::{Block, BaseBlock};
pub use base::{RarSignature, BlockHeader};
pub use general_block_header::GeneralBlockHeader;
pub use main_archive_header::MainArchiveHeader;
pub use file_header::FileHeader;
pub use end_of_archive_header::EndOfArchiveHeader;
