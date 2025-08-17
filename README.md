# RustRar (`rar-rs`)

A pure Rust implementation for reading and unpacking WinRAR 5.0 archives.

## Project Goal

The primary goal of this project is to create a robust, memory-safe, and efficient Rust library capable of parsing and extracting files from RAR 5.0 archives without relying on external C libraries or command-line tools.

## Current Status

The core decoding engine is implemented and the project is now unblocked. The library can successfully parse RAR 5.0 archives and extract uncompressed files. The next step is to implement support for compressed files, starting with the LZMA algorithm.

*   **Requirements:** See [Requirements.md](Requirements.md) for a detailed view of project goals.
*   **Architecture:** The project architecture is documented in [Arch-Block-Oriented.md](Arch-Block-Oriented.md).
*   **Plan:** The overall development plan is in [Plan-Implementation.md](Plan-Implementation.md).

## How to Restore State

To get the AI assistant up to speed on the project, instruct it to perform the `*load` command, which involves reading the `.md` files in this repository.
