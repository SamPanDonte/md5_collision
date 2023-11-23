use crate::{append_prefix, custom_md5, Digest};
use ahash::{HashMap, HashMapExt};

fn bytes(data: u128) -> usize {
    if data > u64::MAX as u128 {
        0
    } else if data > u32::MAX as u128 {
        8
    } else {
        12
    }
}

/// Computes a collision for a given prefix using naive algorithm.
pub fn compute_collision(prefix: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let mut hashes: HashMap<Digest, Vec<u8>> = HashMap::new();
    let mut data = 0u128;

    let (first, second) = loop {
        let message = append_prefix(prefix, &data.to_be_bytes()[bytes(data)..]);
        let digest = custom_md5(&message);

        if let Some(value) = hashes.remove(&digest) {
            break (message, value);
        } else {
            hashes.insert(digest, message);
        }

        data += 1;
    };

    (first, second)
}
