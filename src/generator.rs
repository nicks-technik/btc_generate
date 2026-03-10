use crate::mnemonic;
use crate::address;
use crate::logger::LogEntry;
use bitcoin::Network;
use rayon::prelude::*;

#[derive(Copy, Clone)]
pub enum AddressFormat {
    Legacy,
    SegWit,
    P2SHSegWit,
    Taproot,
}

use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};

pub fn generate_batch(count: usize, format: AddressFormat, network: Network, custom_path: Option<String>, passphrase: Option<String>) -> Vec<LogEntry> {
    let passphrase_str = passphrase.unwrap_or_default();
    
    let pb = ProgressBar::new(count as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .expect("Invalid progress bar template")
        .progress_chars("#>-"));

    (0..count)
        .into_par_iter()
        .progress_with(pb)
        .map(|_| {
            let mnemonic_str = mnemonic::generate_mnemonic();
            let master_key = address::derive_master_key(&mnemonic_str, network, &passphrase_str);
            
            let path_to_use = if let Some(ref path) = custom_path {
                path.clone()
            } else {
                let coin_type = if network == Network::Bitcoin { "0" } else { "1" };
                match format {
                    AddressFormat::Legacy => format!("m/44h/{}h/0h/0/0", coin_type),
                    AddressFormat::P2SHSegWit => format!("m/49h/{}h/0h/0/0", coin_type),
                    AddressFormat::SegWit => format!("m/84h/{}h/0h/0/0", coin_type),
                    AddressFormat::Taproot => format!("m/86h/{}h/0h/0/0", coin_type),
                }
            };

            let address = address::derive_address_by_path(&master_key, network, &path_to_use, format);

            let words: Vec<String> = mnemonic_str.split_whitespace().map(|s| s.to_string()).collect();
            
            LogEntry {
                log_id: uuid::Uuid::new_v4().to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
                network: format!("{:?}", network),
                address,
                derivation_path: path_to_use,
                passphrase: passphrase_str.clone(),
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
        let entries = generate_batch(count, AddressFormat::SegWit, Network::Bitcoin, None, None);
        assert_eq!(entries.len(), count);
    }
}
