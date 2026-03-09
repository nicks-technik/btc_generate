use bitcoin::bip32::{Xpriv, DerivationPath};
use bip39::Mnemonic;
use bitcoin::{Network, Address, PublicKey, CompressedPublicKey};
use bitcoin::secp256k1::Secp256k1;
use std::str::FromStr;

pub fn derive_master_key(mnemonic_str: &str, network: Network) -> Xpriv {
    let mnemonic = Mnemonic::parse(mnemonic_str).expect("Invalid mnemonic");
    let seed = mnemonic.to_seed("");
    Xpriv::new_master(network, &seed).expect("Failed to create master key")
}

pub fn derive_legacy_address(master_key: &Xpriv, network: Network) -> String {
    let secp = Secp256k1::new();
    let path = DerivationPath::from_str("m/44'/0'/0'/0/0").expect("Invalid derivation path");
    let derived_key = master_key.derive_priv(&secp, &path).expect("Failed to derive key");
    let pk = bitcoin::secp256k1::PublicKey::from_secret_key(&secp, &derived_key.private_key);
    let public_key = PublicKey::new(pk);
    let address = Address::p2pkh(&public_key, network);
    address.to_string()
}

pub fn derive_segwit_address(master_key: &Xpriv, network: Network) -> String {
    let secp = Secp256k1::new();
    let path = DerivationPath::from_str("m/84'/0'/0'/0/0").expect("Invalid derivation path");
    let derived_key = master_key.derive_priv(&secp, &path).expect("Failed to derive key");
    let pk = bitcoin::secp256k1::PublicKey::from_secret_key(&secp, &derived_key.private_key);
    let compressed_pk = CompressedPublicKey(pk);
    let address = Address::p2wpkh(&compressed_pk, network);
    address.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_master_key_from_abandon() {
        let mnemonic_str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let network = Network::Bitcoin;
        let master_key = derive_master_key(mnemonic_str, network);
        
        let expected = "xprv9s21ZrQH143K3GJpoapnV8SFfukcVBSfeCficPSGfubmSFDxo1kuHnLisriDvSnRRuL2Qrg5ggqHKNVpxR86QEC8w35uxmGoggxtQTPvfUu";
        assert_eq!(master_key.to_string(), expected);
    }

    #[test]
    fn test_derive_legacy_address_from_abandon() {
        let mnemonic_str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let network = Network::Bitcoin;
        let master_key = derive_master_key(mnemonic_str, network);
        
        let address = derive_legacy_address(&master_key, network);
        let expected = "1LqBGSKuX5yYUonjxT5qGfpUsXKYYWeabA";
        assert_eq!(address, expected);
    }

    #[test]
    fn test_derive_segwit_address_from_abandon() {
        let mnemonic_str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let network = Network::Bitcoin;
        let master_key = derive_master_key(mnemonic_str, network);
        
        let address = derive_segwit_address(&master_key, network);
        let expected = "bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu";
        assert_eq!(address, expected);
    }
}
