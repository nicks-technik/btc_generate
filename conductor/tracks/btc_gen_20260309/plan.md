# Implementation Plan: BTC Mnemonic & Address Generator

## Phase 1: Project Scaffolding & Basic Mnemonic Generation

- [x] Task: Setup Rust project with `cargo init` (a27de9e)
    - [x] Initialize cargo project
    - [x] Configure `Cargo.toml` with basic metadata
- [x] Task: Add core dependencies (ac60469)
    - [x] Add `bitcoin`, `bip39`, `tiny-hdk`, `clap`, `serde`, `csv`, `rayon`
- [x] Task: Implement basic 12-word mnemonic generation (9eaf44e)
    - [x] Implement BIP-39 mnemonic generation logic
    - [x] Add unit tests for mnemonic generation (TDD)
- [ ] Task: Conductor - User Manual Verification 'Phase 1' (Protocol in workflow.md)

## Phase 2: Address Derivation & Multi-Format Support

- [ ] Task: Implement BIP-32/44 key derivation from mnemonic
    - [ ] Setup HD wallet derivation logic
- [ ] Task: Implement Legacy address derivation (P2PKH)
    - [ ] Add unit tests for Legacy derivation
- [ ] Task: Implement SegWit address derivation (P2WPKH)
    - [ ] Add unit tests for SegWit derivation
- [ ] Task: Implement Taproot address derivation (P2TR)
    - [ ] Add unit tests for Taproot derivation
- [ ] Task: Conductor - User Manual Verification 'Phase 2' (Protocol in workflow.md)

## Phase 3: Parallelization & CSV Logging

- [ ] Task: Implement CSV logging
    - [ ] Create struct for log entries with Serde support
    - [ ] Implement CSV writer logic
- [ ] Task: Integrate `rayon` for parallel batch generation
    - [ ] Parallelize the generation loop
    - [ ] Add unit tests for parallel processing
- [ ] Task: Build CLI interface with `clap`
    - [ ] Implement command-line arguments for count, threads, format, and output
- [ ] Task: Conductor - User Manual Verification 'Phase 3' (Protocol in workflow.md)
