use byteorder::{LittleEndian, ReadBytesExt};
use std::num::Wrapping;
use std::io::Cursor;

pub fn murmur3_32(key: &str, seed: u32) -> u32 {
    let c1 = Wrapping::<u32>(0xCC9E2D51);
    let c2 = Wrapping::<u32>(0x1B873593);
    let r1 = 15;
    let r2 = 13;
    let m = Wrapping::<u32>(5);
    let n = Wrapping::<u32>(0xE6546B64);
    let length = Wrapping(key.len() as u32);

    let mut hash = Wrapping(seed);

    /* Iterate four bytes at a time. */
    for chunk in key.as_bytes().chunks(4) {
        let mut final_bytes: Vec<u8>;
        let mut buf = Cursor::new(&chunk[..]);
        let mut k: Wrapping<u32>;

        /* If we're on the last few bytes, we have to convert them to a vector, pad it, then take a slice. */
        if chunk.len() < 4 {
            final_bytes = chunk.to_vec();
            while final_bytes.len() < 4 {
                final_bytes.push(0);
            }

            buf = Cursor::new(&final_bytes[..]);
            k = Wrapping(buf.read_u32::<LittleEndian>().unwrap());

            k = k * c1;
            k = rotl(k, r1);
            k = k * c2;

            hash = hash ^ k;
            break;
        }

        k =  Wrapping(buf.read_u32::<LittleEndian>().unwrap());

        k = k * c1;
        k = rotl(k, r1);
        k = k * c2;

        hash = hash ^ k;
        hash = rotl(hash, r2);
        hash = hash * m + n;
    }

    /* Finalization. */
    hash = hash ^ length;
    hash = hash ^ (hash >> 16);
    hash = hash * Wrapping::<u32>(0x85EBCA6B);
    hash = hash ^ (hash >> 13);
    hash = hash * Wrapping::<u32>(0xC2B2AE35);
    hash = hash ^ (hash >> 16);

    hash.0
}

// Rotate left for a u32 with wrapping semantics.
pub fn rotl(value: Wrapping<u32>, amount: usize) -> Wrapping<u32> {
    (value << amount) | (value >> (32 - amount))
}
