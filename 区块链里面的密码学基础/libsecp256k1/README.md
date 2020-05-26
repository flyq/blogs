# libsecp256k1

Pure Rust implementation of the secp256k1 curve and fast ECDSA signatures. 

## contents
- [libsecp256k1](#libsecp256k1)
  - [contents](#contents)
- [1. 依赖](#1-依赖)
  - [1.1 libsecp256k1-core](#11-libsecp256k1-core)
    - [1.1.1 digest](#111-digest)
    - [1.1.2 subtle](#112-subtle)
    - [1.1.3 crunchy](#113-crunchy)
  - [1.2 arrayref](#12-arrayref)
  - [1.3 rand](#13-rand)


# 1. 依赖

## 1.1 libsecp256k1-core
dependencies：

### 1.1.1 digest
**version**: 0.8

**功能**：

提供密码 Hash 函数的 traits

底层 traits：

Input, BlockInput, Reset, FixedOutput, VariableOutput, ExtendableOutput.

便利 traits：

Digest，DynDigest

Hash 函数实现了 std 的 traits：Default，Clone，Write。

Digest trait 是最常用的 trait：
```rust
fn new() -> Self{}
fn input<B: AsRef<[u8]>>(&mut self, data: B){}
fn chain<B: AsRef<[u8]>>(self, data: B) -> Self
where
    Self: Sized, 
{}
fn result(self) -> GenericArray<u8, Self::OutputSize>{}
fn result_reset(&mut self) -> GenericArray<u8, Self::OutputSize>{}
fn reset(&mut self){}
fn output_size() -> usize{}
fn digest(data: &[u8]) -> GenericArray<u8, Self::OutputSize>{}
```


### 1.1.2 subtle
version: 2.2

**功能**：

在 constant 时间内的条件选择。

### 1.1.3 crunchy
version： 0.2

**功能**：

提高循环逻辑的处理性能。
通过内联展开循环代码的。

```rust
unroll! {
  for i in 0..5 {
    println!("Iteration {}", i);
  }
}
```
will expand into:
```rust
{ println!("Iteration {}", 0); }
{ println!("Iteration {}", 1); }
{ println!("Iteration {}", 2); }
{ println!("Iteration {}", 3); }
{ println!("Iteration {}", 4); }
```


## 1.2 arrayref
version：0.3

四个宏，给数组创建 ref。

```rust
array_mut_ref!()

array_ref!()

array_refs()

mut_array_refs()
```

## 1.3 rand
version：0.7




