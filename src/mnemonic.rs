use bip39::{Mnemonic, Language};

pub fn generate_mnemonic() -> String {
    let mnemonic = Mnemonic::generate_in(Language::English, 12).expect("Failed to generate mnemonic");
    mnemonic.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_mnemonic_has_12_words() {
        let mnemonic = generate_mnemonic();
        let words: Vec<&str> = mnemonic.split_whitespace().collect();
        assert_eq!(words.len(), 12, "Mnemonic should have exactly 12 words, but got: {}", mnemonic);
    }

    #[test]
    fn test_generate_mnemonic_is_random() {
        let m1 = generate_mnemonic();
        let m2 = generate_mnemonic();
        assert_ne!(m1, m2, "Two generated mnemonics should not be the same");
    }
}
