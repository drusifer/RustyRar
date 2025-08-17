# Project Files Overview

This document provides a high-level overview of the files in the `rar-rs` project, optimized for understanding by an AI assistant.

## Source Code (`src/`)

*   **`lib.rs`**: The main library crate root.
*   **`archive.rs`**: Contains the `Archive` struct, which is the main entry point for reading a RAR archive. It provides an iterator over the blocks in the archive.
*   **`decoder.rs`**: Contains low-level functions for decoding the RAR 5.0 format, including `read_vint` and `read_general_block_header`.
*   **`encoder.rs`**: Contains low-level functions for encoding the RAR 5.0 format, primarily used for testing.
*   **`decompression.rs`**: Contains the `Decompressor` trait and a `DummyDecompressor` implementation. This module will be expanded to include actual decompression algorithms.
*   **`structures/`**: A module containing the data structures that map to the RAR 5.0 format.
    *   **`mod.rs`**: Declares the submodules within `structures`.
    *   **`base.rs`**: Contains the `BlockHeader` enum, which represents the different types of blocks in a RAR archive.
    *   **`block.rs`**: Contains the `Block` trait, which defines the common interface for all block types.
    *   **`main_archive_header.rs`**: Contains the `MainArchiveHeader` struct.
    *   **`file_header.rs`**: Contains the `FileHeader` struct.
    *   **`end_of_archive_header.rs`**: Contains the `EndOfArchiveHeader` struct.
    *   **`general_block_header.rs`**: Contains the `GeneralBlockHeader` struct, which is the header common to all block types.

## Tests (`tests/`)

*   **`archive_tests.rs`**: Integration tests for the `Archive` reader.
*   **`decoder_tests.rs`**: Unit tests for the functions in `src/decoder.rs`.
*   **`encoder_tests.rs`**: Unit tests for the functions in `src/encoder.rs`.
*   **`decompression_tests.rs`**: Unit tests for the decompression logic.
*   **`focused_archive_tests.rs`**: A focused test for reading a single block from an archive.

## Project Documentation (`*.md`)

*   **`README.md`**: The main project README.
*   **`Requirements.md`**: The project requirements and user stories.
*   **`Arch-Block-Oriented.md`**: The project's architecture documentation.
*   **`Plan-Implementation.md`**: The project's implementation plan.
*   **`COMMANDS.md`**: The custom commands for interacting with the AI assistant.
*   **`CURRENT_STEP.md`**: Tracks the current development step.
*   **`Files.md`**: This file.
