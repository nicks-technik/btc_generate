mod mnemonic;

fn main() {
    let mnemonic = mnemonic::generate_mnemonic();
    println!("Generated Mnemonic: {}", mnemonic);
}
