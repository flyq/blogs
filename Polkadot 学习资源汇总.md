# Polkadot 学习资源汇总
这里将实时收集汇总 Polkadot 相关的学习资源，包括 Polkadot 本身以及 Substrate 框架。
## `Contents`
- [Polkadot 学习资源汇总](#polkadot-%e5%ad%a6%e4%b9%a0%e8%b5%84%e6%ba%90%e6%b1%87%e6%80%bb)
  - [`Contents`](#contents)
  - [Substrate 是什么](#substrate-%e6%98%af%e4%bb%80%e4%b9%88)
  - [官网](#%e5%ae%98%e7%bd%91)
  - [导航类资源](#%e5%af%bc%e8%88%aa%e7%b1%bb%e8%b5%84%e6%ba%90)
    - [awesome-substrate](#awesome-substrate)
    - [substrate.dev](#substratedev)
    - [PolkaWorld 波卡世界](#polkaworld-%e6%b3%a2%e5%8d%a1%e4%b8%96%e7%95%8c)
    - [Boka.Network](#bokanetwork)
  - [主页类资源](#%e4%b8%bb%e9%a1%b5%e7%b1%bb%e8%b5%84%e6%ba%90)
    - [Polkadot Wiki](#polkadot-wiki)
      - [Polkadot 综合性资源](#polkadot-%e7%bb%bc%e5%90%88%e6%80%a7%e8%b5%84%e6%ba%90)
      - [基于 Polkadot 开发](#%e5%9f%ba%e4%ba%8e-polkadot-%e5%bc%80%e5%8f%91)
      - [Polkadot 全面介绍](#polkadot-%e5%85%a8%e9%9d%a2%e4%bb%8b%e7%bb%8d)
      - [参与维护网络](#%e5%8f%82%e4%b8%8e%e7%bb%b4%e6%8a%a4%e7%bd%91%e7%bb%9c)
    - [Web3 Foundation Research](#web3-foundation-research)
      - [Polkadot 网络协议](#polkadot-%e7%bd%91%e7%bb%9c%e5%8d%8f%e8%ae%ae)
      - [Web3 研究团队成员](#web3-%e7%a0%94%e7%a9%b6%e5%9b%a2%e9%98%9f%e6%88%90%e5%91%98)
      - [Web3 基金会新闻](#web3-%e5%9f%ba%e9%87%91%e4%bc%9a%e6%96%b0%e9%97%bb)


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

## 官网
[Polkadot 官网](https://polkadot.network/)  
[Parity 官网](https://www.parity.io/)  
[Web3 基金会官网](https://web3.foundation/)  

[Parity 官方 GitHub 地址](https://github.com/paritytech/)  
[Polkadot GitHub 地址](https://github.com/paritytech/polkadot)  
[Substrate GitHub 地址](https://github.com/paritytech/substrate)  
[Substrate 开发者中心 GitHub 地址](https://github.com/substrate-developer-hub)  
[Web3 基金会 GitHub 地址](https://github.com/w3f)  

[Polkadot 官方白皮书地址](https://polkadot.network/PolkaDotPaper.pdf)

[Web3 基金会论坛](https://forum.web3.foundation/)  
[官方维护的 Riot 群](##polkadot-%e7%bb%bc%e5%90%88%e6%80%a7%e8%b5%84%e6%ba%90)  
[Polkadot 官方 Medium](https://medium.com/@polkadotnetwork)  
[Polkadot 官方 Reddit](https://www.reddit.com/r/dot)  
[Polkadot 官方 Twitter](https://twitter.com/polkadotnetwork)  
[Polkadot 官方 Telegram](https://t.me/PolkadotOfficial)  
[Polkadot 官方 Youtube](https://www.youtube.com/channel/UCB7PbjuZLEba_znc7mEGNgw)  
[订阅 Polkadot 邮件](https://foundation.us16.list-manage.com/subscribe/post?u=a30a54feb730c7553e5d2eb0c&id=93eced75b7)


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

### PolkaWorld 波卡世界
https://www.polkaworld.org/explore

`标签`：`社区` `中文`

波卡世界是一个中文社区，有[网站](https://www.polkaworld.org/)以及公众号（搜索 `PolkaWorld`）以及对应的微信群。其公众号能及时发布关于 Polkadot 的最新公告，翻译发布一些 Polkadot 文章，定期会组织一些活动等。

### Boka.Network
http://blog.boka.network/links/

`标签`：`导航` `中文`

功能和波卡世界类似。

## 主页类资源
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

#### 参与维护网络
- [收集人](https://wiki.polkadot.network/docs/zh-CN/maintain-collator)
- [提名人](https://wiki.polkadot.network/docs/zh-CN/maintain-nominator)
  - [成为提名人](https://wiki.polkadot.network/docs/zh-CN/maintain-guides-how-to-nominate-alexander)
  - [提名人指南](https://wiki.polkadot.network/docs/zh-CN/maintain-guides-how-to-nominate-kusama)
- 验证人
  - 指南
    - [如何在 Alexander 运行节点](https://wiki.polkadot.network/docs/zh-CN/maintain-guides-how-to-validate-alexander)：逐步介绍如何在 Aexander 测试网上设置验证人的指南。
    - [如何在 Kusama 运行节点](https://wiki.polkadot.network/docs/zh-CN/maintain-guides-how-to-validate-kusama)：逐步介绍如何在 Kusama 金丝雀网络上设置验证人的指南。
    - [验证人奖励发放概述](https://wiki.polkadot.network/docs/zh-CN/maintain-guides-validator-payout)：简短概述验证人奖励发放机制原理。
    - [如何将验证人程序作为 systemd 进程运行](https://wiki.polkadot.network/docs/zh-CN/maintain-guides-how-to-systemd)：有关将验证人程序作为systemd进程运行的指南 ，使它在背后运行，并当重新启动时自动启动。
    - [如何升级您的验证人](https://wiki.polkadot.network/docs/zh-CN/maintain-guides-how-to-upgrade)：当要转换到另一台计算机或运行最新版本的客户端时 - 升级验证人指南。
    - [如何设置哨兵节点](https://wiki.polkadot.network/docs/zh-CN/maintain-guides-how-to-setup-sentry-node)：为验证人设置哨兵节点。
  - 其他参考
    - [如何运行 Polkadot 节点 (Docker)](https://medium.com/@acvlls/setting-up-a-maintain-the-easy-way-3a885283091f)
    - [Web 3.0 验证人节点的无服务器故障转移解决方案](https://medium.com/hackernoon/a-serverless-failover-solution-for-web-3-0-validator-nodes-e26b9d24c71d)：博客详细介绍了如何创建可靠的故障转移解决方案运行验证人。
  - [获取测试网 DOTs](https://wiki.polkadot.network/docs/zh-CN/learn-DOT#getting-testnet-dots)
  - [云服务器清单](https://wiki.polkadot.network/docs/zh-CN/maintain-guides-how-to-validate-kusama#vps-list)
  - [Polkadot 验证人休息室](https://matrix.to/#/!NZrbtteFeqYKCUGQtr:matrix.parity.io?via=matrix.parity.io&via=matrix.org&via=web3.foundation)：验证人聊天室。
  - 安全/密钥管理
    - [验证人安全概览](https://github.com/w3f/validator-security)
  - 监控工具
    - [Polkadot Telemetry 服务](https://telemetry.polkadot.io/#/Alexander)：网络信息，包括在某一条链上有什么节点在运行，正在运行的版本以及同步状态。
    - [Polkadash](http://polkadash.io/)：监测验证人
    - [其它有用链接](https://forum.web3.foundation/t/useful-links-for-validators/20)
- Governance
  - [加入议会](https://wiki.polkadot.network/docs/zh-CN/maintain-guides-how-to-join-council)
  - [投票选举议员](https://wiki.polkadot.network/docs/zh-CN/maintain-guides-how-to-vote-councillor)

### Web3 Foundation Research
https://research.web3.foundation/en/latest/

`标签`：`官方` `研究`

研究领域：
- 去中心化算法：共识，优化，博弈论
- 密码学
- 网络

目前研究：
- [Polkadot network protocol research](https://research.web3.foundation/en/latest/polkadot.html)
- [Messaging for Web3 initiative](https://github.com/w3f/messaging)

联系方式：
- Riot: #w3f:matrix.org 
- [forum](https://forum.web3.foundation/)

#### Polkadot 网络协议
- [Polkadot](https://research.web3.foundation/en/latest/polkadot/index.html)
  1. [Specification of the Polkadot Runtime Environment](https://github.com/w3f/polkadot-re-spec)
  2. Identifying participants to run the network
     1. Keys
     2. Proof-of-Stake
     3. Why not use different sets for different tasks?
  3. Ensuring state transition properties
     1. Utility
     2. Validity
     3. Finality
     4. Availability
     5. Messaging reliability
     6. Size
     7. Bandwidth
  4. Desired architectural qualities

- [可用性与有效性（Availability and Validity）](https://research.web3.foundation/en/latest/polkadot/Availability_and_Validity.html)
- [GRANDPA 共识](https://research.web3.foundation/en/latest/polkadot/GRANDPA.html)
- [平行链分配（Parachain Allocation）](https://research.web3.foundation/en/latest/polkadot/Parachain-Allocation.html)
- [Polkadot 运行时环境规范（Polkadot Runtime Environment Specification）](https://research.web3.foundation/en/latest/polkadot/Polkadot-Runtime-Environment.html)
- [代币经济（Token Economics）](https://research.web3.foundation/en/latest/polkadot/Token%20Economics.html)
- [BABE 共识算法](https://research.web3.foundation/en/latest/polkadot/BABE.html)
  - BABE
    1. Overview
    2. BABE
    3. Best Chain Selection
    4. Relative Time
    5. Security Analysis
  - Sortion
- [NPoS 提名权益证明](https://research.web3.foundation/en/latest/polkadot/NPoS.html)
  - [Intro to Nominated Proof-of-Stake](https://research.web3.foundation/en/latest/polkadot/NPoS/index.html#)
  1. [Overview of results for the NPoS election problem](https://research.web3.foundation/en/latest/polkadot/NPoS/1.%20Overview.html)
  2. [The min-norm max-flow problem](https://research.web3.foundation/en/latest/polkadot/NPoS/1.%20Overview.html)
  3. [The maximin support problem](https://research.web3.foundation/en/latest/polkadot/NPoS/3.%20The%20maximin%20support%20problem.html)
  4. [Sequential Phragmén’s method.](https://research.web3.foundation/en/latest/polkadot/NPoS/4.%20Sequential%20Phragm%C3%A9n%E2%80%99s%20method.html)
  5. [A Phragmén-like Heuristic](https://research.web3.foundation/en/latest/polkadot/NPoS/5.%20A%20Phragm%C3%A9n-like%20Heuristic.html)
- [账号体系 Keys](https://research.web3.foundation/en/latest/polkadot/keys.html)
- [Networking](https://research.web3.foundation/en/latest/polkadot/networking.html)
- [Slashing](https://research.web3.foundation/en/latest/polkadot/slashing.html)
- [XCMP Scheme](https://research.web3.foundation/en/latest/polkadot/XCMP.html)

#### Web3 研究团队成员
- [Sergey Vasilyev](https://research.web3.foundation/en/latest/research_team_members/Sergey.html)
- [Ximin Luo](https://research.web3.foundation/en/latest/research_team_members/Ximin.html)
- [Alfonso Cevallos](https://research.web3.foundation/en/latest/research_team_members/alfonso.html)
- [Alistair Stewart](https://research.web3.foundation/en/latest/research_team_members/alistair.html)
- [Fatemeh Shirazi](https://research.web3.foundation/en/latest/research_team_members/fatemeh.html)
- [Handan Kılınç Alper](https://research.web3.foundation/en/latest/research_team_members/handan.html)
- [Jeff Burdges](https://research.web3.foundation/en/latest/research_team_members/jeff.html)
- [Syed Hosseini](https://research.web3.foundation/en/latest/research_team_members/syed.html)

#### Web3 基金会新闻
https://research.web3.foundation/en/latest/news.html