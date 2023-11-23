use clap::{Parser, ValueEnum};
use hex::{encode, FromHexError};
use md5_collision::{cycle, naive};

#[derive(Clone, ValueEnum)]
enum Algorithm {
    Brent,
    Floyd,
    Naive,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Algorithm to use
    pub algorithm: Algorithm,
    /// Hex encoded prefix for hash algorithm
    pub prefix: String,
    /// Starting point (only for Brent's and Floyd's algorithm)
    #[clap(long, short, default_value = "")]
    pub starting_point: String,
}

fn main() -> Result<(), FromHexError> {
    let args = Args::parse();
    let starting_point = hex::decode(args.starting_point)?;
    let prefix = hex::decode(args.prefix)?;

    let (first, second) = match args.algorithm {
        Algorithm::Brent => cycle::brent::compute_collision(&prefix, &starting_point),
        Algorithm::Floyd => cycle::floyd::compute_collision(&prefix, &starting_point),
        Algorithm::Naive => naive::compute_collision(&prefix),
    };

    println!("Found collision {} and {}", encode(first), encode(second));

    Ok(())
}
