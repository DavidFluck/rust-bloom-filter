#![allow(dead_code)]
#![allow(unused_variables)]
extern crate byteorder;
use byteorder::{BigEndian, ReadBytesExt};
use std::num::Wrapping;
use std::io::Cursor;

fn main() {
    //murmur3_32("Hello, world!", 24, 25)
    //println!("{:b}", rotl(134217728 as u32, 2));
    // 01110000 01100001 01101110 01100100
    println!("{:?}", murmur3_32("panda", Wrapping(5), Wrapping(10)));
}

fn murmur3_32(key: &str, length: Wrapping<u32>, seed: Wrapping<u32>) -> u32 {
    let c1 = Wrapping::<u32>(0xCC9E2D51);
    let c2 = Wrapping::<u32>(0x1B873593);
    let r1 = 15;
    let r2 = 13;
    let m = Wrapping::<u32>(5);
    let n = Wrapping::<u32>(0xE6546B64);

    let mut hash = seed;

    /* Iterate four bytes at a time. */
    for chunk in key.as_bytes().chunks(4) {
        //let mut buf = Cursor::new(&chunk[..]);
        let mut final_bytes = vec![0; 4];
        let mut buf: Cursor<&[u8]>;
        let mut k: Wrapping<u32>;

        final_bytes.clone_from_slice(chunk);
        if chunk.len() < 4 {
            buf = Cursor::new(&final_bytes[..]);
        }
        else {
            buf = Cursor::new(&chunk[..]);
        }

        k =  Wrapping(buf.read_u32::<BigEndian>().unwrap());

        /* If chunk length is less than four, perform final loop step and break. */
        if chunk.len() < 4 {
            k = k * c1;
            k = rotl(k, r1);
            k = k * c2;

            hash = hash ^ k;
            break;
        }

        k = k * c1;
        k = rotl(k, r1);
        k = k * c2;

        hash = hash ^ k;
        hash = rotl(hash, r2);
        hash = hash * m + n;
    }

    hash = hash ^ length;
    hash = hash ^ (hash >> 16);
    hash = hash * Wrapping::<u32>(0x85EBCA6B);
    hash = hash ^ (hash >> 13);
    hash = hash * Wrapping::<u32>(0xC2B2AE35);
    hash = hash ^ (hash >> 16);

    hash.0
}

fn rotl(value: Wrapping<u32>, amount: usize) -> Wrapping<u32> {
    (value << amount) | (value >> (32 - amount))
}
