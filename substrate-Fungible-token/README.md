# 基于 Substrate 开发一条 Token 链

这个文档将手把手教你怎么从零开始使用 Substrate 框架开发一条 Token 链。顾名思义，这条链的功能就是能够在上面发行 Token。其实，类似的文档官方有一个：[substrate-collectables-workshop](https://substrate.dev/substrate-collectables-workshop/#/)，而且汉化也做得非常好，里面的逻辑参考的是曾经在以太坊上大火的 CryptoKitties。考虑到大家可能对 ERC20 Token 合约更加熟悉，因此写了这个文档，如果大家先把这个文档过了一遍，再去看官方的 substrate-collectables-workshop，就会更容易上手。

⚠️ NOTE: 本文档以及里面涉及到的代码仅作 Demo 展示用，不要将其用于生产环境。

## `Contents`
- [基于 Substrate 开发一条 Token 链](#%e5%9f%ba%e4%ba%8e-substrate-%e5%bc%80%e5%8f%91%e4%b8%80%e6%9d%a1-token-%e9%93%be)
  - [Contents](#contents)
  - [一、环境搭建](#%e4%b8%80%e7%8e%af%e5%a2%83%e6%90%ad%e5%bb%ba)
    - [1.1 Rust 环境搭建](#11-rust-%e7%8e%af%e5%a2%83%e6%90%ad%e5%bb%ba)
      - [1.1.1 更新 Ubuntu 软件源](#111-%e6%9b%b4%e6%96%b0-ubuntu-%e8%bd%af%e4%bb%b6%e6%ba%90)
      - [1.1.2 安装 Rust](#112-%e5%ae%89%e8%a3%85-rust)
      - [1.1.3 设置 Rust 源代理后再安装](#113-%e8%ae%be%e7%bd%ae-rust-%e6%ba%90%e4%bb%a3%e7%90%86%e5%90%8e%e5%86%8d%e5%ae%89%e8%a3%85)
    - [1.2 Substrate 环境搭建](#12-substrate-%e7%8e%af%e5%a2%83%e6%90%ad%e5%bb%ba)
      - [1.2.1 安装 Substrate 相关环境](#121-%e5%ae%89%e8%a3%85-substrate-%e7%9b%b8%e5%85%b3%e7%8e%af%e5%a2%83)
      - [1.2.2 Substrate 相关脚本安装](#122-substrate-%e7%9b%b8%e5%85%b3%e8%84%9a%e6%9c%ac%e5%ae%89%e8%a3%85)
      - [1.2.3 关于 apt 源问题的解决](#123-%e5%85%b3%e4%ba%8e-apt-%e6%ba%90%e9%97%ae%e9%a2%98%e7%9a%84%e8%a7%a3%e5%86%b3)
  - [二、新建一个 Substrate 项目](#%e4%ba%8c%e6%96%b0%e5%bb%ba%e4%b8%80%e4%b8%aa-substrate-%e9%a1%b9%e7%9b%ae)
      - [2.1 下载 erc20 项目所需源码](#21-%e4%b8%8b%e8%bd%bd-erc20-%e9%a1%b9%e7%9b%ae%e6%89%80%e9%9c%80%e6%ba%90%e7%a0%81)
      - [2.2 编译启动项目](#22-%e7%bc%96%e8%af%91%e5%90%af%e5%8a%a8%e9%a1%b9%e7%9b%ae)
      - [2.3 Substrate 编译时下载依赖库很慢](#23-substrate-%e7%bc%96%e8%af%91%e6%97%b6%e4%b8%8b%e8%bd%bd%e4%be%9d%e8%b5%96%e5%ba%93%e5%be%88%e6%85%a2)
      - [2.4 启动节点](#24-%e5%90%af%e5%8a%a8%e8%8a%82%e7%82%b9)

## 一、环境搭建

接下来将讲解基于 Ubuntu 16.04 搭建环境。

### 1.1 Rust 环境搭建

#### 1.1.1 更新 Ubuntu 软件源

分步骤执行
```shell
$ sudo apt update

$ sudo apt upgrade
```

#### 1.1.2 安装 Rust
参考 [在 Linux 或 macOS 上安装](https://rustlang-cn.org/office/rust/book/getting-started/ch01-01-installation.html#%E5%9C%A8-linux-%E6%88%96-macos-%E4%B8%8A%E5%AE%89%E8%A3%85-rustup)
```shell
$ curl https://sh.rustup.rs -sSf | sh
```
如果网速正常，就等它安装完就行，如果速度很慢，可以尝试下一步的设置代理：

#### 1.1.3 设置 Rust 源代理后再安装
参考 [rust rustup安装走代理](https://blog.csdn.net/bu2_int/article/details/79758960)  

用自己的编辑器打开用户环境变量文件： .bashrc。
```shell
$ emacs ~/.bashrc
```
拉到文本的最后，加上
```shell
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static 
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
```
最后效果是：
![pic](./images/sub001.PNG)

然后保存退出。并更新环境变量：
```shell
$ source ~/.bashrc
```

然后再接着安装：
```shell
$ curl https://sh.rustup.rs -sSf | sh
```
这一次马上就开始并且弹出提示：
```shell
···
You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:


   default host triple: x86_64-unknown-linux-gnu
     default toolchain: stable
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
>
```
根据提示，我们选择按照默认安装，因此输入 1，并回车。

过了一两分钟就看到提示安装好了：
```shell
···
stable installed - rustc 1.39.0 (4560ea788 2019-11-04)


Rust is installed now. Great!

To get started you need Cargo's bin directory ($HOME/.cargo/bin) in your PATH
environment variable. Next time you log in this will be done
automatically.

To configure your current shell run source $HOME/.cargo/env
```
里面已经安装了很多 Rust 的工具链，更多请参考 [介绍](https://rustlang-cn.org/office/rust/book/ch00-00-introduction.html)：
* `cargo`：Rust 的包管理工具，以及项目管理工具。
* `clippy`
* `rust-docs`：Rust 的文档
* `rust-std`：Rust 的标准库
* `rustc`：Rust 的编译器
* `rustfmt`：Rust 格式化代码的工具。

根据上面的提示，我们需要通过执行以下命令来更新一下环境变量：
```shell
$ source $HOME/.cargo/env
```

接着我们可以验证以下是否安装成功：
```shell
$ cargo --version
$ cargo 1.39.0 (1c6ec66d5 2019-09-30)
```
到现在为止，Rust 环境成功搭建。


### 1.2 Substrate 环境搭建

#### 1.2.1 安装 Substrate 相关环境
官方已经有非常贴心的[安装文档](https://substrate.dev/docs/en/getting-started/installing-substrate)，里面有包括 Fast 模式和 Full 模式。Fast 模式是会跳过 substrate 以及 subkey 的安装过程。这个过程比较花时间，而且不影响我们后续的步骤，因此我们选择 Fast 模式。相反，Full 模式就是会有那两个的安装过程。

执行：
```shell
$ curl https://getsubstrate.io -sSf | bash -s -- --fast
```
期间会安装大量的系统软件，下面是终端的 log 输出：
```shell
Reading state information... Done
build-essential is already the newest version (12.1ubuntu2).
gcc is already the newest version (4:5.3.1-1ubuntu1).
pkg-config is already the newest version (0.29.1-0ubuntu1).
git is already the newest version (1:2.7.4-0ubuntu1.7).
git set to manually installed.
The following additional packages will be installed:
  binfmt-support clang-3.8 cmake-data libclang-3.8-dev libclang-common-3.8-dev
  libclang1-3.8 libffi-dev libjsoncpp1 libobjc-5-dev libobjc4 libssl-doc
  libtinfo-dev llvm-3.8 llvm-3.8-dev llvm-3.8-runtime zlib1g-dev
Suggested packages:
  gnustep gnustep-devel clang-3.8-doc codeblocks eclipse ninja-build
  llvm-3.8-doc
The following NEW packages will be installed:
  binfmt-support clang clang-3.8 cmake cmake-data libclang-3.8-dev
  libclang-common-3.8-dev libclang-dev libclang1-3.8 libffi-dev libjsoncpp1
  libobjc-5-dev libobjc4 libssl-dev libssl-doc libtinfo-dev llvm-3.8
  llvm-3.8-dev llvm-3.8-runtime zlib1g-dev
0 upgraded, 20 newly installed, 0 to remove and 0 not upgraded.
Need to get 65.6 MB of archives.
After this operation, 441 MB of additional disk space will be used.
···
```
最后输出是：
```shell
Receiving objects: 100% (207/207), 45.21 KiB | 8.00 KiB/s, done.
Resolving deltas: 100% (115/115), done.
Checking connectivity... done.
Run source ~/.cargo/env now to update environment
```
根据提示，我们需要更新一下环境变量来让操作系统找得到刚安装的 Substrate 相关的软件。
执行：
```shell
$ source ~/.cargo/env
```
到这里，Substrate 的环境安装好了。

#### 1.2.2 Substrate 相关脚本安装

接下来我们需要安装一些脚本，方便我们以后创建一个新的 Substrate 项目。

这个官方也有[非常贴心的文档](https://substrate.dev/docs/en/getting-started/using-the-substrate-scripts)，我们就参考这个文档来：

分步执行：
```
$ f=`mktemp -d`

$ git clone https://github.com/paritytech/substrate-up $f

$ cp -a $f/substrate-* ~/.cargo/bin

$ cp -a $f/polkadot-* ~/.cargo/bin
```
执行完后我们可以看到：
```shell
$ ls ~/.cargo/bin/
cargo         clippy-driver         rustdoc    rustup
cargo-clippy  polkadot-js-apps-new  rustfmt    substrate-module-new
cargo-fmt     rls                   rust-gdb   substrate-node-new
cargo-miri    rustc                 rust-lldb  substrate-ui-new
```
这些都是一些可执行文件（程序），其中 `substrate-module-new` `substrate-node-new` `substrate-ui-new` 就是我们刚刚安装的脚本，它们分别有以下功能：
- substrate-node-new 开启一个新项目时能用得到，能够马上得到 substrate 的一个 node 模板。
- substrate-module-new 在给某个已有 Substrate 项目新增 module 时用到，Substrate 支持在 runtime 里面有多个 module 来实现不同的逻辑功能。
- substrate-ui-new 这个在创建一个前端 UI 时能用得到。

到这里，这次需要用到的 Substrate 相关的环境就搭建好了。


#### 1.2.3 关于 apt 源问题的解决
⚠️ NOTE: 如果你安装环境没有问题或者安装软件时速度不慢，就不需要执行这个标题下的内容。

第一次安装时出现了一些问题：
```shell
···
build-essential is already the newest version (12.1ubuntu2).
gcc is already the newest version (4:5.3.1-1ubuntu1).
pkg-config is already the newest version (0.29.1-0ubuntu1).
git is already the newest version (1:2.7.4-0ubuntu1.6).
git set to manually installed.
Some packages could not be installed. This may mean that you have
requested an impossible situation or if you are using the unstable
distribution that some required packages have not yet been created
or been moved out of Incoming.
The following information may help to resolve the situation:

The following packages have unmet dependencies:
 clang : Depends: clang-3.6 (>= 3.6~rc1) but it is not going to be installed
 libclang-dev : Depends: libclang-3.6-dev (>= 3.6~rc1) but it is not going to be installed
 libssl-dev : Depends: libssl1.0.0 (= 1.0.2e-1ubuntu1) but 1.0.2g-1ubuntu4.15 is to be installed
              Depends: zlib1g-dev but it is not going to be installed
              Recommends: libssl-doc but it is not going to be installed
E: Unable to correct problems, you have held broken packages.
```

网上搜索，应该是我的 Ubuntu apt 源没有及时更新导致（我之前用的是中科大的 apt 源，可能它没有及时更新），参考[这个文档](https://mirrors.tuna.tsinghua.edu.cn/help/ubuntu/)，把 apt 源换成这个。注意要选择和自己的系统对应的，比如我的是 Ubuntu 16.06，所以选择这个：
![ubuntu_apt](images/sub002.png)

具体操作是：
一、备份：
```shell
$ sudo cp sources.list sources.list.bak
```
二、用编辑器打开 apt 源的文件，清空里面的内容，将对应的 tuna 源的内容复制粘贴进去：
```shell
$ sudo emacs /etc/apt/sources.list
```
tuan 源：

```shell
deb https://mirrors.tuna.tsinghua.edu.cn/ubuntu/ xenial main restricted universe multiverse
# deb-src https://mirrors.tuna.tsinghua.edu.cn/ubuntu/ xenial main restricted universe multiverse
deb https://mirrors.tuna.tsinghua.edu.cn/ubuntu/ xenial-updates main restricted universe multiverse
# deb-src https://mirrors.tuna.tsinghua.edu.cn/ubuntu/ xenial-updates main restricted universe multiverse
deb https://mirrors.tuna.tsinghua.edu.cn/ubuntu/ xenial-backports main restricted universe multiverse
# deb-src https://mirrors.tuna.tsinghua.edu.cn/ubuntu/ xenial-backports main restricted universe multiverse
deb https://mirrors.tuna.tsinghua.edu.cn/ubuntu/ xenial-security main restricted universe multiverse
# deb-src https://mirrors.tuna.tsinghua.edu.cn/ubuntu/ xenial-security main restricted universe multiverse

# deb https://mirrors.tuna.tsinghua.edu.cn/ubuntu/ xenial-proposed main restricted universe multiverse
# deb-src https://mirrors.tuna.tsinghua.edu.cn/ubuntu/ xenial-proposed main restricted universe multiverse
```
最后结果是：
![apt](images/sub003.PNG)

然后保存退出即可。

三、更新软件：
分步执行：
```shell
$ sudo apt update

$ sudo apt upgrade
```
果然有好多软件没有更新：
```shell
Reading package lists... Done
Building dependency tree       
Reading state information... Done
Calculating upgrade... Done
The following NEW packages will be installed:
  linux-headers-4.15.0-72 linux-headers-4.15.0-72-generic
  linux-image-4.15.0-72-generic linux-modules-4.15.0-72-generic
  linux-modules-extra-4.15.0-72-generic
The following packages will be upgraded:
  amd64-microcode apport apport-gtk cpio cpp-5 dbus dbus-x11 file firefox
  firefox-locale-en g++-5 gcc-5 gcc-5-base ghostscript ghostscript-x git
  git-man gnome-software gnome-software-common grub-common grub-pc grub-pc-bin
  grub2-common imagemagick imagemagick-6.q16 imagemagick-common
  initramfs-tools initramfs-tools-bin initramfs-tools-core intel-microcode
  libarchive13 libasan2 libatomic1 libcc1-0 libcilkrts5 libdbus-1-3
  libdjvulibre-text libdjvulibre21 libgcc-5-dev libgomp1 libgs9 libgs9-common
  libitm1 libjpeg-turbo8 liblsan0 libmagic1 libmagickcore-6.q16-2
  libmagickcore-6.q16-2-extra libmagickwand-6.q16-2 libmpx0 libnss3
  libnss3-nssdb libpam-systemd libpcap0.8 libquadmath0 libsmbclient
  libsqlite3-0 libssh-4 libstdc++-5-dev libstdc++6 libsystemd0 libtsan0
  libubsan0 libudev1 libvpx3 libwbclient0 libwhoopsie0 libxslt1.1
  linux-generic-hwe-16.04 linux-headers-generic-hwe-16.04
  linux-image-generic-hwe-16.04 linux-libc-dev python3-apport
  python3-distupgrade python3-problem-report python3-update-manager samba-libs
  systemd systemd-sysv ubuntu-release-upgrader-core
  ubuntu-release-upgrader-gtk ubuntu-software udev unattended-upgrades
  update-manager update-manager-core whoopsie
87 upgraded, 5 newly installed, 0 to remove and 0 not upgraded.
Need to get 185 MB of archives.
After this operation, 342 MB of additional disk space will be used.
Do you want to continue? [Y/n] Y
Get:1 https://mirrors.tuna.tsinghua.edu.cn/ubuntu xenial-updates/main amd64 dbus-x11 amd64 1.10.6-1ubuntu3.5 [21.5 kB]
···
```
接下来可以回 `1.2.1 安装 Substrate 相关环境` 再次执行那个命令了。

## 二、新建一个 Substrate 项目
#### 2.1 下载 erc20 项目所需源码
为了目录的整洁性，新建一个目录并进入：
```shell
$ mkdir polkadot

$ cd polkadot
```

substrate-node-new 的语法是：`Usage: substrate-node-new <NAME> <AUTHOR>`。作者 AUTHOR 是任意的，下面我的演示就是用 flyq，看者可以使用自己的。项目名 NAME 我们统一使用 erc20 。

使用 substrate-node-new 创建一个名为 erc20 的 substrate 项目：  
执行后脚本会从 GitHub 下载对应的源代码包，需要等待下载。
```shell
$ substrate-node-new erc20 flyq 

  Substrate Node Template Setup 
  Downloading project...
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100 71537  100 71537    0     0   2592      0  0:00:27  0:00:27 --:--:--  5026
Customizing project...
Initializing repository...

$ ls
erc20
```

可以看出仅仅用了半分钟就下载好了，对应目录下多出了一个文件夹 `erc20/`，这个就是我们项目文件夹。

PS：有时候会卡很久：
![down](images/sub004.PNG)
比如上面就是我们卡了一个半小时了，我直接 CTRL+c 中断了，然后删掉那个没下载完整的： `sudo rm -r substrate-node-template/`，重新试了一遍，就好了。

#### 2.2 编译启动项目

因为我们的这个脚本 `substrate-node-new` 是安装的 [Substrate 1.0 版本](https://github.com/paritytech/substrate-up/blob/master/substrate-node-new#L44) 的，因此启动项目之前还需要先编译 wasm: `./scripts/build.sh` 。如果是 [Substrate 2.0 版本](https://gist.github.com/flyq/e2471d5f80d4b61e6e7b101a0a36429d#file-substrate-node-new-2-0-L42)，下面 `./scripts/build.sh ` 步骤就可以省略。官方正式的 2.0 版本还没有发布。
下面步骤终端有 log 输出的我贴出部分，方便对比。
```shell
$ cd erc20/

$ ./scripts/init.sh
*** Initializing WASM build environment
info: syncing channel updates for 'nightly-x86_64-unknown-linux-gnu'

  nightly-x86_64-unknown-linux-gnu unchanged - rustc 1.41.0-nightly (3ed3b8bb7 2019-12-17)
···
···
Compiling wasm-gc v0.1.6 (https://github.com/alexcrichton/wasm-gc#deb1c6dc)
    Finished release [optimized] target(s) in 1m 24s
  Installing /home/flyq/.cargo/bin/wasm-gc
   Installed package `wasm-gc v0.1.6 (https://github.com/alexcrichton/wasm-gc#deb1c6dc)` (executable `wasm-gc`)
```
上面 init.sh 脚本为了更新 Rust Toolchain，以及下载安装 Wasm-gc 相关工具。

接着我们执行：
```shell
$ ./scripts/build.sh 
Building webassembly binary in runtime/wasm...
    Updating git repository `https://github.com/paritytech/substrate.git`
···
···
warning: the feature `alloc` has been stable since 1.36.0 and no longer requires an attribute to enable
 --> /home/flyq/workspace/polkadot/erc20/runtime/src/lib.rs:4:43
  |
4 | #![cfg_attr(not(feature = "std"), feature(alloc))]
  |                                           ^^^^^
  |
  = note: `#[warn(stable_features)]` on by default

   Compiling erc20-runtime-wasm v1.0.0 (/home/flyq/workspace/polkadot/erc20/runtime/wasm)
    Finished release [optimized] target(s) in 17m 09s
```
上面为了编译 Substrate 链里面的 Wasm，因为 Substrate 为了升级时不需要硬分叉（想一下比特币或者以太坊升级时的麻烦吧），链上维护了一个 Wasm 形式的链的逻辑。这样如果整个链需要升级，在通过投票等治理环节后，就会上传一份新的 Wasm 代码。那些同步升级了的节点，他们的本地二进制形式的逻辑和链上 Wasm 形式的逻辑相同，就执行二进制逻辑（这样比执行 Wasm 效率更高），而那些暂时还没有同步更新的节点，他们会执行从链上同步过来的 Wasm 形式的逻辑代码，直到节点把二进制形式的程序也成功升级，这样在整个链的升级过程中，避免了节点的分叉。



接下来我们编译链的二进制程序：
```shell
$ cargo build --release
  Downloaded ctrlc v3.1.3 (registry `git://mirrors.ustc.edu.cn/crates.io-index`)
  Downloaded serde_json v1.0.40 (registry `git://mirrors.ustc.edu.cn/crates.io-index`)
  Downloaded slog-async v2.3.0 (registry `git://mirrors.ustc.edu.cn/crates.io-index`)
···
···
  Compiling substrate-service v1.0.0 (https://github.com/paritytech/    substrate.git?rev=cc1d67e973fd02c0c997b164ba516cf041bf21f1#cc1d67e9)
  Compiling substrate-cli v1.0.0 (https://github.com/paritytech/substrate.git?rev=cc1d67e973fd02c0c997b164ba516cf041bf21f1#cc1d67e9)
   Finished release [optimized] target(s) in 21m 23s
```
上面我们花了 21 分钟编译完成。这个取决于网速和电脑 CPU 性能。

这是可以看到多出了一个 target/ 文件夹，这里面包含了我们的编译结果：
```shell
$ ls
build.rs    Cargo.toml  README.md  scripts  target
Cargo.lock  LICENSE     runtime    src
```

#### 2.3 Substrate 编译时下载依赖库很慢

⚠️ NOTE: 如果编译时网速不慢，没有下载时卡死等，就不需要执行这个标题下的内容。

在编译 Substrate 的时候，需要下载很多 Rust 库，部分库的下载地址在 GitHub 上，要是出现无法下载，可以参考[这篇文章](https://blog.csdn.net/xiangxianghehe/article/details/53471936)，把它换成国内中科大源就行，最后的效果就是在 `~/.cargo/` 文件夹下新建了一个 config 文件，里面内容是：：
![git_mirror](images/sub005.PNG)


#### 2.4 启动节点

```shell
$ ./target/release/erc20 --dev
2019-12-18 18:23:43 Substrate Node
2019-12-18 18:23:43   version 1.0.0-x86_64-linux-gnu
2019-12-18 18:23:43   by flyq, 2017, 2018
2019-12-18 18:23:43 Chain specification: Development
2019-12-18 18:23:43 Node name: scary-limit-0230
2019-12-18 18:23:43 Roles: AUTHORITY
2019-12-18 18:23:43 Initializing Genesis block/state (state: 0xa5fd…b5af, header-hash: 0x7606…b33f)
2019-12-18 18:23:43 Loaded block-time = 10 seconds from genesis on first-launch
···
···
```
上面可以看到，这个条链正常启动，而且 log 输出有很多提示信息，包括版本，出块时间等。

这时我们可以打开和节点运行在同一个操作系统里面的浏览器，输入网址：`https://polkadot.js.org/apps/#/settings`，将链接的链改为 `Local Node (Own, 127.0.0.1:9944)`，最后点击 `Save & Reload`：
![firefox](images/sub006.png)

这个是 Substrate 链配套的 UI，包括了浏览器的功能，钱包的功能。我们可以通过 `substrate-ui-new` 工具自己起一个，为了方便这里直接用了 https://polkadot.js.org/apps 的，其效果是相同的。

到这里，我们成功将一条链运行起来了，并且还有了对应的浏览器以及钱包：
![all](images/sub007.PNG)

下面是它的钱包功能，系统自动生成了一系列的初始账号，并且给 ALICE 账号预分配了 1.151M 的 DEV，刚刚我给 BOB 账号转了 1000 DEV。可以看到，左侧栏里面还有很多其他功能，包括：
- 转账（transfer），
- 链上状态查询（Chain state），
- 发起交易包括调用函数之类的（Extrinsics），
- 超级权限（Sudo，一般用于链刚刚启动时还不稳定的时候，后续就交给治理了），
- 设置（Settings）等等。


