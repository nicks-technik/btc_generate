use bitcoin::bip32::Xpriv;
use bip39::Mnemonic;
use bitcoin::Network;

pub fn derive_master_key(mnemonic_str: &str, network: Network) -> Xpriv {
    let mnemonic = Mnemonic::parse(mnemonic_str).expect("Invalid mnemonic");
    let seed = mnemonic.to_seed("");
    Xpriv::new_master(network, &seed).expect("Failed to create master key")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_master_key_from_abandon() {
        let mnemonic_str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let network = Network::Bitcoin;
        let master_key = derive_master_key(mnemonic_str, network);
        
        // Correct expected master xpriv for 'abandon...' on mainnet (BIP39 standard)
        let expected = "xprv9s21ZrQH143K3GJpoapnV8SFfukcVBSfeCficPSGfubmSFDxo1kuHnLisriDvSnRRuL2Qrg5ggqHKNVpxR86QEC8w35uxmGoggxtQTPvfUu";
        assert_eq!(master_key.to_string(), expected);
    }
}
