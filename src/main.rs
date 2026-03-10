mod mnemonic;
mod address;
mod logger;
mod generator;

use bitcoin::Network;
use clap::{Parser, ValueEnum};
use generator::AddressFormat;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of mnemonics to generate
    #[arg(short, long, default_value_t = 1)]
    count: usize,

    /// Number of threads to use (default: number of CPUs)
    #[arg(short, long)]
    threads: Option<usize>,

    /// Address format to derive
    #[arg(short, long, value_enum, default_value_t = Format::Segwit)]
    format: Format,

    /// Network to use (bitcoin, testnet)
    #[arg(short, long, value_enum, default_value_t = Net::Bitcoin)]
    network: Net,

    /// Custom derivation path (overrides default for format)
    #[arg(short, long)]
    path: Option<String>,

    /// Optional BIP-39 passphrase (13th word)
    #[arg(short = 'P', long)]
    passphrase: Option<String>,

    /// Output CSV file path (default: <NETWORK>_generated_addresses.csv)
    #[arg(short, long)]
    output: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Format {
    Legacy,
    Segwit,
    P2shSegwit,
    Taproot,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Net {
    Bitcoin,
    Testnet,
}

impl std::fmt::Display for Net {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Net::Bitcoin => write!(f, "bitcoin"),
            Net::Testnet => write!(f, "testnet"),
        }
    }
}

impl From<Format> for AddressFormat {
    fn from(f: Format) -> Self {
        match f {
            Format::Legacy => AddressFormat::Legacy,
            Format::Segwit => AddressFormat::SegWit,
            Format::P2shSegwit => AddressFormat::P2SHSegWit,
            Format::Taproot => AddressFormat::Taproot,
        }
    }
}

impl From<Net> for Network {
    fn from(n: Net) -> Self {
        match n {
            Net::Bitcoin => Network::Bitcoin,
            Net::Testnet => Network::Testnet,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let output_path = args.output.unwrap_or_else(|| {
        format!("{}_generated_addresses.csv", args.network.to_string())
    });

    if let Some(t) = args.threads {
        rayon::ThreadPoolBuilder::new().num_threads(t).build_global()?;
    }

    println!("Generating {} {} addresses on {}...", args.count, match args.format {
        Format::Legacy => "Legacy",
        Format::Segwit => "SegWit",
        Format::P2shSegwit => "P2SH-SegWit",
        Format::Taproot => "Taproot",
    }, args.network);
    let network = args.network.into();
    let start_time = std::time::Instant::now();
    let entries = generator::generate_batch(args.count, args.format.into(), network, args.path, args.passphrase);
    let duration = start_time.elapsed();

    logger::write_to_csv(&output_path, &entries)?;

    println!("Successfully generated {} addresses in {:?}", args.count, duration);
    if args.count > 0 {
        let rate = args.count as f64 / duration.as_secs_f64();
        println!("Rate: {:.2} addresses/sec", rate);
    }
    println!("Entries saved to {}", output_path);

    Ok(())
}
