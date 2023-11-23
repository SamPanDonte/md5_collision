use clap::Parser;
use hex::{decode, encode, FromHexError};
use md5_collision::custom_md5;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Value to hash in hex
    pub value: String,
}

fn main() -> Result<(), FromHexError> {
    let data = decode(Args::parse().value)?;
    println!("{}", encode(custom_md5(data).as_bytes()));
    Ok(())
}
