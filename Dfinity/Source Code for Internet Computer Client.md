# Source Code for Internet Computer Client

## Contents
- [Source Code for Internet Computer Client](#source-code-for-internet-computer-client)
  - [Contents](#contents)
  - [背景](#背景)
  - [试运行](#试运行)
    - [问题一 bindgen 版本冲突：](#问题一-bindgen-版本冲突)


## 背景
互联网计算机的客户端程序开源了，代码在 https://github.com/dfinity/ic

## 试运行
官方文档只有一个 docker 编译的文档，我准备直接 naive 编译。

fork ic 代码到自己的仓库，方便修改。

git clone 自己的仓库到本地。

先创建一个 .gitignore 文件，然后提交，以免后续编译后巨大的 target 文件夹被加入到 .git/ 目录

接着开始编译：

编译组件1: ic-replica:
```sh
cargo build -p ic-replica
```

### 问题一 bindgen 版本冲突：
出现版本冲突时，我们的原则是优先使用最新的版本。一般都会前向兼容。

```sh
➜  rs git:(master) ✗ cargo build -p ic-replica
warning: path override for crate `rocksdb` has altered the original list of
dependencies; the dependency on `librocksdb-sys` was either added or
modified to not match the previously resolved version

This is currently allowed but is known to produce buggy behavior with spurious
recompiles and changes to the crate graph. Path overrides unfortunately were
never intended to support this feature, so for now this message is just a
warning. In the future, however, this message will become a hard error.

To change the dependency graph via an override it's recommended to use the
`[replace]` feature of Cargo instead of the path override feature. This is
documented online at the url below for more information.

https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html

    Updating crates.io index
error: failed to select a version for `clang-sys`.
    ... required by package `bindgen v0.55.0`
    ... which is depended on by `librocksdb-sys v6.11.4 (/opt/homebrew/opt/rust-rocksdb)`
    ... which is depended on by `rocksdb v0.15.0 (/opt/homebrew/opt/rust-rocksdb)`
    ... which is depended on by `ic-artifact-pool v0.8.0 (/Users/flyq/workspace/github/ic/rs/artifact_pool)`
    ... which is depended on by `ic-p2p v0.8.0 (/Users/flyq/workspace/github/ic/rs/p2p)`
    ... which is depended on by `ic-replica v0.8.0 (/Users/flyq/workspace/github/ic/rs/replica)`
versions that meet the requirements `^1` are: 1.2.0, 1.1.1, 1.1.0, 1.0.3, 1.0.2, 1.0.1, 1.0.0

the package `clang-sys` links to the native library `clang`, but it conflicts with a previous package which links to `clang` as well:
package `clang-sys v0.29.3`
    ... which is depended on by `bindgen v0.53.3`
    ... which is depended on by `lmdb-rkv-sys v0.11.99 (https://github.com/dfinity-lab/lmdb-rs?rev=e45bfaaa9055178ab55f8f31bfece50ebd94f901#e45bfaaa)`
    ... which is depended on by `lmdb-rkv v0.14.99 (https://github.com/dfinity-lab/lmdb-rs?rev=e45bfaaa9055178ab55f8f31bfece50ebd94f901#e45bfaaa)`
    ... which is depended on by `ic-artifact-pool v0.8.0 (/Users/flyq/workspace/github/ic/rs/artifact_pool)`
    ... which is depended on by `ic-p2p v0.8.0 (/Users/flyq/workspace/github/ic/rs/p2p)`
    ... which is depended on by `ic-replica v0.8.0 (/Users/flyq/workspace/github/ic/rs/replica)`

failed to select a version for `clang-sys` which could resolve this conflict
```

里面某个组件依赖的库 bindgen 版本冲突，修改至一致。先尝试把低版本，lmdb-rkv-sys 依赖的 bindgen 版本升级至 0.55.0：
1. fork https://github.com/dfinity-lab/lmdb-rs，修改 [./lmdb-sys/Cargo.toml](https://github.com/flyq/lmdb-rs/blob/master/lmdb-sys/Cargo.toml#L31)，
    ```toml
    bindgen = { version = "0.55.0", default-features = false, features = ["runtime"] }
    ```
1. 修改 [./rs/artifact_pool/Cargo.toml line 25](https://github.com/flyq/ic/blob/master/rs/artifact_pool/Cargo.toml#L25)， 以及 [./rs/artifact_pool/Cargo.toml line 42](https://github.com/flyq/ic/blob/master/rs/artifact_pool/Cargo.toml#L42)
   ```toml
   ...
   lmdb-rkv = { git = "https://github.com/flyq/lmdb-rs", rev = "a5276d05f10d44915f490c7ba2201161db9225ac" }
   ...
   lmdb-rkv-sys = { git = "https://github.com/flyq/lmdb-rs", rev = "a5276d05f10d44915f490c7ba2201161db9225ac" }
   ...
   ```
1. 接着编译，发现仍没修改干净 cow_state 也依赖这个库：
   ```sh
   ➜  rs git:(master) cargo build -p ic-replica
        Updating git repository `https://github.com/flyq/lmdb-rs`
        Updating crates.io index
    error: failed to select a version for `clang-sys`.
        ... required by package `bindgen v0.55.0`
        ... which is depended on by `lmdb-rkv-sys v0.11.99 (https://github.com/flyq/lmdb-rs?rev=a5276d05f10d44915f490c7ba2201161db9225ac#a5276d05)`
        ... which is depended on by `ic-artifact-pool v0.8.0 (/Users/flyq/workspace/github/q/Untitled/rs/artifact_pool)`
        ... which is depended on by `ic-artifact-manager v0.8.0 (/Users/flyq/workspace/github/q/Untitled/rs/artifact_manager)`
        ... which is depended on by `ic-p2p v0.8.0 (/Users/flyq/workspace/github/q/Untitled/rs/p2p)`
        ... which is depended on by `ic-replica v0.8.0 (/Users/flyq/workspace/github/q/Untitled/rs/replica)`
    versions that meet the requirements `^1` are: 1.2.0, 1.1.1, 1.1.0, 1.0.3, 1.0.2, 1.0.1, 1.0.0

    the package `clang-sys` links to the native library `clang`, but it conflicts with a previous package which links to `clang` as well:
    package `clang-sys v0.29.3`
        ... which is depended on by `bindgen v0.53.3`
        ... which is depended on by `lmdb-rkv-sys v0.11.99 (https://github.com/dfinity-lab/lmdb-rs?rev=e45bfaaa9055178ab55f8f31bfece50ebd94f901#e45bfaaa)`
        ... which is depended on by `lmdb-rkv v0.14.99 (https://github.com/dfinity-lab/lmdb-rs?rev=e45bfaaa9055178ab55f8f31bfece50ebd94f901#e45bfaaa)`
        ... which is depended on by `ic-cow-state v0.8.0 (/Users/flyq/workspace/github/q/Untitled/rs/cow_state)`
        ... which is depended on by `ic-canonical-state v0.8.0 (/Users/flyq/workspace/github/q/Untitled/rs/canonical_state)`
        ... which is depended on by `ic-messaging v0.8.0 (/Users/flyq/workspace/github/q/Untitled/rs/messaging)`
        ... which is depended on by `ic-consensus v0.8.0 (/Users/flyq/workspace/github/q/Untitled/rs/consensus)`
        ... which is depended on by `ic-artifact-pool v0.8.0 (/Users/flyq/workspace/github/q/Untitled/rs/artifact_pool)`
        ... which is depended on by `ic-artifact-manager v0.8.0 (/Users/flyq/workspace/github/q/Untitled/rs/artifact_manager)`
        ... which is depended on by `ic-p2p v0.8.0 (/Users/flyq/workspace/github/q/Untitled/rs/p2p)`
        ... which is depended on by `ic-replica v0.8.0 (/Users/flyq/workspace/github/q/Untitled/rs/replica)`

    failed to select a version for `clang-sys` which could resolve this conflict
    ```
1. 同样的，修改 [./rs/cow_state/Cargo.toml](https://github.com/flyq/ic/blob/master/rs/cow_state/Cargo.toml#L14)，然后接着编译，仍然有问题：
   ```sh
    ➜  rs git:(master) cargo build -p ic-replica
        Updating git repository `https://github.com/flyq/lmdb-rs`
        Updating crates.io index
    error: failed to select a version for `clang-sys`.
        ... required by package `bindgen v0.55.0`
        ... which is depended on by `lmdb-rkv-sys v0.11.99 (https://github.com/flyq/lmdb-rs?rev=a5276d05f10d44915f490c7ba2201161db9225ac#a5276d05)`
        ... which is depended on by `ic-artifact-pool v0.8.0 (/Users/flyq/workspace/github/q/Untitled/rs/artifact_pool)`
        ... which is depended on by `ic-artifact-manager v0.8.0 (/Users/flyq/workspace/github/q/Untitled/rs/artifact_manager)`
        ... which is depended on by `ic-p2p v0.8.0 (/Users/flyq/workspace/github/q/Untitled/rs/p2p)`
        ... which is depended on by `ic-replica v0.8.0 (/Users/flyq/workspace/github/q/Untitled/rs/replica)`
    versions that meet the requirements `^1` are: 1.2.0, 1.1.1, 1.1.0, 1.0.3, 1.0.2, 1.0.1, 1.0.0

    the package `clang-sys` links to the native library `clang`, but it conflicts with a previous package which links to `clang` as well:
    package `clang-sys v0.29.3`
        ... which is depended on by `bindgen v0.54.0`
        ... which is depended on by `librocksdb-sys v6.11.4`
        ... which is depended on by `rocksdb v0.15.0`
        ... which is depended on by `ic-artifact-pool v0.8.0 (/Users/flyq/workspace/github/q/Untitled/rs/artifact_pool)`
        ... which is depended on by `ic-artifact-manager v0.8.0 (/Users/flyq/workspace/github/q/Untitled/rs/artifact_manager)`
        ... which is depended on by `ic-p2p v0.8.0 (/Users/flyq/workspace/github/q/Untitled/rs/p2p)`
        ... which is depended on by `ic-replica v0.8.0 (/Users/flyq/workspace/github/q/Untitled/rs/replica)`

    failed to select a version for `clang-sys` which could resolve this conflict
   ```
1. 上面的问题是 `rocksdb -> librocksdb-sys` 依赖有问题，直接把 整个 rocksdb 替换掉，因为我
   1. 下载已经魔改了 rocksdb 库
   ```sh
    cd /opt/homebrew/opt/
    git clone https://github.com/hdevalence/rust-rocksdb.git
    cd rust-rocksdb
    git submodule update --init --recursive
   ```
   1. 修改 cargo config
      ```sh
      emacs ~/.cargo/config
      ```
      添加
      ```sh
      paths = ["/opt/homebrew/opt/rust-rocksdb/"]
      ```
      最后同步 env PATH
      ```sh
      source ~/.cargo/env
      ```
    1. 修改 [ic/rs/artifact_pool](https://github.com/flyq/ic/blob/master/rs/artifact_pool/Cargo.toml#L27) 为本地：
       ```toml
       rocksdb = { path = "/opt/homebrew/opt/rust-rocksdb", optional = true }
       ```
    1. 然后 cd 切换到 ic/rs 目录，接着编译
       ```sh
        ➜  rs git:(master) cargo build -p ic-replica
            Updating crates.io index
            Updating git repository `https://github.com/flyq/lmdb-rs`
          Downloaded enum-map-derive v0.4.3
         ...
         Compiling ic-types v0.1.2 (https://github.com/dfinity/agent-rs.git?rev=6652a800969e7e09ecdf40ed16a2d78abecde7d3#6652a800)
         Compiling serde_bytes v0.11.4
         Compiling serde_cbor v0.11.1
         Compiling url v2.2.1
        error: failed to run custom build command for `lmdb-rkv-sys v0.11.99 (https://github.com/flyq/lmdb-rs?rev=a5276d05f10d44915f490c7ba2201161db9225ac#a5276d05)`

        Caused by:
        process didn't exit successfully: `/Users/flyq/workspace/github/q/Untitled/rs/target/debug/build/lmdb-rkv-sys-3b0066ce8f91d05e/build-script-build` (exit code: 101)
        --- stderr
        error: header '/Users/flyq/.cargo/git/checkouts/lmdb-rs-2503a52cba40d0bf/a5276d0/lmdb-sys/lmdb/libraries/liblmdb/lmdb.h' does not exist.
        thread 'main' panicked at 'Unable to generate bindings: ()', /Users/flyq/.cargo/git/checkouts/lmdb-rs-2503a52cba40d0bf/a5276d0/lmdb-sys/bindgen.rs:73:10
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        warning: build failed, waiting for other jobs to finish...
        warning: rocksdb/options/options_helper.cc:659:19: warning: loop variable 'iter' creates a copy from type 'const std::__1::pair<const std::__1::basic_string<char>, rocksdb::OptionTypeInfo>' [-Wrange-loop-construct]
        warning:   for (const auto iter : type_info) {
        warning:                   ^
        warning: rocksdb/options/options_helper.cc:659:8: note: use reference type 'const std::__1::pair<const std::__1::basic_string<char>, rocksdb::OptionTypeInfo> &' to prevent copying
        warning:   for (const auto iter : type_info) {
        warning:        ^~~~~~~~~~~~~~~~~
        warning:                   &
        warning: 1 warning generated.
        warning: rocksdb/util/crc32c.cc:345:20: warning: unused function 'Fast_CRC32' [-Wunused-function]
        warning: static inline void Fast_CRC32(uint64_t* l, uint8_t const **p) {
        warning:                    ^
        warning: 1 warning generated.
        error: build failed
        ```
