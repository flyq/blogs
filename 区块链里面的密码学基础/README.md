# 区块链里面的密码学基础


## `Contents`
- [区块链里面的密码学基础](#%e5%8c%ba%e5%9d%97%e9%93%be%e9%87%8c%e9%9d%a2%e7%9a%84%e5%af%86%e7%a0%81%e5%ad%a6%e5%9f%ba%e7%a1%80)
  - [`Contents`](#contents)
  - [substrate](#substrate)

## substrate

哈希函数：
* sha2
* keccak
* blake2
* xxhash

椭圆曲线密码：
* ed25519
* sr25519
* secp256k1

地址格式：
* SS58

primitives/core/

文档中也有所描述，链接：  
 https://docs.rs/substrate-primitives/1.0.0/substrate_primitives/  

SHA-2的实现：  
https://github.com/RustCrypto/hashes  
Substrate中提供了 sha2_256。

Keccak的实现：  
https://github.com/debris/tiny-keccak  
Substrate中提供了 keccak_256。

Blake2的实现  
 https://github.com/cesarb/blake2-rfc

 xxHash的实现：  
https://libraries.io/cargo/twox-hash
 https://github.com/Cyan4973/xxHash

Ed25519实现：  
https://github.com/dalek-cryptography/ed25519-dalek

更多的对于这部分的内容，可以参考波卡中的秘钥信息，链接  
 https://wiki.polkadot.network/docs/en/learn-keys 。

这里还需要提到的一个实现是bip39，增加了对sr25519的支持：  
https://github.com/paritytech/substrate-bip39  

Web3基金会在 Schnorrkel(https://github.com/w3f/schnorrkel)  这个库中也实现了这项改进，称其为Schnorrkel/Ristretto x25519，简称sr25519，并将其用于Substrate。这个库还支持其他的协议，例如分层确定性密钥派生（Hierarchical Deterministic Key Derivation，HDKD）, 多签（multi-signatures，MuSig）, VRF (verifiable random function，可验证随机函数)。