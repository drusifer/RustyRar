# Architecture: `rar-cli`

This document outlines the architecture for the `rar-cli` command-line utility.

## Workspace Architecture

The project is structured as a Cargo workspace to manage the `rar-rs` library and the `rar-cli` binary independently.

*   **`rar-rs` (Library):** The core library containing all logic for RAR file parsing, decompression, and encoding.
*   **`rar-cli` (Binary):** A command-line utility that acts as a user-friendly wrapper around the `rar-rs` library.

This separation of concerns allows the `rar-rs` library to be used by other applications, while the `rar-cli` provides a concrete implementation for end-users.

```mermaid
graph TD
    subgraph "rar-rs Workspace"
        A[rar-cli Crate] --> B[rar-rs Crate];
    end
```

## CLI Internal Architecture

The `rar-cli` utility will use the `clap` crate for command-line argument parsing. The main entry point (`main.rs`) will be responsible for parsing these arguments and dispatching the appropriate command to a dedicated module.

### Key Components:

*   **`main.rs`**: The application entry point. Its primary role is to initialize the argument parser and call the correct command handler.
*   **Command Modules (`list.rs`, `unpack.rs`, etc.)**: Each command will be implemented in its own module. This keeps the codebase organized and easy to maintain. Each module will contain a `run` function that takes the parsed arguments and executes the command's logic.

### Control Flow

The control flow is initiated by the user running a command. `clap` parses the input, and `main.rs` matches on the command to delegate the work to the appropriate module.
 
```mermaid
graph TD
    A["User runs rar-cli <command> <args>"] --> B{main.rs};
    B --> C{"Parse Arguments (clap)"};
    C --> D{"Dispatch Command"};
    D --> E[list::run];
    D --> F[unpack::run];
    D --> G[test::run];
    D --> H[pack::run];
```

## Class Relationships

The following diagram shows the high-level relationships between the main structs and traits in the workspace.

```mermaid
classDiagram
    direction BT

    subgraph rar-cli
        class main {
            <<entrypoint>>
        }
        class list_command {
            run()
        }
        class unpack_command {
            run()
        }
    end

    subgraph rar-rs
        class Archive {
            +next() : BlockHeader
            +read_file_data()
        }

        class BlockHeader {
            <<enum>>
            Main
            File
            End
        }

        class Block {
            <<trait>>
            decode()
            encode()
        }

        class FileHeader {
            data
        }
        class MainArchiveHeader {
            data
        }

        class Decompressor {
            <<trait>>
            decompress()
        }
        class DummyDecompressor
    end


    main --> list_command : calls
    main --> unpack_command : calls

    list_command ..> Archive : uses
    unpack_command ..> Archive : uses

    Archive --> BlockHeader : returns
    Archive ..> Block : uses for decoding

    Block <|.. FileHeader
    Block <|.. MainArchiveHeader

    unpack_command ..> Decompressor : uses
    Decompressor <|.. DummyDecompressor
```
