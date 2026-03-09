mod mnemonic;
mod address;
mod logger;

use bitcoin::Network;

fn main() {
    let mnemonic_str = mnemonic::generate_mnemonic();
    println!("Generated Mnemonic: {}", mnemonic_str);

    let network = Network::Bitcoin;
    let master_key = address::derive_master_key(&mnemonic_str, network);

    let legacy = address::derive_legacy_address(&master_key, network);
    let segwit = address::derive_segwit_address(&master_key, network);
    let taproot = address::derive_taproot_address(&master_key, network);

    println!("Legacy Address:  {}", legacy);
    println!("SegWit Address:  {}", segwit);
    println!("Taproot Address: {}", taproot);
}
