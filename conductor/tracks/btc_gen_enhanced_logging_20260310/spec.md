# Specification: Enhanced Logging & Multi-Network Support

## 1. Overview
This track enhances the BTC Mnemonic & Address Generator with support for different Bitcoin networks (Mainnet, Testnet), custom derivation paths, and a more detailed, header-driven CSV logging format.

## 2. Requirements

### 2.1 Functional Requirements
- **Multi-Network Support:**
    - CLI flag `--network` (`bitcoin`, `testnet`).
    - Addresses derived according to the selected network.
- **Custom Derivation Paths:**
    - CLI flag `--path` to override default BIP-44/84/86 paths.
- **Enhanced Logging (CSV):**
    - Include **Timestamp** (ISO 8601).
    - Include **Derivation Path** used for the address.
    - Include **Network** (Mainnet/Testnet).
    - Include **Log ID** (UUID or incremental).
    - Add **Headers** to the CSV file.

### 2.2 User Stories
- **Testnet Developer:** Generate mnemonics and addresses for Bitcoin Testnet to safely test wallet integrations.
- **Advanced User:** Use custom derivation paths for specialized wallet recovery or non-standard address derivation.
- **Auditor:** Use the detailed CSV logs with timestamps and path information for auditing generation events.

### 2.3 Non-Functional Requirements
- **Transparency:** The CSV log must be easy to read with headers.
- **Security:** Maintain the offline-first design.
- **Performance:** Ensure parallel processing handles the additional log metadata efficiently.

## 3. Technical Design

### 3.1 Tech Stack
- **Existing:** `bitcoin`, `bip39`, `rayon`, `clap`, `serde`, `csv`.
- **New:** `chrono` for timestamps.

### 3.2 Enhanced Logging Format (CSV with Headers)
`log_id,timestamp,network,address,derivation_path,word1,word2,word3,word4,word5,word6,word7,word8,word9,word10,word11,word12`

### 3.3 Updated CLI Interface
`btc_gen --count <NUMBER> --threads <NUMBER> --format <legacy|segwit|taproot> --network <bitcoin|testnet> --path <DERIVATION_PATH> --output <FILE>`
