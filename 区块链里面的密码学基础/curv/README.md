# Curv

参考：https://github.com/KZen-networks/curv

## Contents
- [Curv](#curv)
  - [Contents](#contents)
- [1. 依赖库](#1-%e4%be%9d%e8%b5%96%e5%ba%93)
  - [1.1 rand](#11-rand)
  - [1.2 serde](#12-serde)
  - [1.3 serde_derive](#13-serdederive)
  - [1.4 zeroize](#14-zeroize)
  - [1.5 sha3](#15-sha3)
  - [1.6 sha2](#16-sha2)
  - [1.7 hmac](#17-hmac)
  - [1.8 digest](#18-digest)
  - [1.9 hex](#19-hex)
  - [1.10 blake2b_simd](#110-blake2bsimd)
  - [1.11 rust-crypto](#111-rust-crypto)
  - [1.12 merkle-sha3](#112-merkle-sha3)
  - [1.13 bls12_381](#113-bls12381)
  - [1.14 sapling-crypto](#114-sapling-crypto)
  - [1.15 pairing](#115-pairing)
  - [1.16 rust-gmp](#116-rust-gmp)
  - [1.17 secp256k1](#117-secp256k1)
  - [1.18 curve25519-dalek](#118-curve25519-dalek)
  - [1.19 cryptoxide](#119-cryptoxide)
  - [[dev-dependencies] bincode](#dev-dependencies-bincode)
  - [[dev-dependencies] serde_json](#dev-dependencies-serdejson)

# 1. 依赖库

## 1.1 rand
[docs rand](https://docs.rs/rand/0.7.3/rand/)

```rust
use rand::prelude::*;

if rand::random() { // generates a boolean
    // Try printing a random unicode code point (probably a bad idea)!
    println!("char: {}", rand::random::<char>());
}

let mut rng = rand::thread_rng();
let y: f64 = rng.gen(); // generates a float between 0 and 1

let mut nums: Vec<i32> = (1..100).collect();
nums.shuffle(&mut rng);
```
## 1.2 serde

## 1.3 serde_derive

## 1.4 zeroize

## 1.5 sha3

## 1.6 sha2

## 1.7 hmac

## 1.8 digest

## 1.9 hex

## 1.10 blake2b_simd

## 1.11 rust-crypto

## 1.12 merkle-sha3

## 1.13 bls12_381

## 1.14 sapling-crypto

https://github.com/omershlo/librustzcash

## 1.15 pairing

https://github.com/omershlo/librustzcash.git

## 1.16 rust-gmp

https://github.com/KZen-networks/rust-gmp

## 1.17 secp256k1

## 1.18 curve25519-dalek

## 1.19 cryptoxide

## [dev-dependencies] bincode

## [dev-dependencies] serde_json