# 基于 Substrate 开发一条 Token 链

这个文档将手把手教你怎么从零开始使用 Substrate 框架开发一条 Token 链。顾名思义，这条链的功能就是能够在上面发行 Token。其实，类似的文档官方有一个：[substrate-collectables-workshop](https://substrate.dev/substrate-collectables-workshop/#/)，而且汉化也做得非常好，里面的逻辑参考的是曾经在以太坊上大火的 CryptoKitties。考虑到大家可能对 ERC20 Token 合约更加熟悉，因此写了这个文档，如果大家先把这个文档过了一遍，再去看官方的 substrate-collectables-workshop，就会更容易上手。

⚠️ NOTE: 本文档以及里面涉及到的代码仅作 Demo 展示用，不要将其用于生产环境。

## `Contents`
- [基于 Substrate 开发一条 Token 链](#%e5%9f%ba%e4%ba%8e-substrate-%e5%bc%80%e5%8f%91%e4%b8%80%e6%9d%a1-token-%e9%93%be)
  - [Contents](#contents)
  - [环境搭建](#%e7%8e%af%e5%a2%83%e6%90%ad%e5%bb%ba)
    - [Rust 环境搭建](#rust-%e7%8e%af%e5%a2%83%e6%90%ad%e5%bb%ba)
      - [更新 Ubuntu 软件源](#%e6%9b%b4%e6%96%b0-ubuntu-%e8%bd%af%e4%bb%b6%e6%ba%90)
      - [安装 Rust](#%e5%ae%89%e8%a3%85-rust)
      - [设置 Rust 源代理](#%e8%ae%be%e7%bd%ae-rust-%e6%ba%90%e4%bb%a3%e7%90%86)

## 环境搭建

接下来将讲解基于 Ubuntu 16.04 搭建环境。

### Rust 环境搭建

#### 更新 Ubuntu 软件源

分步骤执行
```shell
sudo apt update

sudo apt upgrade
```

#### 安装 Rust
参考 [在 Linux 或 macOS 上安装](https://rustlang-cn.org/office/rust/book/getting-started/ch01-01-installation.html#%E5%9C%A8-linux-%E6%88%96-macos-%E4%B8%8A%E5%AE%89%E8%A3%85-rustup)
```shell
curl https://sh.rustup.rs -sSf | sh
```
如果网速正常，就等它安装完就行，如果速度很慢，可以尝试下一步的设置代理：

#### 设置 Rust 源代理
参考 [rust rustup安装走代理](https://blog.csdn.net/bu2_int/article/details/79758960)
