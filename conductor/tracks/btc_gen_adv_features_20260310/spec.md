# Specification: Advanced Security & UX Features

## 1. Overview
This track introduces support for BIP-39 passphrases (the "13th word"), Nested SegWit (P2SH-P2WPKH) address derivation, and a CLI progress bar to improve the user experience during large batch generations.

## 2. Requirements

### 2.1 Functional Requirements
- **BIP-39 Passphrase Support:**
    - CLI flag `--passphrase` to specify an optional passphrase.
    - Used during seed derivation from the 12-word mnemonic.
- **Nested SegWit Support:**
    - Add `p2sh-segwit` to the `--format` options.
    - Derives addresses according to BIP-49: `m/49h/0h/0h/0/0`.
- **CLI Progress Bar:**
    - Use `indicatif` to show a progress bar during `generate_batch`.
    - Display the current progress, estimated time remaining, and final performance metrics (e.g., "Generated 10,000 addresses in 2.5s (4,000 addr/s)").

### 2.2 User Stories
- **Security-Conscious User:** Use a passphrase to add an extra layer of security to my generated mnemonics.
- **Legacy Wallet User:** Generate Nested SegWit addresses for compatibility with older wallets that don't support Native SegWit (bc1).
- **Batch Generator:** See a progress bar when generating a large number of addresses so I know how long it will take.

### 2.3 Non-Functional Requirements
- **UX:** The progress bar should be smooth and not interfere with log outputs.
- **Accuracy:** The performance metrics should be calculated accurately based on wall-clock time.
- **Safety:** The passphrase should be handled securely (though still logged if that's the current behavior—I'll check the product vision).

## 3. Technical Design

### 3.1 Tech Stack
- **Existing:** `bitcoin`, `bip39`, `rayon`, `clap`, `chrono`, `uuid`, `serde`, `csv`.
- **New:** `indicatif` for progress bars.

### 3.2 Logging Format (CSV Update)
Include `passphrase` column in the CSV if provided.

### 3.3 Updated CLI Interface
`btc_gen --count <NUMBER> --format <legacy|segwit|p2sh-segwit|taproot> --passphrase <STRING> ...`
