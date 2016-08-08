# Rust Bloom Filter
## Introduction
A Bloom filter is a space-efficient, probabilistic set data structure. False positives are possible, but false negatives are not, making it useful as a negative cache.

A Bloom filter consists of a bit array of length _m_ and a number, _k_, of hash functions, _h_ (in practice the same hash function with a different seed, typically 0 to _k_ - 1). To insert a given key _K_, we perform _h(K)_ _k_ - 1 times. For each _h(K)_, we constrain it modulo _m_, which gives us a value in the range [0, m - 1]. (This technically introduces some bias, but in practice, it's rarely an issue.) We set this bit in the bit array for each iteration until we complete _k_ - 1 rounds. The result is a bit vector with up to _k_ bits set.

Lookup is almost identical to insertion, except we leave the bits alone and only check if each one is set. For each _k_, we again perform _h(K)_, each time checking if that corresponding bit is set. If it is, we continue; if not, we break out of the loop and return a falsey value, meaning the key is definitely not in the set. If we don't fail, however, we can say that the key is _probably_ in the set. We can't say it with certainty because it's possible that one or more other keys set the same bits as the given key. This probability changes based on the values for _m_ and _k_, and there is a way to work out the optimal values for a desired false positive rate.

## Features

## Future Work
So far, we can only use strings as hash keys. I would like to support hashing any generically encodable type.

`is_member` returns either `true` or `false`. I would like to wrap this in an `Option<T>`, or perhaps something a bit more complex, such as a type that can represent either "not in the set" or "probably in the set, with this probability of a false positive".
