# Product Guidelines: BTC Mnemonic & Address Generator

## UX & Interaction Principles
- **CLI-First Design:** The tool should be intuitive to use from the command line, with clear help messages and logical argument structures.
- **Informative Feedback:** Provide real-time feedback during the generation and derivation process (e.g., "Mnemonic generated...", "Address derived...").
- **Error Handling:** Gracefully handle errors, such as invalid keyword lists or derivation failures, with descriptive error messages.
- **Minimalism:** Avoid unnecessary output; focus on the primary task of generation and logging.

## Prose & Communication Style
- **Technical & Direct:** Use clear, technical language that is easy for developers and researchers to understand.
- **Consistency:** Ensure consistent terminology (e.g., always use "mnemonic phrase" instead of "seed phrase" or "recovery phrase" interchangeably).
- **Documentation:** Provide a clear README with setup instructions and examples of how to run the tool.

## Technical & Security Guidelines
- **Offline Security:** Emphasize that the tool is designed for offline use. Any network-related libraries should be avoided.
- **Data Privacy:** Log files should be treated as sensitive data. The tool should include a warning about the security implications of logging mnemonics.
- **Determinism:** Ensure that for a given mnemonic and derivation path, the derived address is always the same (standard BIP-32/44 behavior).
- **Code Clarity:** Prioritize readable and well-commented code, especially in the sensitive cryptographic sections.
