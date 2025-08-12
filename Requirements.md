# Project Requirements

This file tracks the user stories and requirements for the `rar-rs` project.

## User Stories

*   **As a developer, I want a Rust library that can parse and extract files from a RAR 5.0 archive, so that I can programmatically access the contents of the archive.**
    *   [Y] Define core data structures for the RAR format (`src/structures.rs`).
    *   [Y] Implement a decoder for the general block header (`src/decoder.rs`).
    *   [Y] Write unit tests for the decoder (`src/decoder.rs`).
    *   [Y] Implement decoders for specific block types (Main Archive Header, File Header, End of Archive Header).
    *   [ ] Create an `Archive` reader to iterate through blocks.
    *   [ ] Implement file data extraction and decompression.

---
*Key: [Y] = Done, [O] = In Progress / No Tests, [ ] = Not Started*
