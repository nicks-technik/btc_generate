# Tech Stack: BTC Mnemonic & Address Generator (Rust-based with Parallelism)

## Language & Runtime
- **Rust (latest stable):** High-performance, memory-safe language ideal for cryptographic operations.
- **Cargo:** Rust's built-in package manager and build system.

## Parallelism & Performance
- **rayon:** A data-parallelism library that makes it easy to parallelize the generation process across multiple CPU cores.

## Cryptographic Libraries
- **bitcoin:** The primary crate for Bitcoin support, including address derivation and key management.
- **bip39:** A crate for generating and validating BIP-39 mnemonic phrases.
- **tiny-hdk:** Provides BIP-32 (hierarchical deterministic) key derivation support.

## Tooling & CLI
- **clap:** A robust and efficient library for building command-line interfaces.
- **serde:** For high-performance serialization and deserialization of log data.
- **csv:** For writing generated mnemonics and addresses to structured CSV files.

## Development & Testing
- **cargo test:** Rust's built-in testing framework for unit and integration tests.
- **clippy:** An advanced linter for Rust to ensure high-quality, idiomatic code.
- **rustfmt:** The standard Rust code formatter.

## Persistence
- **CSV or JSON:** Efficient file-based logging for storing generated data.
