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

## Phase 2: Archive Reader API & Refactoring

Once the low-level decoders are complete, we create a higher-level API and improve the code structure.

1.  **[Done] Create `Archive` Struct:** This struct manages the underlying reader and provides an iterator-like interface to access the blocks.
2.  **[Done] Implement Block Iterator:** The user can loop through the archive's blocks seamlessly.
3.  **[Done] Refactor to Object-Oriented Design:** Restructure the code to use a `Block` trait and specific block type implementations for better organization and maintainability.
4.  **[Done] Refine Module Structure:** Break down the monolithic `structures.rs` file into a more organized module, with each block type in its own file.

## Phase 3: Decompression

The final phase will involve integrating a decompression library to extract the actual file data.

1.  **[Done] Define `Decompressor` Trait:** Create a `Decompressor` trait in `src/decompression.rs` to provide a common interface for decompression algorithms.
2.  **[In Progress] Implement Decompression:**
    *   **[Done]** Create a `DummyDecompressor` for uncompressed files.
    *   **[ ]** Research and select a suitable crate for LZMA compression.
    *   **[ ]** Implement an `LzmaDecompressor`.
    *   **[ ]** Update the `get_decompressor` factory to return the correct decompressor based on the file header.
3.  **[Done] Integrate Decompression:** Add logic to the `ArchiveReader` to decompress and return the file data.
