extern crate byteorder;
extern crate bit_vec;

mod bloom_filter;
mod murmur3;

use bloom_filter::BloomFilter;

pub fn main() {
    let mut bf = BloomFilter::new(18, 3);
    println!("Inserting key: dogs");
    bf.insert("dogs");

    println!("Bloom filter after insertion: {:?}", bf.bv);

    println!("Checking membership: dogs");
    let result = bf.is_member("dogs");
    println!("Is member? {:?}", result);

    bf.insert("cats");
    bf.insert("orange");
    bf.insert("Canada");

    println!("Bloom filter after insertion: {:?}", bf.bv);
    let false_pos = bf.false_positive();
    println!("Probability of false positives: {:?}", false_pos);
    println!("Is orange in the set? {:?}", bf.is_member("orange"));
}
