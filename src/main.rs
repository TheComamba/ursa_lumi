#![warn(clippy::unwrap_used)]

use clap::Parser;
use generation_parameters::GenerationParameters;
use std::{fs::File, path::PathBuf};
use stellar_system::StellarSystem;

mod galactic_chunk;
mod generation_parameters;
mod mass_density;
mod population;
mod stellar_system;

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
mod tests {
    #[macro_export]
    macro_rules! assert_diff {
        ($x:expr, $y:expr, $d:expr $(, $($arg:tt)+)?) => {
            let x_f64: f64 = $x as f64;
            let y_f64: f64 = $y as f64;
            let d_f64: f64 = $d as f64;

            assert!((x_f64 - y_f64).abs() < d_f64, $($($arg)+)?);
        }
    }

    #[macro_export]
    macro_rules! assert_ratio {
        ($x:expr, $y:expr, $max_dev:expr $(, $($arg:tt)+)?) => {
            let x_f64: f64 = $x as f64;
            let y_f64: f64 = $y as f64;
            let dev_f64: f64 = $max_dev as f64;

            let ratio = if x_f64.abs() > y_f64.abs() {
                x_f64 / y_f64
            } else {
                y_f64 / x_f64
            };
            assert!((ratio.abs()-1.).abs() <= dev_f64, $($($arg)+)?);
        }
    }
}
