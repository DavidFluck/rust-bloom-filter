use bit_vec::BitVec;
use std;
use murmur3::murmur3_32;

/* m: the number of bits in the bit vector
 * k: the number of hash functions to use (practically, a seed for the Murmur hash)
 * bv: the bit vector which will be checked to determine set membership
 * cardinality: the number of elements we've inserted so far
 */
pub struct BloomFilter {
    pub m: u32,
    pub k: u32,
    pub bv: BitVec,
    pub cardinality: u32,
}

impl BloomFilter {
    pub fn new(m: u32, k: u32) -> BloomFilter {
        let bv = BitVec::from_elem(m as usize, false);
        BloomFilter { m: m, k: k, bv: bv, cardinality: 0 }
    }

    pub fn insert(&mut self, item: &str) {
        for x in 0..self.k {
            let hash = murmur3_32(item, x);
            let bloom_index = hash % self.m;

            self.bv.set(bloom_index as usize, true);
        }

        self.cardinality += 1;
    }

    pub fn is_member(&self, item: &str) -> bool {
        let mut found = true;
        for x in 0..self.k {
            let hash = murmur3_32(item, x);

            // We need to get our hash value from [0, 2^32 - 1] to [0, m - 1]. This is slightly biased because we're constraining the range with a modulus, but according to some random paper I found but can't locate at the moment, this isn't an issue in practice.
            let bloom_index = hash % self.m;

            // I'm not convinced there isn't a more elegant way to do this.
            found = match self.bv.get(bloom_index as usize) {
                Some(false) => false,
                _ => continue,
            };
        }

        found
    }

    // Return an f32 representing the probability that a false positive result will be returned for a random key.
    pub fn false_positive(&self) -> f32 {
        let n = self.cardinality as f32;
        let m = self.m as f32;
        let k = self.k as f32;
        (1.0 - std::f32::consts::E.powf(-k*n/m)).powf(k)
    }
}
