# Architecture: Decoder

This document outlines the architecture for the `rar-rs` decoder, which is responsible for parsing the block-based structure of a RAR 5.0 archive.

## Component Overview

The decoder is designed as a set of functions that operate on a `std::io::Read` source. This approach keeps the decoding logic separate from the underlying data source (e.g., file, network stream), promoting flexibility and testability.

### Key Components:

1.  **`structures.rs`**: Contains the Rust struct definitions that mirror the RAR 5.0 binary format as described in `specs.md`. These are plain data objects with no logic.
2.  **`decoder.rs`**: Contains the parsing logic to read from a byte stream and populate the structs in `structures.rs`. The `decoder` module provides functions to read and parse each block type.

## Control Flow

The primary control flow for decoding involves reading a block, identifying its type, and then dispatching to a type-specific parser.

### Mermaid Diagram: Decoding Process

```mermaid
sequenceDiagram;
    participant User;
    participant ArchiveReader;
    participant Decoder;
     
    User->>ArchiveReader: open("archive.rar")
    ArchiveReader->>Decoder: read_signature()
    loop For each block in archive
        ArchiveReader->>Decoder: read_general_block_header()
        Decoder-->>ArchiveReader: GeneralBlockHeader
        alt Header Type is File
            ArchiveReader->>Decoder: read_file_header()
            Decoder-->>ArchiveReader: FileHeader
            User->>ArchiveReader: read_file_data()
        else Header Type is Main
            ArchiveReader->>Decoder: read_main_archive_header()
            Decoder-->>ArchiveReader: MainArchiveHeader
        else Header Type is End of Archive
            ArchiveReader->>Decoder: read_end_of_archive_header()
            Decoder-->>ArchiveReader: EndOfArchiveHeader
        end;
    end;
```

