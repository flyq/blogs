# Polkadot 学习资源汇总
这里将实时收集汇总 Polkadot 相关的学习资源，包括 Polkadot 本身以及 Substrate 框架。
## `Contents`
- [Polkadot 学习资源汇总](#polkadot-%e5%ad%a6%e4%b9%a0%e8%b5%84%e6%ba%90%e6%b1%87%e6%80%bb)
  - [`Contents`](#contents)
  - [Substrate 是什么](#substrate-%e6%98%af%e4%bb%80%e4%b9%88)
  - [导航类资源](#%e5%af%bc%e8%88%aa%e7%b1%bb%e8%b5%84%e6%ba%90)
    - [awesome-substrate](#awesome-substrate)
    - [Substrate.dev](#substratedev)
  - [主页类](#%e4%b8%bb%e9%a1%b5%e7%b1%bb)
    - [Plo](#plo)


## Substrate 是什么
> [Substrate](https://github.com/paritytech/substrate) 是一个开源的区块链构建框架，可以在短时间内建立完整、可配置的区块链。另外一个可选功能是，可以将构建的区块链部署到 [Polkadot](https://github.com/paritytech/polkadot) 网络中，这一网络拥有共享安全等其它优势。而且，包括 Kusama 网络，Polkadot 主网，也将基于 Substrate 构建。更多内容参考[这里](https://www.parity.io/a-brief-summary-of-everything-substrate-polkadot/)。

使用Substrate，你可以获得下面这些**开箱即用**的功能，还可以自定义这些功能：

* 一条PoS的区块链
* 可升级的运行环境
* 插件式的共识 (PoS, PoW, PoA)
* 区块生产和同步
* 高效存储
* 高效状态机
* P2P网络层
* 内置的基本加密工具
* 支持轻客户端
* [工具集](https://substrate.dev/rustdocs/v1.0/substrate_service/index.html?search=srml)，用来修改运行环境的逻辑，例如管理存储、账户、权益、资产、共识算法等等
* UI 工具

它有一下特点：
- [x] Everything you Need to Build a Blockchain
- [x] Fast and efficient database.
- [x] Modular P2P networking stack in libp2p.
- [x] Hot-swappable consensus layer.
- [x] Customizable transaction queue management system.
- [x] Diverse library of runtime modules.

## 导航类资源
### awesome-substrate
- [英文版](https://github.com/substrate-developer-hub/awesome-substrate/blob/master/README.md)
- [中文版](https://github.com/substrate-developer-hub/awesome-substrate/blob/master/README_CN.md)

`标签`：`官方` `汇总`  `实时更新`

这里实时更新了 Substrate 相关的信息的汇总。在这里可以得到：
- Substrate Wiki、Polkadot Wiki、Research Wiki 等相关的官方文档；
- 可以讨论问题以及得到最新信息的社交频道；
- 有 Demo 讲解的开发指南；
- 各种主题的演讲视频链接；
- 以及 Polkadot 生态里面的相关项目；
- 最新公告和活动等。

### Substrate.dev

https://substrate.dev/

`标签`：`官方` `开发者中心`

这里是 Polkadot 团队官方维护的 Substrate 开发者中心，为了方便开发者学习、研究、使用 Substrate，Polkadot 团队精心地写了非常有用的文档：

- [Substrate 文档](https://substrate.dev/rustdocs/)：官方 Substrate 文档。这个文档以 Rust 的标准库文档的形式给出，如果之前看过 [Rust 标准库文档](https://doc.rust-lang.org/std/index.html)的话，会很习惯这个形式。这里包含了 Substrate 框架里面所有涉及到的库。比如 Substrate 里面实现的[工具集](https://substrate.dev/rustdocs/v1.0/substrate_service/index.html?search=srml)，提供了很多非常好的功能。
- [Substrate Wiki](https://substrate.dev/docs/)：指南、实例和深入解释。这里讲解了 Substrate 是什么，怎么安装，并且深入讲解了 Substrate 里面的一些细节：包括 Extrinsics，Transaction Lifecycle，Low-level Data Format，Off-Chain Workers，Session Keys，SS58 Address Format 以及 Runtime，还有一些工具：比如 The subkey Tool，ink! 智能合约工具。
- [Substrate recipes 最佳实践](https://substrate.dev/recipes/)：这里是一组简单的代码模式，这些代码模式演示了使用 Substrate 构建区块链时的最佳实践。通过它可以学习怎样写出更加安全的 Runtime module。
- [Substrate 示例教程](https://substrate.dev/en/tutorials)：这里有难易不同的好几个示例教程，手把手地带你基于 Substrate 来开发某个场景下的区块链。当你对 Polkadot 等有一个大概的了解之后，非常推荐从这里开始 Substrate Coding 之旅。
- [交流方式](https://substrate.dev/en/community)：这里的 Riot Chat 是类似一个微信的存在，里面可以讨论关于 Substrate 的问题，甚至有时候能够得到 Gavin Wood 的回答！
- [Get Started on Substrate](https://substrate.dev/en/who/runtime-developer)：从这里可以学会怎样从零开始使用 Substrate 构建一个区块链。
- [Get Started on Polkadot-JS](https://substrate.dev/en/who/front-end-developer)：从这里可以学会怎样从零开始使用 Polkadot-JS 来让前端与 Substrate 链进行交互。
- [Get Started on ink!](https://substrate.dev/en/who/contract-developer)：从这里可以学会怎样从零开始使用 ink! 来构建运行在 Substrate 链上面的智能合约。
- [基于 Substrate 构建的区块链项目](https://substrate.dev/en/users.html)：从这里可以看到目前已经有了四五十个项目使用 Substrate 构建他们的区块链。

## 主页类
### Plo