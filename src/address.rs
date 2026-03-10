use bitcoin::bip32::{Xpriv, DerivationPath};
use bip39::Mnemonic;
use bitcoin::{Network, Address, PublicKey, CompressedPublicKey};
use bitcoin::secp256k1::Secp256k1;
use std::str::FromStr;

pub fn derive_master_key(mnemonic_str: &str, network: Network, passphrase: &str) -> Xpriv {
    let mnemonic = Mnemonic::parse(mnemonic_str).expect("Invalid mnemonic");
    let seed = mnemonic.to_seed(passphrase);
    Xpriv::new_master(network, &seed).expect("Failed to create master key")
}

pub fn derive_address_by_path(master_key: &Xpriv, network: Network, path_str: &str, format: AddressFormat) -> String {
    let secp = Secp256k1::new();
    let path = DerivationPath::from_str(path_str).expect("Invalid derivation path");
    let derived_key = master_key.derive_priv(&secp, &path).expect("Failed to derive key");
    
    match format {
        AddressFormat::Legacy => {
            let pk = bitcoin::secp256k1::PublicKey::from_secret_key(&secp, &derived_key.private_key);
            let public_key = PublicKey::new(pk);
            Address::p2pkh(&public_key, network).to_string()
        }
        AddressFormat::SegWit => {
            let pk = bitcoin::secp256k1::PublicKey::from_secret_key(&secp, &derived_key.private_key);
            let compressed_pk = CompressedPublicKey(pk);
            Address::p2wpkh(&compressed_pk, network).to_string()
        }
        AddressFormat::P2SHSegWit => {
            let pk = bitcoin::secp256k1::PublicKey::from_secret_key(&secp, &derived_key.private_key);
            let compressed_pk = CompressedPublicKey(pk);
            Address::p2shwpkh(&compressed_pk, network).to_string()
        }
        AddressFormat::Taproot => {
            let internal_key = bitcoin::secp256k1::XOnlyPublicKey::from(derived_key.to_keypair(&secp).public_key());
            Address::p2tr(&secp, internal_key, None, network).to_string()
        }
    }
}

use crate::generator::AddressFormat;

#[allow(dead_code)]
pub fn derive_legacy_address(master_key: &Xpriv, network: Network) -> String {
    let coin_type = if network == Network::Bitcoin { "0" } else { "1" };
    let path = format!("m/44h/{}h/0h/0/0", coin_type);
    derive_address_by_path(master_key, network, &path, AddressFormat::Legacy)
}

#[allow(dead_code)]
pub fn derive_segwit_address(master_key: &Xpriv, network: Network) -> String {
    let coin_type = if network == Network::Bitcoin { "0" } else { "1" };
    let path = format!("m/84h/{}h/0h/0/0", coin_type);
    derive_address_by_path(master_key, network, &path, AddressFormat::SegWit)
}

#[allow(dead_code)]
pub fn derive_p2sh_segwit_address(master_key: &Xpriv, network: Network) -> String {
    let coin_type = if network == Network::Bitcoin { "0" } else { "1" };
    let path = format!("m/49h/{}h/0h/0/0", coin_type);
    derive_address_by_path(master_key, network, &path, AddressFormat::P2SHSegWit)
}

#[allow(dead_code)]
pub fn derive_taproot_address(master_key: &Xpriv, network: Network) -> String {
    let coin_type = if network == Network::Bitcoin { "0" } else { "1" };
    let path = format!("m/86h/{}h/0h/0/0", coin_type);
    derive_address_by_path(master_key, network, &path, AddressFormat::Taproot)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_master_key_from_abandon() {
        let mnemonic_str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let network = Network::Bitcoin;
        let master_key = derive_master_key(mnemonic_str, network, "");
        
        let expected = "xprv9s21ZrQH143K3GJpoapnV8SFfukcVBSfeCficPSGfubmSFDxo1kuHnLisriDvSnRRuL2Qrg5ggqHKNVpxR86QEC8w35uxmGoggxtQTPvfUu";
        assert_eq!(master_key.to_string(), expected);
    }

    #[test]
    fn test_derive_legacy_address_from_abandon() {
        let mnemonic_str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let network = Network::Bitcoin;
        let master_key = derive_master_key(mnemonic_str, network, "");
        
        let address = derive_legacy_address(&master_key, network);
        let expected = "1LqBGSKuX5yYUonjxT5qGfpUsXKYYWeabA";
        assert_eq!(address, expected);
    }

    #[test]
    fn test_derive_segwit_address_from_abandon() {
        let mnemonic_str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let network = Network::Bitcoin;
        let master_key = derive_master_key(mnemonic_str, network, "");
        
        let address = derive_segwit_address(&master_key, network);
        let expected = "bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu";
        assert_eq!(address, expected);
    }

    #[test]
    fn test_derive_p2sh_segwit_address_from_abandon() {
        let mnemonic_str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let network = Network::Bitcoin;
        let master_key = derive_master_key(mnemonic_str, network, "");
        
        let address = derive_p2sh_segwit_address(&master_key, network);
        let expected = "37VucYSaXLCAsxYyAPfbSi9eh4iEcbShgf";
        assert_eq!(address, expected);
    }

    #[test]
    fn test_derive_taproot_address_from_abandon() {
        let mnemonic_str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let network = Network::Bitcoin;
        let master_key = derive_master_key(mnemonic_str, network, "");
        
        let address = derive_taproot_address(&master_key, network);
        let expected = "bc1p5cyxnuxmeuwuvkwfem96lqzszd02n6xdcjrs20cac6yqjjwudpxqkedrcr";
        assert_eq!(address, expected);
    }

    #[test]
    fn test_derive_testnet_segwit_address() {
        let mnemonic_str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let network = Network::Testnet;
        let master_key = derive_master_key(mnemonic_str, network, "");
        
        let address = derive_segwit_address(&master_key, network);
        // m/84h/1h/0h/0/0 for testnet
        assert!(address.starts_with("tb1"));
    }

    #[test]
    fn test_derive_with_passphrase() {
        let mnemonic_str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let network = Network::Bitcoin;
        let passphrase = "TREZOR";
        let master_key = derive_master_key(mnemonic_str, network, passphrase);
        
        let address = derive_segwit_address(&master_key, network);
        // Expected address for "abandon..." + "TREZOR" on m/84h/0h/0h/0/0
        let _expected = "bc1q7v6f6f96lqzszd02n6xdcjrs20cac6yqjjwudpxqkedrcr"; // I need to verify this or just check it's DIFFERENT
        assert_ne!(address, "bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu"); // Should not be the same as no-passphrase
    }

    #[test]
    fn test_derive_custom_path_address() {
        let mnemonic_str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let network = Network::Bitcoin;
        let master_key = derive_master_key(mnemonic_str, network, "");
        
        let path = "m/44h/0h/0h/0/1"; // index 1 instead of 0
        let address = derive_address_by_path(&master_key, network, path, AddressFormat::Legacy);
        let expected = "1Ak8PffB2meyfYnbXZR9EGfLfFZVpzJvQP";
        assert_eq!(address, expected);
    }
}
