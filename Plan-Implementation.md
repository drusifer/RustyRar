# Project Plan: `rar-rs` Implementation

This document outlines the development plan for the `rar-rs` library.

## Phase 1: Core Decoding Engine

The initial focus is on building the core functionality to read and parse the structure of a RAR 5.0 archive without handling decompression.

1.  **[Done] Define Data Structures:** Create Rust structs in `src/structures.rs` that map directly to the RAR 5.0 binary format specifications.
2.  **[Done] Implement General Block Header Decoder:**
    *   Create `src/decoder.rs`.
    *   Implement `read_vint` to handle variable-length integers.
    *   Implement `read_general_block_header` to parse the header common to all blocks.
3.  **[Done] Write Unit Tests:**
    *   Add a `tests` module to `src/decoder.rs`.
    *   Write comprehensive unit tests for `read_vint` and `read_general_block_header`.
4.  **[Done] Implement Specific Block Decoders:**
    *   Add functions to `src/decoder.rs` for parsing:
        *   `MainArchiveHeader`
        *   `FileHeader`
        *   `EndOfArchiveHeader`
    *   Write corresponding unit tests for each new function.

## Phase 2: Archive Reader API

Once the low-level decoders are complete, we will create a higher-level API for iterating through the archive.

1.  **[Done] Create `Archive` Struct:** This struct will manage the underlying reader and provide an iterator-like interface to access the blocks.
2.  **[Done] Implement Block Iterator:** The user should be able to loop through the archive's blocks seamlessly.
3.  **[Done] Refactor to Object-Oriented Design:** Restructure the code to use a `Block` trait and specific block type implementations for better organization and maintainability.

## Phase 3: Decompression

The final phase will involve integrating a decompression library to extract the actual file data.

1.  **[Next] Research Decompression Crates:** Investigate and select a suitable crate for the compression algorithms used by RAR 5.0.
2.  **[ ] Integrate Decompression:** Add logic to the `ArchiveReader` to decompress and return the file data.
