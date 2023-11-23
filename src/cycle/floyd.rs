use crate::compute_function;
use crate::cycle::converge;

/// Computes a collision for a given prefix and starting point using Floyd's algorithm.
pub fn compute_collision(prefix: &[u8], starting_point: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let mut turtle = compute_function(prefix, starting_point);
    let mut hare = compute_function(prefix, &turtle);

    loop {
        turtle = compute_function(prefix, &turtle);
        hare = compute_function(prefix, &compute_function(prefix, &hare));

        if turtle == hare {
            break;
        }
    }

    converge(starting_point.to_vec(), hare, prefix)
}
