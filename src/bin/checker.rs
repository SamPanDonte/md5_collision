use md5_collision::custom_md5;

// Found it! [45, 26, 98, 14, C4, D2, C2] and [45, 26, 98, E, 7B, 45, 1E]
// Found it! [45, 26, 98, 46, 95, F8, 18, 3E, FF, D9] and [45, 26, 98, 62, D6, 56, 16, 80, 1A, B9]
fn main() {
    println!(
        "{:?}",
        // custom_md5([0x45, 0x26, 0x98, 0x14, 0xC4, 0xD2, 0xC2])
        custom_md5([0x45, 0x26, 0x98, 0x46, 0x95, 0xF8, 0x18, 0x3E, 0xFF, 0xD9])
    );
    println!(
        "{:?}",
        // custom_md5([0x45, 0x26, 0x98, 0x0E, 0x7B, 0x45, 0x1E])
        custom_md5([0x45, 0x26, 0x98, 0x62, 0xD6, 0x56, 0x16, 0x80, 0x1A, 0xB9])
    );
}