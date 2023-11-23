use crate::compute_function;

pub mod brent;
pub mod floyd;

fn converge(mut turtle: Vec<u8>, mut hare: Vec<u8>, prefix: &[u8]) -> (Vec<u8>, Vec<u8>) {
    loop {
        let new_turtle = compute_function(prefix, &turtle);
        let new_hare = compute_function(prefix, &hare);

        if new_turtle == new_hare {
            break (turtle, hare);
        }

        turtle = new_turtle;
        hare = new_hare;
    }
}
