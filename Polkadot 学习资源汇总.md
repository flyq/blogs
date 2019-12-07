# Polkadot 学习资源汇总
这里将实时收集汇总 Polkadot 相关的学习资源，包括 Polkadot 本身以及 Substrate 框架。
## `Contents`
- [Polkadot 学习资源汇总](#polkadot-%e5%ad%a6%e4%b9%a0%e8%b5%84%e6%ba%90%e6%b1%87%e6%80%bb)
  - [`Contents`](#contents)
  - [Substrate 是什么](#substrate-%e6%98%af%e4%bb%80%e4%b9%88)
  - [导航类资源](#%e5%af%bc%e8%88%aa%e7%b1%bb%e8%b5%84%e6%ba%90)
    - [awesome-substrate](#awesome-substrate)
    - [substrate.dev](#substratedev)
  - [主页类](#%e4%b8%bb%e9%a1%b5%e7%b1%bb)
    - [Polkadot Wiki](#polkadot-wiki)
      - [Polkadot 综合性资源](#polkadot-%e7%bb%bc%e5%90%88%e6%80%a7%e8%b5%84%e6%ba%90)
      - [基于 Polkadot 开发](#%e5%9f%ba%e4%ba%8e-polkadot-%e5%bc%80%e5%8f%91)
      - [Polkadot 全面介绍](#polkadot-%e5%85%a8%e9%9d%a2%e4%bb%8b%e7%bb%8d)
      - [参与网络](#%e5%8f%82%e4%b8%8e%e7%bd%91%e7%bb%9c)


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

它有以下特点：
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

`标签`：`官方` `汇总`  `实时更新` `多语言版本`

这里实时更新了 Substrate 相关的信息的汇总。在这里可以得到：
- Substrate Wiki、Polkadot Wiki、Research Wiki 等相关的官方文档；
- 可以讨论问题以及得到最新信息的社交频道；
- 有 Demo 讲解的开发指南；
- 各种主题的演讲视频链接；
- 以及 Polkadot 生态里面的相关项目；
- 最新公告和活动等。

### substrate.dev

https://substrate.dev/

`标签`：`官方` `开发者中心` `多语言版本`

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
### Polkadot Wiki
https://wiki.polkadot.network/docs/en/

`标签`：`官方` `Wiki` `多语言版本`

#### Polkadot 综合性资源
- [Polakdot Wiki 的索引](https://wiki.polkadot.network/docs/en/)
- [社区](https://wiki.polkadot.network/docs/en/community)
  - 通用
    - 区块链浏览器：[Polkadot UI](https://polkadot.js.org/apps/#/explorer)
    - 源码链接：[Polkadot GitHub](https://github.com/paritytech/polkadot/)
    - [聚会宣讲活动](https://github.com/w3f/Web3-collaboration/blob/master/meetups.md)
  - Riot
    - [Polkadot 入门聊天室](https://riot.im/app/#/room/#polkadotnoobs:matrix.org)：供入门者了解 Polkadot 的聊天室，**提问的好地方**。
    - [Polkadot Watercooler Chat](https://riot.im/app/#/room/#polkadot-watercooler:matrix.org)：讨论 Polkadot 一般问题。
    - [Polkadot 验证人频道](https://riot.im/app/#/room/#polkadot-validator-lounge:matrix.org)：验证人学习如何设定节点。
    - [Substrate 技术频道](https://riot.im/app/#/room/#substrate-technical:matrix.org)：在这询问 Substrate 有关技术问题。
  - 媒体链接
    - [订阅 Polkadot 邮件](https://polkadot.network/#roadmap)：电子邮件注册表格在頁面底部。
    - [Polkadot Telegram 群](https://t.me/polkadotofficial)：Polkadot 官方电报群 (聊天前请先阅读规则)。
    - [Polkadot Twitter](https://twitter.com/polkadotnetwork)：Polkadot 官方推特帐户。
    - [Polkadot Reddit](https://www.reddit.com/r/dot/)：Polkadot 官方 Reddit 账号。
    - [Polkadot Youtube 频道](https://www.youtube.com/channel/UCB7PbjuZLEba_znc7mEGNgw)：来自创始人和 Polkadot 的视频内容，讲述了网络和愿景。
  - Polkadot 生态项目
    - [有那些项目在 Polkadot 生态上](https://forum.web3.foundation/t/teams-building-on-polkadot/67)
- [怎样参与贡献](https://wiki.polkadot.network/docs/zh-CN/contributing)
- [术语表](https://wiki.polkadot.network/docs/zh-CN/glossary)
- [新闻](https://wiki.polkadot.network/docs/zh-CN/news)
- [研究专页](https://wiki.polkadot.network/docs/zh-CN/research)

#### 基于 Polkadot 开发
- [开发者专页](https://wiki.polkadot.network/docs/en/build-index)
  - [Polkadot 开发者入门指南](https://wiki.polkadot.network/docs/zh-CN/build-build-with-polkadot)
  - [开发路线图](https://wiki.polkadot.network/docs/zh-CN/build-dev-roadmap)
  - [平行链开发套件(PDKs)](https://wiki.polkadot.network/docs/zh-CN/build-pdk)
  - [Cumulus](https://wiki.polkadot.network/docs/zh-CN/build-cumulus)
  - [智能合约](https://wiki.polkadot.network/docs/zh-CN/build-smart-contracts)
- [工具链索引](https://wiki.polkadot.network/docs/en/build-tools-index)
  - 区块链浏览器/资源管理器
    - [Polka.io](https://polka.io/)
    - [Polkadot-JS Apps Explorer](https://polkadot.js.org/apps/#/explorer)
    - [Polkascan](https://polkascan.io/)
  - 钱包
    - [Polkadot-JS Apps——账户](https://polkadot.js.org/apps/#/accounts)
    - [Bonds oo7 Polkadot UI](https://github.com/paritytech/substrate-ui)
    - [Polkawallet](https://polkawallet.io/)
    - [SpeckleOS](https://www.speckleos.io/)
    - [Enzyme](https://getenzyme.dev/)
    - [Math Wallet](https://www.mathwallet.org/)
  - 网络监控 & 报告
    - [Polkadot 遥测服务](https://telemetry.polkadot.io/)
    - [Polkabot](https://gitlab.com/Polkabot/polkabot)：Polkadot使用Riot聊天来进行网络监控和报告，用户可以自定义创建bot插件。[相关文章](https://medium.com/polkadot-network/polkabot-a3dba18c20c8)
    - [PolkaStats](https://polkastats.io/)：[GitHub 链接](https://github.com/Colm3na/polkastats-v2/)
  - Rust 相关项目
    - 客户端
      - [Polkadot](https://github.com/paritytech/polkadot)：Polkadot运行时环境的Rust实现
    - 工具
      - [Substrate](https://github.com/paritytech/substrate)
      - [Substrate Development Hub](https://substrate.dev/)
  - C++ 相关项目
    - [Kagome](https://github.com/soramitsu/kagome)：Polkadot C++ 客户端。
    - [Polkadot API Cpp](https://github.com/usetech-llc/polkadot_api_cpp)：С++ API for Polkadot
  - C# 相关项目
    - [Polkadot API - .NET](https://github.com/usetech-llc/polkadot_api_dotnet)：Polkadot Substrate API for .NET
  - Go 相关项目
    - [Gossamer](https://github.com/ChainSafe/gossamer): A Go implementation of the Polkadot Runtime Environment.
    - [Golkadot](https://github.com/opennetsys/golkadot): A Go implementation of Polkadot Substrate.
    - [GSRPC](https://github.com/centrifuge/go-substrate-rpc-client/): Substrate RPC client for go aka GSRPC
  - JS 相关项目
    - 客户端
      - [Polkadot-JS client](https://github.com/polkadot-js/client)
    - 库
      - [Polkadot-JS Common](https://polkadot.js.org/common/)
      - [Bonds oo7](https://github.com/polkadot-js/oo7-polkadot)
    - 命令行工具
      - [@polkadot/api-cli](https://github.com/polkadot-js/tools/tree/master/packages/api-cli)：Polkadot API 的简单命令行界面。[文档](https://polkadot.js.org/api/api/)
      - [@polkadot/monitor-rpc ](https://github.com/polkadot-js/tools/tree/master/packages/monitor-rpc) Polkadot 的一个简单 RPC 监视器。
    - RPC 工具
      - [@polkadot/api/rpc-provider](https://github.com/polkadot-js/api/tree/master/packages/rpc-provider)
      - [RPC documentation](RPC documentation)
- 其他资源
  - [交易所整合](https://wiki.polkadot.network/docs/zh-CN/build-exchange-integration)
  - [例子](https://wiki.polkadot.network/docs/zh-CN/build-examples-index)：这里有很多和 Polkadot 相关的例子。
  - [如何查看及部署平行链](https://wiki.polkadot.network/docs/zh-CN/build-deploy-parachains)
  - [黑客马拉松](https://wiki.polkadot.network/docs/zh-CN/build-hackathon)

#### Polkadot 全面介绍
这里对 Polkadot 进行了全面的介绍，包括关于 Polkadot 的特点：
- [Polkadot 基础](https://wiki.polkadot.network/docs/en/learn-introduction)
  - [Polkadot 架构](https://wiki.polkadot.network/docs/zh-CN/learn-architecture)
  - [网络安全](https://wiki.polkadot.network/docs/zh-CN/learn-security)
  - [共识机制](https://wiki.polkadot.network/docs/zh-CN/learn-consensus)
  - [治理](https://wiki.polkadot.network/docs/zh-CN/learn-governance)
  - [桥接方案](https://wiki.polkadot.network/docs/zh-CN/learn-bridges)
  - [平行链](https://wiki.polkadot.network/docs/zh-CN/learn-parachains)
  - [平行线程](https://wiki.polkadot.network/docs/zh-CN/learn-parathreads)
  - [Polkadot 运行时环境](https://wiki.polkadot.network/docs/zh-CN/learn-PRE)
  - [财政库](https://wiki.polkadot.network/docs/zh-CN/learn-treasury)
- 进阶
  - [抵押](https://wiki.polkadot.network/docs/zh-CN/learn-staking)
  - [随机性](https://wiki.polkadot.network/docs/zh-CN/learn-randomness)
  - [SPREE](https://wiki.polkadot.network/docs/zh-CN/learn-spree)
  - [WebAssemble(Wasm)](https://wiki.polkadot.network/docs/zh-CN/learn-wasm)
    - [WebAssembly.org](https://webassembly.org/)：WebAssembly 官网。
    - [Wasmi](https://github.com/paritytech/Wasmi)：Parity 团队使用 Rust 编写的 WebAssembly 解释器。
    - [Parity Wasm](https://github.com/paritytech/parity-Wasm)：WebAssembly 序列化 / 反序列化。
    - [Wasm 实用程序](https://github.com/paritytech/Wasm-utils)
  - [顺序弗拉格曼方法](https://wiki.polkadot.network/docs/zh-CN/learn-phragmen)
  - 跨链消息传递 (ICMP)
- 密码学相关
  - [密码学讲解](https://wiki.polkadot.network/docs/zh-CN/learn-cryptography)
  - [密钥](https://wiki.polkadot.network/docs/zh-CN/learn-keys)
- 经济系统
  - [DOT](https://wiki.polkadot.network/docs/zh-CN/learn-DOT)
  - [平行链插槽拍卖](https://wiki.polkadot.network/docs/zh-CN/learn-auction)
- 与其他方案比较
  - [Cosmos 和 Polkadot](https://wiki.polkadot.network/docs/zh-CN/learn-comparisons-cosmos)
  - [Dfinity 和 Polkadot](https://wiki.polkadot.network/docs/zh-CN/learn-comparisons-dfinit)
  - [其他](https://wiki.polkadot.network/docs/zh-CN/learn-comparisons)
- Miscellaneous
  - [路线图](https://wiki.polkadot.network/docs/zh-CN/learn-roadmap)
  - [相关链接](https://wiki.polkadot.network/docs/zh-CN/learn-relevant-links)
  - [常见问题](https://wiki.polkadot.network/docs/zh-CN/learn-faq)

#### 参与网络
