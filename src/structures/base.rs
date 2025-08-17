// src/structures/base.rs

use crate::structures::{
    end_of_archive_header::EndOfArchiveHeader, file_header::FileHeader,
    main_archive_header::MainArchiveHeader,
};

/// An enumeration of all possible block types in a RAR archive.
#[derive(Debug, PartialEq, Eq)]
pub enum BlockHeader {
    Main(MainArchiveHeader),
    File(FileHeader),
    End(EndOfArchiveHeader),
}
