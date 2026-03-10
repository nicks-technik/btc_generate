# BTC Mnemonic & Address Generator

A high-performance, parallelized tool for generating Bitcoin mnemonic phrases (12 words) and deriving their corresponding Bitcoin addresses in various formats.

## Features

- **BIP-39 Mnemonic Generation:** Generates secure 12-word mnemonics.
- **Multi-Format Address Derivation:** Supports derivation of:
  - **Legacy (P2PKH):** `m/44'/0'/0'/0/0`
  - **SegWit (P2WPKH):** `m/84'/0'/0'/0/0`
  - **Taproot (P2TR):** `m/86'/0'/0'/0/0`
- **Parallel Processing:** Utilizes `rayon` for fast batch generation across multiple CPU cores.
- **CSV Logging:** Automatically logs generated mnemonics and addresses to a structured CSV file.
- **Offline Secure:** Designed to run entirely offline with no network dependencies.

## Tech Stack

- **Rust:** Core language for performance and safety.
- **bitcoin-rust:** For Bitcoin address derivation and key management.
- **bip39:** For mnemonic phrase generation.
- **rayon:** For parallel processing.
- **clap:** For robust CLI argument parsing.
- **serde & csv:** For data serialization and logging.

## Installation

Ensure you have Rust and Cargo installed. If not, you can install them from [rustup.rs](https://rustup.rs/).

```bash
# Clone the repository
git clone <repository-url>
cd btc_generate

# Build the project
cargo build --release
```

## Usage

You can run the generator using `cargo run`.

### Basic Usage

Generate 10 SegWit addresses (default) and save to `generated_addresses.csv`:

```bash
cargo run -- --count 10
```

### Advanced Usage

Specify the number of threads, address format, and custom output file:

```bash
cargo run -- --count 100 --threads 4 --format taproot --output my_keys.csv
```

#### Command Line Options

- `-c, --count <COUNT>`: Number of mnemonics to generate (default: 1).
- `-t, --threads <THREADS>`: Number of threads to use (default: number of CPUs).
- `-f, --format <FORMAT>`: Address format to derive (`legacy`, `segwit`, `taproot`).
- `-o, --output <OUTPUT>`: Output CSV file path (default: `generated_addresses.csv`).

## Security Warning

**CRITICAL:** This tool logs mnemonic phrases to a CSV file. While this is useful for research and testing, it is **highly insecure** for production funds. Anyone with access to the CSV file can steal the funds associated with those mnemonics. Use with extreme caution and never use generated mnemonics for storing significant value unless you understand the risks.

## License

MIT
