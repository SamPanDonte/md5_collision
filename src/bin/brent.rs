use clap::Parser;
use md5_collision::{append_prefix, compute_function, Args};
fn main() -> Result<(), hex::FromHexError> {
    let prefix = hex::decode(Args::parse().prefix)?;

    let starting_point = [0x12];

    let mut power = 1u64;
    let mut lam = 1u64;

    let mut turtle = starting_point.to_vec();
    let mut hare = compute_function(&prefix, &turtle);

    while turtle != hare {
        if power == lam {
            turtle = hare.clone();
            power *= 2;
            lam = 0;
        }
        hare = compute_function(&prefix, &hare);
        lam += 1;
    }

    turtle = starting_point.to_vec();
    hare = starting_point.to_vec();

    for _ in 0..lam {
        hare = compute_function(&prefix, &hare);
    }

    loop {
        let new_turtle = compute_function(&prefix, &turtle);
        let new_hare = compute_function(&prefix, &hare);

        if new_turtle == new_hare {
            break;
        }

        turtle = new_turtle;
        hare = new_hare;
    }

    println!(
        "Found collision {} and {}",
        hex::encode(append_prefix(&prefix, &turtle)),
        hex::encode(append_prefix(&prefix, &hare))
    );
    Ok(())
}
