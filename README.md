# RustRar (`rar-rs`)

A pure Rust implementation for reading and unpacking WinRAR 5.0 archives.

## Project Goal

The primary goal of this project is to create a robust, memory-safe, and efficient Rust library capable of parsing and extracting files from RAR 5.0 archives without relying on external C libraries.

## Current Status

The core decoding engine is now complete. The project is moving into Phase 2, which focuses on building a high-level `Archive` reader API for iterating through archive blocks.

*   **Requirements:** See [Requirements.md](Requirements.md) for the latest user stories and progress.
*   **Architecture:** The current architecture is documented in [Arch-Decoder.md](Arch-Decoder.md).
*   **Plan:** The development plan is outlined in [Plan-Implementation.md](Plan-Implementation.md).

## Getting Started

The project is managed with Cargo. To build and test the code, use the following commands:

```bash
# Build the project
cargo build

# Run the unit tests
cargo test
```

## How to Restore State

To get the AI assistant up to speed on the project, instruct it to perform the `*load` command, which involves reading the `.md` files in this repository.
