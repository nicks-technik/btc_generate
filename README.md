# BTC Mnemonic & Address Generator

A high-performance, parallelized tool for generating Bitcoin mnemonic phrases (12 words) and deriving their corresponding Bitcoin addresses in various formats.
## Features

- **BIP-39 Mnemonic Generation:** Generates secure 12-word mnemonics.
- **Multi-Format Address Derivation:** Supports derivation of:
  - **Legacy (P2PKH):** Starts with `1` (Mainnet) or `m`/`n` (Testnet). Default path: `m/44h/0h/0h/0/0`.
  - **Nested SegWit (P2SH-P2WPKH):** Starts with `3` (Mainnet) or `2` (Testnet). Default path: `m/49h/0h/0h/0/0`.
  - **SegWit (P2WPKH):** Starts with `bc1q` (Mainnet) or `tb1q` (Testnet). Default path: `m/84h/0h/0h/0/0`.
  - **Taproot (P2TR):** Starts with `bc1p` (Mainnet) or `tb1p` (Testnet). Default path: `m/86h/0h/0h/0/0`.
- **BIP-39 Passphrase Support:** Optional 13th word for extra security using the `--passphrase` flag (Default: None).
- **Multi-Network Support:** Support for Bitcoin **Mainnet** (`bitcoin`) and **Testnet** (`testnet`).
- **Custom Derivation Paths:** Ability to specify any BIP-32 derivation path.
- **Parallel Processing:** Utilizes `rayon` for fast batch generation across multiple CPU cores.
- **CLI Progress Bar:** Real-time feedback and performance metrics during generation.

## Technical Reference

### Address Prefixes & Networks

| Format | Network | Prefix | Example |
| :--- | :--- | :--- | :--- |
| **Legacy** | Mainnet | `1` | `1LqBGSKuX...` |
| **Legacy** | Testnet | `m` or `n` | `mmp4hYp...` |
| **P2SH-SegWit** | Mainnet | `3` | `37VucYSaX...` |
| **P2SH-SegWit** | Testnet | `2` | `2Mv8QY...` |
| **SegWit** | Mainnet | `bc1q` | `bc1qcr8te4...` |
| **SegWit** | Testnet | `tb1q` | `tb1qcr8te4...` |
| **Taproot** | Mainnet | `bc1p` | `bc1p5cyxnu...` |
| **Taproot** | Testnet | `tb1p` | `tb1p5cyxnu...` |

### What is a Derivation Path?

A **Derivation Path** is a set of "directions" (BIP-32) that tells the tool how to calculate a specific private key from your 12-word mnemonic. For example, `m/84h/0h/0h/0/0`:
- `m`: The master root key.
- `84h`: The **Purpose** (84 for Native SegWit).
- `0h`: The **Coin Type** (0 for Bitcoin Mainnet, 1 for Testnet).
- `0h`: The **Account** (Sub-wallets).
- `0`: The **Change** (0 for receiving addresses).
- index: The address number.

The tool automatically picks the correct standard path based on your chosen `--format` and `--network`. Use the `--path` flag only if you need a non-standard derivation.

## Tech Stack
  - `log_id`: Unique identifier for each entry.
  - `timestamp`: ISO 8601 generation time.
  - `network`: Selected Bitcoin network.
  - `address`: Derived public address.
  - `derivation_path`: The path used for derivation.
  - `passphrase`: Optional BIP-39 passphrase.
  - `word1..word12`: The 12-word mnemonic phrase.

## Tech Stack

- **Rust:** Core language for performance and safety.
- **bitcoin-rust:** For Bitcoin address derivation and key management.
- **bip39:** For mnemonic phrase generation.
- **rayon:** For parallel processing.
- **clap:** For robust CLI argument parsing.
- **chrono:** For timestamp generation.
- **uuid:** For unique log identifiers.
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

Specify the number of threads, address format, network, custom path, and output file:

```bash
cargo run -- --count 100 --threads 4 --format taproot --network testnet --path "m/86h/1h/0h/0/1" --output my_keys.csv
```

#### Command Line Options

- `-c, --count <COUNT>`: Number of mnemonics to generate (default: 1).
- `-t, --threads <THREADS>`: Number of threads to use (default: number of CPUs).
- `-f, --format <FORMAT>`: Address format to derive (`legacy`, `segwit`, `p2sh-segwit`, `taproot`) (default: `segwit`).
- `-n, --network <NETWORK>`: Bitcoin network (`bitcoin`, `testnet`) (default: `bitcoin`).
- `-p, --path <PATH>`: Custom derivation path (e.g., `m/84h/0h/0h/0/0`).
- `-P, --passphrase <PASSPHRASE>`: Optional BIP-39 passphrase (default: None).
- `-o, --output <OUTPUT>`: Output CSV file path (default: `generated_addresses.csv`).

## Security Warning

**CRITICAL:** This tool logs mnemonic phrases to a CSV file. While this is useful for research and testing, it is **highly insecure** for production funds. Anyone with access to the CSV file can steal the funds associated with those mnemonics. Use with extreme caution and never use generated mnemonics for storing significant value unless you understand the risks.

## License

MIT
