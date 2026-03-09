# Implementation Plan: BTC Mnemonic & Address Generator

## Phase 1: Project Scaffolding & Basic Mnemonic Generation [checkpoint: b256941]

- [x] Task: Setup Rust project with `cargo init` (a27de9e)
    - [x] Initialize cargo project
    - [x] Configure `Cargo.toml` with basic metadata
- [x] Task: Add core dependencies (ac60469)
    - [x] Add `bitcoin`, `bip39`, `tiny-hdk`, `clap`, `serde`, `csv`, `rayon`
- [x] Task: Implement basic 12-word mnemonic generation (9eaf44e)
    - [x] Implement BIP-39 mnemonic generation logic
    - [x] Add unit tests for mnemonic generation (TDD)
- [x] Task: Conductor - User Manual Verification 'Phase 1' (Protocol in workflow.md) (b256941)

## Phase 2: Address Derivation & Multi-Format Support [checkpoint: c6f51b1]

- [x] Task: Implement BIP-32/44 key derivation from mnemonic (f434f7a)
    - [x] Setup HD wallet derivation logic
- [x] Task: Implement Legacy address derivation (P2PKH) (f049d50)
    - [x] Add unit tests for Legacy derivation
- [x] Task: Implement SegWit address derivation (P2WPKH) (0437062)
    - [x] Add unit tests for SegWit derivation
- [x] Task: Implement Taproot address derivation (P2TR) (3e4732d)
    - [x] Add unit tests for Taproot derivation
- [x] Task: Conductor - User Manual Verification 'Phase 2' (Protocol in workflow.md) (c6f51b1)

## Phase 3: Parallelization & CSV Logging

- [x] Task: Implement CSV logging (c0bea06)
    - [x] Create struct for log entries with Serde support
    - [x] Implement CSV writer logic
- [x] Task: Integrate `rayon` for parallel batch generation (5b772f8)
    - [x] Parallelize the generation loop
    - [x] Add unit tests for parallel processing
- [ ] Task: Build CLI interface with `clap`
    - [ ] Implement command-line arguments for count, threads, format, and output
- [ ] Task: Conductor - User Manual Verification 'Phase 3' (Protocol in workflow.md)
