use crate::compute_function;
use crate::cycle::converge;

/// Computes a collision for a given prefix and starting point using Brent's algorithm.
pub fn compute_collision(prefix: &[u8], starting_point: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let mut power = 1u64;
    let mut lam = 1u64;

    let mut turtle = starting_point.to_vec();
    let mut hare = compute_function(prefix, &turtle);

    while turtle != hare {
        if power == lam {
            turtle = hare.clone();
            power *= 2;
            lam = 0;
        }
        hare = compute_function(prefix, &hare);
        lam += 1;
    }

    turtle = starting_point.to_vec();
    hare = starting_point.to_vec();

    for _ in 0..lam {
        hare = compute_function(prefix, &hare);
    }

    converge(turtle, hare, prefix)
}
