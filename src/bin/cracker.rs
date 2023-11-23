use ahash::{HashMap, HashMapExt};
use md5_collision::{custom_md5, Digest};

fn main() {
    let prefix = vec![0x45, 0x26, 0x98];
    let mut hashes: HashMap<Digest, Vec<u8>> = HashMap::new();

    let mut data = 0u128;

    let (first, second) = loop {
        let mut message = prefix.clone();

        let mut found = false;
        for byte in data.to_be_bytes() {
            if byte != 0 || found {
                found = true;
                message.push(byte);
            }
        }

        let digest = custom_md5(&message);

        if let Some(value) = hashes.remove(&digest) {
            break (message, value);
        } else {
            hashes.insert(digest, message);
        }

        data += 4;

        if data % 1000000 == 0 {
            println!("{} mln", data / 1000000);
        }
    };

    println!("Found it! {first:02X?} and {second:02X?}");
}
