# Specification: BTC Mnemonic & Address Generator

## 1. Overview
This track implements a high-performance Rust CLI tool to generate Bitcoin mnemonic phrases (BIP-39) and derive their corresponding addresses (Legacy, SegWit, Taproot) using parallel processing.

## 2. Requirements

### 2.1 Functional Requirements
- **Mnemonic Generation:** Generate standard 12-word BIP-39 mnemonics.
- **Address Derivation:** 
    - Support Legacy (P2PKH).
    - Support SegWit (P2WPKH).
    - Support Taproot (P2TR).
- **Deterministic Derivation:** Ensure consistent address derivation for a given mnemonic and path.
- **Parallel Processing:** Utilize multiple CPU cores to generate and derive addresses in batches.
- **Structured Logging:** Save results to a CSV file containing the address and all 12 words.

### 2.2 User Stories
- **High-Volume Researcher:** Generate thousands of addresses quickly to analyze patterns.
- **CLI Developer:** Use the tool via command-line arguments for automated workflows.

### 2.3 Non-Functional Requirements
- **Performance:** Leverages `rayon` for multi-core scaling.
- **Security:** Offline-first design; no network dependencies.
- **Quality:** TDD approach with >80% code coverage.

## 3. Technical Design

### 3.1 Tech Stack
- **Language:** Rust
- **Parallelism:** `rayon`
- **Bitcoin Libs:** `bitcoin`, `bip39`, `tiny-hdk`
- **CLI:** `clap`
- **Serialization:** `serde`, `csv`

### 3.2 Logging Format (CSV)
`address,word1,word2,word3,word4,word5,word6,word7,word8,word9,word10,word11,word12`

### 3.3 CLI Interface
`btc_gen --count <NUMBER> --threads <NUMBER> --format <legacy|segwit|taproot> --output <FILE>`
