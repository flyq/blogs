# Substrate API Client

- [Substrate API Client](#substrate-api-client)
- [背景](#背景)
- [使用体验](#使用体验)
	- [读取信息](#读取信息)
	- [写入信息](#写入信息)
- [测试账户怎么生成的](#测试账户怎么生成的)
- [sp_keyring 库](#sp_keyring-库)
- [sp_core 库的 Pair 组成](#sp_core-库的-pair-组成)
	- [schnorrkel 库的 Keypair](#schnorrkel-库的-keypair)
	- [schnorrkel 库的 PublicKey](#schnorrkel-库的-publickey)
		- [curve25519-dalek 库的 CompressedRistretto RistrettoPoint](#curve25519-dalek-库的-compressedristretto-ristrettopoint)
	- [schnorrkel 库 的 SecretKey](#schnorrkel-库-的-secretkey)
		- [curve25510-dalek 库的 Scalar](#curve25510-dalek-库的-scalar)
- [sp_core 库的 Pair::from_string()](#sp_core-库的-pairfrom_string)
- [sp_core 库的 Pair::sign()](#sp_core-库的-pairsign)

# 背景

最近在看有哪些基于Rust版本的SDK，能够和 Substrate 进行交互。发现有两个，一个是官方的 [substrate-subxt](https://github.com/paritytech/substrate-subxt), 一个是社区某个团队做的 [substrate-api-client](https://github.com/scs/substrate-api-client)。

其中 substrate-subxt 有 8340 行左右 rust 源码，substrate-api-client 比较新，有 3377 行左右 rust 源码。因此尝试性使用了 substrate-api-client。

# 使用体验
## 读取信息
substrate-api-client 做得比较方便，比如这样的几行就能够读取链上信息。

```rust
    let url = String::from("ws://127.0.0.1:9944");
    let mut api = Api::new(url);

    // get some plain storage value
    let result: u128 = api
        .get_storage_value("Balances", "TotalIssuance", None)
        .unwrap();
    println!("[+] TotalIssuance is {}", result);
```

## 写入信息

```rust 
    let url = String::from("ws://127.0.0.1:9944");
    let from = AccountKeyring::Alice.pair();
    let api = Api::new(url).set_signer(from);

    // set the recipient
    let to = AccountKeyring::Bob.to_account_id();

    // call Balances::transfer
    // the names are given as strings
    #[allow(clippy::redundant_clone)]
    let xt: UncheckedExtrinsicV4<_> =
        compose_extrinsic!(api.clone(), "Balances", "transfer", to, Compact(42 as u128));

    println!("[+] Composed Extrinsic:\n {:?}\n", xt);

    // send and watch extrinsic until finalized
    let tx_hash = api
        .send_extrinsic(xt.hex_encode(), XtStatus::Finalized)
        .unwrap();
    println!("[+] Transaction got finalized. Hash: {:?}", tx_hash);
```

# 测试账户怎么生成的

像 from 账户，直接通过 `AccountKeyring::Alice.pair()` 得到。而且，substrate 链里面确实一启动就有了一堆测试账户。账户难道不是通过随机数生成私钥，再通过私钥生成公钥的吗？ 

PS：关于账户以及 ECDSA 的详细介绍，参考这四篇文章：[elliptic-curve-cryptography-a-gentle-introduction](https://andrea.corbellini.name/2015/05/17/elliptic-curve-cryptography-a-gentle-introduction/)

所以，一步步深入下去看看到底这些测试账户是怎么生成的。

# sp_keyring 库

而 AccountKeyring 来源于 sp_keyring 库： `pub use sr25519::Keyring as AccountKeyring;`，因此查看 `keyring/src/sr25519.rs` 的代码：
```rust
/// Set of test accounts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter)]
pub enum Keyring {
	Alice,
	Bob,
	Charlie,
	Dave,
	Eve,
	Ferdie,
	One,
	Two,
}
```
因此测试账户只是enum。然后 Keyring 有以下代码：
```rust
impl Keyring {
    
    ···
    pub fn sign(self, msg: &[u8]) -> Signature {
		Pair::from(self).sign(msg)
	}

	pub fn pair(self) -> Pair {
		Pair::from_string(&format!("//{}", <&'static str>::from(self)), None)
			.expect("static values are known good; qed")
	}

	pub fn to_seed(self) -> String {
		format!("//{}", self)
    }
    ···

}
```
可以看出，像签名(sign)，生成公私钥对(pair)等大部分功能是通过 Pair 实现。其中 `use sp_core::{sr25519::{Pair, Public, Signature}, Pair as PairT, Public as PublicT, H256};`，可以看出，Pair 是 sp_core 里面的 sr25519::Pair。

先看 sign 方法：
```rust
    pub fn sign(self, msg: &[u8]) -> Signature {
		Pair::from(self).sign(msg)
	}
```
这里从 Keyring enum 类型，转换到 Pair 类型/struct，从下文中可以看到：
```rust
impl From<Keyring> for Pair {
	fn from(k: Keyring) -> Self {
		k.pair()
	}
}
```
最终还是调用 Keyring::pair() 方法。并且得到 Pair struct 之后，再调用 sign 方法。

接着看 Keyring::pair 方法：
```rust
	pub fn pair(self) -> Pair {
		Pair::from_string(&format!("//{}", <&'static str>::from(self)), None)
			.expect("static values are known good; qed")
    }
```
这个就调用 Pair struct 的 from_string 方法了。需要看 sp_core 库里 Pair 的实现了。并且还需要关注这两个方法的实现：一个是 `Pair::from_string()`；一个是 `Pair::sign()`。

因此接下我们需要完成三个任务：一个是 struct Pair 的组成；一个是 `Pair::from_string()`；一个是 `Pair::sign()`。

# sp_core 库的 Pair 组成

查看 sp_core 库里面的 Pair：
发现有两个 Pair：

一个在 `core/src/sr25519.rs` 文件中，就是 sp_core::sr25519::Pair，这个 Pair 是一个 struct。

一个在 `core/src/crypto.rs` 文件中，就是 sp_core::Pair，这个 Pair 是一个 trait，trait 就是定位行为/特征（方法/函数）的地方。

我们的重点是 Pair struct。当然，其实在 `core/src/sr25519.rs` 里面也用到了 `core/src/crypto.rs` 的 trait Pair，并且改名为 TraitPair：
```rust
use crate::crypto::{
	Pair as TraitPair, DeriveJunction, Infallible, SecretStringError
};
```
并且还 impl 了：
```rust
#[cfg(feature = "full_crypto")]
impl TraitPair for Pair {
    ···
}
```
后续我们可以看到，TraitPair 里面的 from_string_with_seed 是一个很重要的函数。

在 `core/src/sr25519.rs` 里面有 `sp_core::sr25519::Pair`：

```rust
pub struct Pair(pub Keypair);
```
可以看出，这个文件里面的 Pair 是一个 struct。
注意，这里 Keypair 本不是 Pub，我这里想打印出 Keypair 的一些东西，所以我这里改成为 pub，这样仅仅用于测试，不要用于生产环境。

这个 struct 本身只是对另外一个 Struct Keypair 的包装，并且：
```rust
use schnorrkel::{signing_context, ExpansionMode, Keypair, SecretKey, MiniSecretKey, PublicKey,
	derive::{Derivation, ChainCode, CHAIN_CODE_LENGTH}
};
```
这个 Keypair 来自于 [schnorrkel](https://github.com/w3f/schnorrkel)

## schnorrkel 库的 Keypair

https://github.com/w3f/schnorrkel

Keypair，包括公钥和私钥：
```rust
/// A Ristretto Schnorr keypair.
#[derive(Clone,Debug)]
// #[derive(Clone,Zeroize)]
// #[zeroize(drop)]
pub struct Keypair {
    /// The secret half of this keypair.
    pub secret: SecretKey,
    /// The public half of this keypair.
    pub public: PublicKey,
}
```

## schnorrkel 库的 PublicKey
而 PublicKey 是：
```rust
/// A Ristretto Schnorr public key.
///
/// Internally, these are represented as a `RistrettoPoint`, meaning
/// an Edwards point with a static guarantee to be 2-torsion free.
///
/// At present, we decompress `PublicKey`s into this representation
/// during deserialization, which improves error handling, but costs
/// a compression during signing and verifiaction.
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PublicKey(pub (crate) RistrettoBoth);
```

这个 RistrettoBoth，在 `src/points.rs` 里面：
```rust
/// A `RistrettoBoth` contains both an uncompressed `RistrettoPoint`
/// as well as the corresponding `CompressedRistretto`.  It provides
/// a convenient middle ground for protocols that both hash compressed
/// points to derive scalars for use with uncompressed points.
#[derive(Copy, Clone, Default, Eq)]  // PartialEq optimized below
pub struct RistrettoBoth {
    compressed: CompressedRistretto,
    point: RistrettoPoint,
}
```

毫不意外，`use curve25519_dalek::ristretto::{CompressedRistretto,RistrettoPoint};` RistrettoBoth 里面的两个类型又用到其他的库了，这次是 [curve25519-dalek](https://github.com/dalek-cryptography/curve25519-dalek)。这里补充一下，dalek 是大名鼎鼎的密码学基础库实现组织，基于 Rust，详见：https://dalek.rs/#about 

### curve25519-dalek 库的 CompressedRistretto RistrettoPoint
在 `src/ristretto.rs` 里：
```rust
/// A Ristretto point, in compressed wire format.
///
/// The Ristretto encoding is canonical, so two points are equal if and
/// only if their encodings are equal.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CompressedRistretto(pub [u8; 32]);
```
CompressedRistretto 是一个 [u8;32] 的数组，等效于256位。

RistrettoPoint 在 `src/ristretto.rs` 里
```rust
/// A `RistrettoPoint` represents a point in the Ristretto group for
/// Curve25519.  Ristretto, a variant of Decaf, constructs a
/// prime-order group as a quotient group of a subgroup of (the
/// Edwards form of) Curve25519.
///
/// Internally, a `RistrettoPoint` is implemented as a wrapper type
/// around `EdwardsPoint`, with custom equality, compression, and
/// decompression routines to account for the quotient.  This means that
/// operations on `RistrettoPoint`s are exactly as fast as operations on
/// `EdwardsPoint`s.
///
#[derive(Copy, Clone)]
pub struct RistrettoPoint(pub(crate) EdwardsPoint);
```
然后在 `src/edwards.rs` 里面：
```rust
/// An `EdwardsPoint` represents a point on the Edwards form of Curve25519.
#[derive(Copy, Clone)]
#[allow(missing_docs)]
pub struct EdwardsPoint {
    pub(crate) X: FieldElement,
    pub(crate) Y: FieldElement,
    pub(crate) Z: FieldElement,
    pub(crate) T: FieldElement,
}
```
然后在 `src/field.rs` 里有：FieldElement
```rust
/// A `FieldElement` represents an element of the field
/// \\( \mathbb Z / (2\^{255} - 19)\\).
///
/// The `FieldElement` type is an alias for one of the platform-specific
/// implementations.
#[cfg(feature = "u64_backend")]
pub type FieldElement = backend::serial::u64::field::FieldElement51;
```
在 `curve25519-dalek\src\backend\serial\u64\field.rs`：
```rust
/// A `FieldElement51` represents an element of the field
/// \\( \mathbb Z / (2\^{255} - 19)\\).
///
/// In the 64-bit implementation, a `FieldElement` is represented in
/// radix \\(2\^{51}\\) as five `u64`s; the coefficients are allowed to
/// grow up to \\(2\^{54}\\) between reductions modulo \\(p\\).
///
/// # Note
///
/// The `curve25519_dalek::field` module provides a type alias
/// `curve25519_dalek::field::FieldElement` to either `FieldElement51`
/// or `FieldElement2625`.
///
/// The backend-specific type `FieldElement51` should not be used
/// outside of the `curve25519_dalek::field` module.
#[derive(Copy, Clone)]
pub struct FieldElement51(pub (crate) [u64; 5]);
```
---

还挺复杂的，最后总结 publicKey 由 [u8;32] 和 4 个 [u64; 5] 组成，等效于 1536 位， 256 * 6。

## schnorrkel 库 的 SecretKey

对于私钥，SecretKey：
```rust
/// A seceret key for use with Ristretto Schnorr signatures.
///
/// Internally, these consist of a scalar mod l along with a seed for
/// nonce generation.  In this way, we ensure all scalar arithmatic
/// works smoothly in operations like threshold or multi-signatures,
/// or hierarchical deterministic key derivations.
///
/// We keep our secret key serializaion "almost" compatable with EdDSA
/// "expanded" secret key serializaion by multiplying the scalar by the
/// cofactor 8, as integers, and dividing on deserializaion.
/// We do not however attempt to keep the scalar's high bit set, especially
/// not during hierarchical deterministic key derivations, so some Ed25519
/// libraries might compute the public key incorrectly from our secret key.
#[derive(Clone,Zeroize)]
#[zeroize(drop)]
pub struct SecretKey {
    /// Actual public key represented as a scalar.
    pub (crate) key: Scalar,
    /// Seed for deriving the nonces used in signing.
    ///Pri
    /// We require this be random and secret or else key compromise attacks will ensue.
    /// Any modificaiton here may dirupt some non-public key derivation techniques.
    pub (crate) nonce: [u8; 32],
}
```
包括一个 Scalar，和一个 nonce，nonce 就是一个 [u8; 32]，等效于256位。
Scalar 就需要又去 `curve25519-dalek\src\scalar.rs` 看了。

### curve25510-dalek 库的 Scalar
`curve25519-dalek\src\scalar.rs`
```rust
/// The `Scalar` struct holds an integer \\(s < 2\^{255} \\) which
/// represents an element of \\(\mathbb Z / \ell\\).
#[derive(Copy, Clone, Hash)]
pub struct Scalar {
    /// `bytes` is a little-endian byte encoding of an integer representing a scalar modulo the
    /// group order.
    ///
    /// # Invariant
    ///
    /// The integer representing this scalar must be bounded above by \\(2\^{255}\\), or
    /// equivalently the high bit of `bytes[31]` must be zero.
    ///
    /// This ensures that there is room for a carry bit when computing a NAF representation.
    //
    // XXX This is pub(crate) so we can write literal constants.  If const fns were stable, we could
    //     make the Scalar constructors const fns and use those instead.
    pub(crate) bytes: [u8; 32],
}
```
可以看出，Scalar 是一个 [u8; 32]，等效于 256 位。

---
因此 SecretKey 相当于 512 位。


然后整个 Pair 占了 512 + 1536 = 256 * 8  = 2048 bit。

# sp_core 库的 Pair::from_string()


我们先看 `Pair::from_string()`，在 `core/src/sr25519.rs` 里面没有看到有方法为 struct Pair 实现了。但是我们可以看到：
```rust
#[cfg(feature = "full_crypto")]
use crate::crypto::{
	Pair as TraitPair, DeriveJunction, Infallible, SecretStringError
};

#[cfg(feature = "full_crypto")]
impl TraitPair for Pair {
    ···
}
```
源码从 crypto 里面导入了 trait Pair，并且改名为 TraitPair，并且为 struct Pair 实现了 TraitPair。

去查看 `core/src/crypto.rs` 里面的 trait Pair：
```rust
/// Trait suitable for typical cryptographic PKI key pair type.
///
/// For now it just specifies how to create a key from a phrase and derivation path.
#[cfg(feature = "full_crypto")]
pub trait Pair: CryptoType + Sized + Clone + Send + Sync + 'static {

    ···
	/// Interprets the string `s` in order to generate a key Pair. Returns both the pair and an optional seed, in the
	/// case that the pair can be expressed as a direct derivation from a seed (some cases, such as Sr25519 derivations
	/// with path components, cannot).
	///
	/// This takes a helper function to do the key generation from a phrase, password and
	/// junction iterator.
	///
	/// - If `s` is a possibly `0x` prefixed 64-digit hex string, then it will be interpreted
	/// directly as a `MiniSecretKey` (aka "seed" in `subkey`).
	/// - If `s` is a valid BIP-39 key phrase of 12, 15, 18, 21 or 24 words, then the key will
	/// be derived from it. In this case:
	///   - the phrase may be followed by one or more items delimited by `/` characters.
	///   - the path may be followed by `///`, in which case everything after the `///` is treated
	/// as a password.
	/// - If `s` begins with a `/` character it is prefixed with the Substrate public `DEV_PHRASE` and
	/// interpreted as above.
	///
	/// In this case they are interpreted as HDKD junctions; purely numeric items are interpreted as
	/// integers, non-numeric items as strings. Junctions prefixed with `/` are interpreted as soft
	/// junctions, and with `//` as hard junctions.
	///
	/// There is no correspondence mapping between SURI strings and the keys they represent.
	/// Two different non-identical strings can actually lead to the same secret being derived.
	/// Notably, integer junction indices may be legally prefixed with arbitrary number of zeros.
	/// Similarly an empty password (ending the SURI with `///`) is perfectly valid and will generally
	/// be equivalent to no password at all.
	///
	/// `None` is returned if no matches are found.
	#[cfg(feature = "std")]
	fn from_string_with_seed(s: &str, password_override: Option<&str>)
		-> Result<(Self, Option<Self::Seed>), SecretStringError>
	{
		let re = Regex::new(r"^(?P<phrase>[\d\w ]+)?(?P<path>(//?[^/]+)*)(///(?P<password>.*))?$")
			.expect("constructed from known-good static value; qed");
		let cap = re.captures(s).ok_or(SecretStringError::InvalidFormat)?;

		let re_junction = Regex::new(r"/(/?[^/]+)")
			.expect("constructed from known-good static value; qed");
		let path = re_junction.captures_iter(&cap["path"])
			.map(|f| DeriveJunction::from(&f[1]));

		let phrase = cap.name("phrase").map(|r| r.as_str()).unwrap_or(DEV_PHRASE);
		let password = password_override.or_else(|| cap.name("password").map(|m| m.as_str()));

		let (root, seed) = if phrase.starts_with("0x") {
			hex::decode(&phrase[2..]).ok()
				.and_then(|seed_vec| {
					let mut seed = Self::Seed::default();
					if seed.as_ref().len() == seed_vec.len() {
						seed.as_mut().copy_from_slice(&seed_vec);
						Some((Self::from_seed(&seed), seed))
					} else {
						None
					}
				})
				.ok_or(SecretStringError::InvalidSeed)?
		} else {
			Self::from_phrase(phrase, password)
				.map_err(|_| SecretStringError::InvalidPhrase)?
		};
		root.derive(path, Some(seed)).map_err(|_| SecretStringError::InvalidPath)
	}


    /// Interprets the string `s` in order to generate a key pair.
	///
	/// See [`from_string_with_seed`](Pair::from_string_with_seed) for more extensive documentation.
	#[cfg(feature = "std")]
	fn from_string(s: &str, password_override: Option<&str>) -> Result<Self, SecretStringError> {
		Self::from_string_with_seed(s, password_override).map(|x| x.0)
    }
}
```
可以看出，from_string() 最终还是调用 from_string_with_seed()。然后就详细看一下怎么从种子生成 Pair。


# sp_core 库的 Pair::sign()
然后再看 `Pair::sign()`:
```rust
#[cfg(feature = "full_crypto")]
use schnorrkel::{signing_context, ExpansionMode, Keypair, SecretKey, MiniSecretKey, PublicKey,
	derive::{Derivation, ChainCode, CHAIN_CODE_LENGTH}
};

// signing context
#[cfg(feature = "full_crypto")]
const SIGNING_CTX: &[u8] = b"substrate";

···
#[cfg(feature = "full_crypto")]
impl TraitPair for Pair {
    
    ···
	fn sign(&self, message: &[u8]) -> Signature {
		let context = signing_context(SIGNING_CTX);
		self.0.sign(context.bytes(message)).into()
    }
    ···
```
可以看出，签名用到了 schnorrkel 库的内容，包括 `schnorrkel::Keypair::sign()`，`schnorrkel::signing_context`, `schnorrkel::context::SigningContext::bytes()`。rust 文档参考：https://docs.rs/schnorrkel/0.9.1/schnorrkel/index.html。


