use std::path::PathBuf;

use clap::Parser;

mod generation_parameters;

fn main() {
    let args = Arguments::parse();
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[arg(short, long, value_name = "FILE")]
    params: PathBuf,
    #[arg(short, long, value_name = "FILE")]
    out: PathBuf,
}
