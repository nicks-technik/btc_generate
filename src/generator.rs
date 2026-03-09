use crate::mnemonic;
use crate::address;
use crate::logger::LogEntry;
use bitcoin::Network;
use rayon::prelude::*;

pub enum AddressFormat {
    Legacy,
    SegWit,
    Taproot,
}

pub fn generate_batch(count: usize, format: AddressFormat, network: Network) -> Vec<LogEntry> {
    (0..count)
        .into_par_iter()
        .map(|_| {
            let mnemonic_str = mnemonic::generate_mnemonic();
            let master_key = address::derive_master_key(&mnemonic_str, network);
            
            let address = match format {
                AddressFormat::Legacy => address::derive_legacy_address(&master_key, network),
                AddressFormat::SegWit => address::derive_segwit_address(&master_key, network),
                AddressFormat::Taproot => address::derive_taproot_address(&master_key, network),
            };

            let words: Vec<String> = mnemonic_str.split_whitespace().map(|s| s.to_string()).collect();
            
            LogEntry {
                address,
                word1: words[0].clone(),
                word2: words[1].clone(),
                word3: words[2].clone(),
                word4: words[3].clone(),
                word5: words[4].clone(),
                word6: words[5].clone(),
                word7: words[6].clone(),
                word8: words[7].clone(),
                word9: words[8].clone(),
                word10: words[9].clone(),
                word11: words[10].clone(),
                word12: words[11].clone(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_batch_count() {
        let count = 10;
        let entries = generate_batch(count, AddressFormat::SegWit, Network::Bitcoin);
        assert_eq!(entries.len(), count);
    }
}
