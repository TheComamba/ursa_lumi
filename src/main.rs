#![warn(clippy::unwrap_used)]

use clap::Parser;
use generation_parameters::GenerationParameters;
use std::{fs::File, path::PathBuf};
use stellar_system::StellarSystem;

mod galactic_chunk;
mod generation_parameters;
mod stellar_system;

pub(crate) const MAX_ITEMS_IN_VECTOR: usize = 2147483648; //2^31

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::parse();
    let params = File::open(&args.params)?;
    let params: GenerationParameters = serde_json::from_reader(params)?;
    let stellar_system = StellarSystem::new(params);
    let out = File::create(&args.out)?;
    serde_json::to_writer(out, &stellar_system)?;
    Ok(())
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[arg(short, long, value_name = "FILE")]
    params: PathBuf,
    #[arg(short, long, value_name = "FILE")]
    out: PathBuf,
}

#[cfg(test)]
mod test {
    #[test]
    fn allocating_a_vector_with_max_items_does_not_panic() {
        let _ = Vec::<usize>::with_capacity(super::MAX_ITEMS_IN_VECTOR);
    }
}
