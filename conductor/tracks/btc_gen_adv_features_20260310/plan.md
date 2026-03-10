# Implementation Plan: Advanced Security & UX Features

## Phase 1: Nested SegWit (BIP-49) Support

- [x] Task: Implement Nested SegWit (P2SH-P2WPKH) derivation in `address.rs`
- [x] Task: Update unit tests for BIP-49 derivation
- [x] Task: Update CLI options to include `p2sh-segwit`

## Phase 2: BIP-39 Passphrase Support

- [x] Task: Update `generate_batch` and `derive_master_key` to support an optional passphrase
- [x] Task: Update `LogEntry` and CSV logic to include the passphrase column
- [x] Task: Implement CLI flag `--passphrase`
- [x] Task: Update unit tests for passphrase-based derivation

## Phase 3: UX Enhancements (Progress Bar)

- [x] Task: Add `indicatif` to `tech-stack.md` and `Cargo.toml`
- [x] Task: Integrate `ProgressBar` into `generate_batch`
- [x] Task: Implement final performance summary in `main.rs`
- [x] Task: Conductor - User Manual Verification 'Advanced Features'
