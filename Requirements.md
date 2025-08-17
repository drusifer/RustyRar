# Project Requirements

This file tracks the user stories and requirements for the `rar-rs` project.

## User Stories

*   **As a developer, I want a Rust library that can parse and extract files from a RAR 5.0 archive, so that I can programmatically access the contents of the archive.**
    *   [Y] Define core data structures for the RAR format (`src/structures.rs`).
    *   [Y] Implement a decoder for the general block header (`src/decoder.rs`).
    *   [Y] Write unit tests for the decoder (`src/decoder.rs`).
    *   [Y] Implement decoders for specific block types (Main Archive Header, File Header, End of Archive Header).
    *   [Y] Create an `Archive` reader to iterate through blocks.
    *   [Y] Refactor the code to use a `Block` trait and an object-oriented design.
    *   [Y] Implement file data extraction and decompression (with a dummy decompressor).

*   **As a developer, I want to create a Decompressor class that encapsulates the "Decompress Data" operation, so that the interface can be agnostic of the underlying decompression algorithm and easy to extend and easy to test.**
    *   [Y] Define a `Decompressor` trait with a `decompress` method.
    *   [ ] Create concrete implementations of the `Decompressor` trait for different compression algorithms (e.g., LZ, PPMd).
    *   [Y] Integrate the `Decompressor` into the `Archive` reader to handle file extraction.

*   **As a developer, I want the library to be a pure Rust implementation, with no dependencies on external C/C++ bindings or command-line tools (like `rar` or `unrar`), so that it is self-contained and portable.**
    *   [Y] Ensure all decompression algorithms are implemented in Rust or provided by Rust-native crates.
    *   [Y] Remove any reliance on the `rar` command-line tool for testing or other purposes.

*   **As a command-line user, I want a utility to pack and unpack .rar files, so that I can easily manage RAR archives from my terminal.**
    *   [ ] Create a new binary crate for the command-line utility.
    *   [ ] Implement command-line argument parsing.
    *   [ ] Implement the `unpack` command.
    *   [ ] Implement the `pack` command.
    *   [ ] Implement the `list` command.
    *   [ ] Implement the `test` command.

---
*Key: [Y] = Done, [O] = In Progress / No Tests, [X] = Blocked/Failing, [ ] = Not Started*
