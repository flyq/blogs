# Candid

## Motoko
Motoko 项目里面语言自带的哪些接口会暴露等等，并且会自动生成 Candid 文件，没啥问题。

## Rust
Rust 项目里面需要用到依赖库。

### dfn_candid

一个是 ic 源码里面的 dfn_candid，它是基于 candid 0.6.15 封装而成。

**依赖**
```toml
serde = "1.0.99"
candid = "0.6.15"
on_wire = { path = "../on_wire" }
dfn_core = { path = "../dfn_core" }
ic-base-types = {path="../../types/base_types"}
```

**和 candid 的区别**
1. 新建了一个 Candid, CandidOne 结构，并实现了 `on_wire::{FromWire, IntoWire, NewType}` 这几个 trait。 其中 `Candid<T>` 的 T 受 ArgumentEncoder 和 ArgumentDecoder trait 约束。`CandidOne<T>` 的 T 受 CandidType 和 DeserializeOwned trait 约束。
   ```rs
   pub struct Candid<T>(pub T);
   pub struct CandidOne<T>(pub T);
   ```
2. ```rs
   pub fn candid<A, B>(a: CandidOne<A>, b: B) -> (A, Candid<B>) {}

   pub fn candid_multi_arity<A, B>(a: Candid<A>, b: B) -> (A, Candid<B>) {}

   pub fn candid_one<A, B>(a: CandidOne<A>, b: B) -> (A, CandidOne<B>) {}
   ```
   这几个公开结构，帮忙将变量装换成成输出。

### candid
这个主要查看 https://docs.rs/candid/0.7.1/candid/index.html
代码在 https://github.com/dfinity/candid

