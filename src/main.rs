fn main() {
    //murmur3_32("Hello, world!", 24, 25)
    //println!("{:b}", rotl(134217728 as u32, 2));
    rotl(23, 1);
}

fn murmur3_32(key: &str, length: u32, seed: u32) {
    let bytes: &[u8] = key.as_bytes();

    let c1 = 0xCC9E2D51;
    let c2 = 0x1B873593;
    let r1 = 15;
    let r2 = 13;
    let m = 5;
    let n = 0xE6546B64;

    let mut hash = seed;

    /* Iterate four bytes at a time. */
    for thing in bytes.chunks(4) {
        println!("{:?}", thing);
    }
}

fn rotl(value: u32, amount: u32) {
    (value << amount) | (value >> (32 - amount));
}
