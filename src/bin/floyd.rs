use clap::Parser;
use md5_collision::{append_prefix, compute_function, Args};

fn main() -> Result<(), hex::FromHexError> {
    let prefix = hex::decode(Args::parse().prefix)?;

    let starting_point = [];
    let mut turtle = compute_function(&prefix, &starting_point);
    let mut hare = compute_function(&prefix, &turtle);

    loop {
        turtle = compute_function(&prefix, &turtle);
        hare = compute_function(&prefix, &compute_function(&prefix, &hare));

        if turtle == hare {
            break;
        }
    }

    turtle = starting_point.to_vec();
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
