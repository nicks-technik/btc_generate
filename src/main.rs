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

    /// Output CSV file path
    #[arg(short, long, default_value = "generated_addresses.csv")]
    output: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Format {
    Legacy,
    Segwit,
    Taproot,
}

impl From<Format> for AddressFormat {
    fn from(f: Format) -> Self {
        match f {
            Format::Legacy => AddressFormat::Legacy,
            Format::Segwit => AddressFormat::SegWit,
            Format::Taproot => AddressFormat::Taproot,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if let Some(t) = args.threads {
        rayon::ThreadPoolBuilder::new().num_threads(t).build_global()?;
    }

    println!("Generating {} {} addresses...", args.count, match args.format {
        Format::Legacy => "Legacy",
        Format::Segwit => "SegWit",
        Format::Taproot => "Taproot",
    });

    let network = Network::Bitcoin;
    let entries = generator::generate_batch(args.count, args.format.into(), network);

    logger::write_to_csv(&args.output, &entries)?;

    println!("Successfully wrote entries to {}", args.output);

    Ok(())
}
