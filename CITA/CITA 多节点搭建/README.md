# CITA 多节点环境搭建手册

## contents
- [CITA 多节点环境搭建手册](#cita-%e5%a4%9a%e8%8a%82%e7%82%b9%e7%8e%af%e5%a2%83%e6%90%ad%e5%bb%ba%e6%89%8b%e5%86%8c)
  - [contents](#contents)
  - [特性](#%e7%89%b9%e6%80%a7)
  - [配置步骤](#%e9%85%8d%e7%bd%ae%e6%ad%a5%e9%aa%a4)
    - [一、安装对应环境](#%e4%b8%80%e5%ae%89%e8%a3%85%e5%af%b9%e5%ba%94%e7%8e%af%e5%a2%83)
      - [登录服务器](#%e7%99%bb%e5%bd%95%e6%9c%8d%e5%8a%a1%e5%99%a8)
      - [更新软件源](#%e6%9b%b4%e6%96%b0%e8%bd%af%e4%bb%b6%e6%ba%90)
      - [安装趁手工具](#%e5%ae%89%e8%a3%85%e8%b6%81%e6%89%8b%e5%b7%a5%e5%85%b7)
      - [安装 CITA 环境](#%e5%ae%89%e8%a3%85-cita-%e7%8e%af%e5%a2%83)
        - [安装 Docker 环境（所有节点服务器需要安装）](#%e5%ae%89%e8%a3%85-docker-%e7%8e%af%e5%a2%83%e6%89%80%e6%9c%89%e8%8a%82%e7%82%b9%e6%9c%8d%e5%8a%a1%e5%99%a8%e9%9c%80%e8%a6%81%e5%ae%89%e8%a3%85)
        - [安装 Docker-compose （只有节点0，也就是运行工具链的服务器需要安装）](#%e5%ae%89%e8%a3%85-docker-compose-%e5%8f%aa%e6%9c%89%e8%8a%82%e7%82%b90%e4%b9%9f%e5%b0%b1%e6%98%af%e8%bf%90%e8%a1%8c%e5%b7%a5%e5%85%b7%e9%93%be%e7%9a%84%e6%9c%8d%e5%8a%a1%e5%99%a8%e9%9c%80%e8%a6%81%e5%ae%89%e8%a3%85)
        - [安装客户端工具：CITA-CLI （所有节点服务器需要安装）](#%e5%ae%89%e8%a3%85%e5%ae%a2%e6%88%b7%e7%ab%af%e5%b7%a5%e5%85%b7cita-cli-%e6%89%80%e6%9c%89%e8%8a%82%e7%82%b9%e6%9c%8d%e5%8a%a1%e5%99%a8%e9%9c%80%e8%a6%81%e5%ae%89%e8%a3%85)
        - [下载 CITA 链的程序 （一般来说节点服务器不需要安装，一般是本地生成后通过 scp 统一上传到服务器。）](#%e4%b8%8b%e8%bd%bd-cita-%e9%93%be%e7%9a%84%e7%a8%8b%e5%ba%8f-%e4%b8%80%e8%88%ac%e6%9d%a5%e8%af%b4%e8%8a%82%e7%82%b9%e6%9c%8d%e5%8a%a1%e5%99%a8%e4%b8%8d%e9%9c%80%e8%a6%81%e5%ae%89%e8%a3%85%e4%b8%80%e8%88%ac%e6%98%af%e6%9c%ac%e5%9c%b0%e7%94%9f%e6%88%90%e5%90%8e%e9%80%9a%e8%bf%87-scp-%e7%bb%9f%e4%b8%80%e4%b8%8a%e4%bc%a0%e5%88%b0%e6%9c%8d%e5%8a%a1%e5%99%a8)
    - [二、定制配置](#%e4%ba%8c%e5%ae%9a%e5%88%b6%e9%85%8d%e7%bd%ae)
      - [概述](#%e6%a6%82%e8%bf%b0)
      - [具体配置](#%e5%85%b7%e4%bd%93%e9%85%8d%e7%bd%ae)
    - [三、初始化并启动链](#%e4%b8%89%e5%88%9d%e5%a7%8b%e5%8c%96%e5%b9%b6%e5%90%af%e5%8a%a8%e9%93%be)
      - [1. 在本地初始化链](#1-%e5%9c%a8%e6%9c%ac%e5%9c%b0%e5%88%9d%e5%a7%8b%e5%8c%96%e9%93%be)
      - [2. 在 2 台服务器上创建目录](#2-%e5%9c%a8-2-%e5%8f%b0%e6%9c%8d%e5%8a%a1%e5%99%a8%e4%b8%8a%e5%88%9b%e5%bb%ba%e7%9b%ae%e5%bd%95)
      - [3. 将生成的节点拷贝到所有主机](#3-%e5%b0%86%e7%94%9f%e6%88%90%e7%9a%84%e8%8a%82%e7%82%b9%e6%8b%b7%e8%b4%9d%e5%88%b0%e6%89%80%e6%9c%89%e4%b8%bb%e6%9c%ba)
      - [4. 依次启动节点0，节点1](#4-%e4%be%9d%e6%ac%a1%e5%90%af%e5%8a%a8%e8%8a%82%e7%82%b90%e8%8a%82%e7%82%b91)
    - [四、配置工具链，Demo](#%e5%9b%9b%e9%85%8d%e7%bd%ae%e5%b7%a5%e5%85%b7%e9%93%bedemo)
      - [1. 安装需要的开发工具](#1-%e5%ae%89%e8%a3%85%e9%9c%80%e8%a6%81%e7%9a%84%e5%bc%80%e5%8f%91%e5%b7%a5%e5%85%b7)
      - [2. 安装 rebirth 缓存服务器](#2-%e5%ae%89%e8%a3%85-rebirth-%e7%bc%93%e5%ad%98%e6%9c%8d%e5%8a%a1%e5%99%a8)
      - [3. 安装区块链浏览器](#3-%e5%ae%89%e8%a3%85%e5%8c%ba%e5%9d%97%e9%93%be%e6%b5%8f%e8%a7%88%e5%99%a8)
      - [4. 安装移动端钱包](#4-%e5%ae%89%e8%a3%85%e7%a7%bb%e5%8a%a8%e7%ab%af%e9%92%b1%e5%8c%85)
      - [5. 安装 First-forever-demo](#5-%e5%ae%89%e8%a3%85-first-forever-demo)


## 特性
节点分为普通节点，共识节点。均可以增加。所有节点最多 256 个。
很多特性可以通过链级配置设置出来

## 配置步骤
### 一、安装对应环境
#### 登录服务器
```shell
$ ssh XXX@IP_address
```
其中 `XXX` 代表用户名，`IP_address` 代表要登陆的服务器的 IP 地址。

一般刚登录就会提示一些系统信息，可以得到是什么操作系统以及对应的发行版本。如果没有：
```shell
$ cat /proc/version
```
通过这个可以得到发行版本，从而根据不同的发行版执行不同的命令。下面因为使用的是 Ubuntu 发行版，就用 Ubuntu 为例子。

#### 更新软件源
依次执行：
```shell
$ sudo apt update

$ sudo apt upgrade
```
更新软件的时候，会有个确认：是否更新软件，输入 `Y` 再回车确认即可。

#### 安装趁手工具
个人习惯使用 emacs, byobu。先安装它们。
```shell
sudo apt install emacs 
```
byobu 的安装配置参考[这里](https://cloud.tencent.com/developer/article/1350307)

```shell
sudo apt install byobu
```

#### 安装 CITA 环境

所有将会运行节点的服务器都需要安装环境。但是为了保证环境的一致性，一般先在某个地方生成链级配置生成节点配置文件，然后分别用 scp 传到各个节点服务器。

##### 安装 Docker 环境（所有节点服务器需要安装）

以下命令按条执行：
```shell
$ sudo apt install docker.io

$ sudo systemctl start docker

$ sudo systemctl enable docker

$ docker --version
```
最后一步计算机输出:
```shell
Docker version 18.09.7, build 2d0083d
```
运行来检测权限:
```shell
docker images
```
如果报错：
```shell
Got permission denied while trying to connect to the Docker daemon socket at unix:///var/run/docker.sock: Get http://%2Fvar%2Frun%2Fdocker.sock/v1.39/images/json: dial unix /var/run/docker.sock: connect: permission denied
```
参考：https://stackoverflow.com/questions/48957195/how-to-fix-docker-got-permission-denied-issue  
可以通过依次运行：
```shell
sudo groupadd docker

sudo  usermod -aG docker $USER
```
来解决。
退出终端并重新进入：
1. 退出终端：`exit`
2. 重新进入：` ssh XXX@IP_address`


再次执行:
```shell
docker images
```

发现输出是这样：
```shell
REPOSITORY          TAG                 IMAGE ID            CREATED             SIZE
```
说明 Docker 成功安装。

##### 安装 Docker-compose （只有节点0，也就是运行工具链的服务器需要安装）
依次执行下面命令
```shell
$ sudo apt install docker-compose

$ docker-compose --version
```

最后将输出：
```shell
docker-compose version 1.17.1, build unknown
```

##### 安装客户端工具：CITA-CLI （所有节点服务器需要安装）
我们把整个从 CITA 下载的客户端等程序，以及后续区块链的数据等，都放在 `/data/cita` 文件夹下。

1. 创建目录
```shell
$ mkdir -p /data/cita
```

2. 进入到该目录
```shell
$ cd /data/cita/
```

3. 下载 `CITA-CLI` 安装包，它可以给后续的 CITA 链发送命令行指令来操作链。
```shell
$ wget https://github.com/cryptape/cita-cli/releases/download/0.19.6/cita-cli-x86_64-musl-tls-0.19.6.tar.gz
```

4. 解压压缩包，得到 CITA-CLI 程序。
```shell
$ tar zxvf cita-cli-x86_64-musl-tls-0.19.6.tar.gz
```
可以通过 `ls` 来查看结果，发现当前目录下(`/data/cita/`)已经有两项，一项是下载的压缩包，一项是解压出来的 cita-cli 程序：
```shell
cita-cli  cita-cli-x86_64-musl-tls-0.19.6.tar.gz
```
查看 cita-cli 的版本以检测它是否能正常运行：
```shell
$ ./cita-cli --version
cita-cli 0.19.6-2-g6167b75 2019-05-22, use rustls
```
一切正常。

5. 复制 CITA-CLI 到 系统可执行文件目录下
```shell
$ sudo cp -rp cita-cli /usr/local/bin/
```

##### 下载 CITA 链的程序 （一般来说节点服务器不需要安装，一般是本地生成后通过 scp 统一上传到服务器。）
之前尝试在某个服务器根据链级配置生成各节点配置，发现从服务器 A 无法传到服务器 B，因为登录服务用的是 ssh 密钥认证，只有本地有一份。到时还需要 scp 复制到本地，再复制到 B，还不如干脆本地生成。

**因此，下面的命令都是再本地 Ubuntu 计算机里面执行的：**

1. 切换目录
```shell
$ cd /data/cita/
```

2. 下载 CITA 安装包
```shell
$ wget https://github.com/cryptape/cita/releases/download/v1.0.0/cita_secp256k1_sha3.tar.gz
```

3. 解压 CITA 程序
```shell
$ tar zxvf cita_secp256k1_sha3.tar.gz
```

最后得到 `cita_secp256k1_sha3` 文件夹，里面有三个文件夹：
```shell
$ ls cita_secp256k1_sha3/
bin  resource  scripts
```


到这里，CITA 的环境还没完全配置好，还需要等下在本地进行完链级配置后再把配置文件，系统合约，可执行文件传到各服务器。

### 二、定制配置
*这次启动比较着急，而且也只是测试，场景需要不是特别明确，并没有很细节地去定制*
#### 概述
具体参考 CITA 文档，以及 [链级配置](./链级配置.md)

总的来讲，CITA 链分三类配置：
* 链级配置：主要是区块链本身的一些选择；
* 微服务配置：各个模块，比如授权合约，共识，Executor，Chain 等模块的配置
* 算法配置：三个场景加密算法选择，包括国密标准。

大部分重要配置需要提前确定，后续不可更改。

后续可变更的：
1. 底层链的版本，这个可以升级，可以不同版本节点同时维护一个链。
2. 全网链的版本升级，这个需要紧急制定，然后需要超级管理员调用系统合约。
3. 链的名称，操作者的昵称（不是地址），操作者网站。
4. 增删普通节点，增删共识节点。
5. 超级管理员的地址
6. [打补丁]数据修订，即可修改任意账户的余额，任意非系统合约的逻辑，部分KV数据库里面的内容。

配置方式：
1. 启动时以参数输入链级配置
2. 配置文件输入部分链级配置 init_data.yml
3. 配置文件输入微服务配置 *.toml

#### 具体配置
2个共识节点，0 个普通节点。其中一台服务器配置 1 个节点，一台服务器配置 1 个节点 + 所有的工具链以及 DEMO。

出块时间设置 3 s，即默认值

quata 模式，即发起交易不需要 Gas 费，只要使用的资源不超过某设定值即可。超过将会执行失败。



### 三、初始化并启动链

参考[官方文档](https://docs.citahub.com/zh-CN/cita/install#%E9%83%A8%E7%BD%B2-cita)的分布式部署。

#### 1. 在本地初始化链
生成个节点对应文件：
```shell
bin/cita create --super_admin "[Super Admin 地址的私钥]" --nodes "[节点0的服务器IP]:4000,[节点1的服务器IP]:4000"
```

#### 2. 在 2 台服务器上创建目录
```shell
$ mkdir -p /data/cita/
```

#### 3. 将生成的节点拷贝到所有主机
```shell
$ scp -r cita_secp256k1_sha3/* [节点0IP地址]:/data/cita/
$ scp -r cita_secp256k1_sha3/* [节点1IP地址]:/data/cita/
```

#### 4. 依次启动节点0，节点1
以节点 0 为例：
依次执行：
```shell
$ ssh [节点0用户名]@[节点0IP地址]
$ cd /data/cita/
$ ./bin/cita setup test-chain/0
$ ./bin/cita start test-chain/0
```

如果在执行 setup 时报错：
```shell
$ bin/cita setup test-chain/0
shell-init: error retrieving current directory: getcwd: cannot access parent directories: No such file or directory
error: exec: "bin/cita": stat bin/cita: no such file or directory
```

则因为 cita 的那个 docker 容器还在运行。
通过 `docker ps` 找到 `cita/cita-run` 对应的 containerID，然后 stop，rm 关闭删掉对应的容器即可。

最后 setup 是的正常 log 输出是这样的：
```shell
$ ./bin/cita setup test-chain/0
Start docker container cita_run_container ...
WARNING: Published ports are discarded when using host network mode
a3ab1bded784fd62cf7b671c8a2eac3439209a95e92b34e2989ce687ab38a4b0
sudo: unable to resolve host ip-10-0-48-21
sudo: unable to resolve host ip-10-0-48-21
sudo: unable to resolve host ip-10-0-48-21
sudo: unable to resolve host ip-10-0-48-21
sudo: unable to resolve host ip-10-0-48-21
sudo: unable to resolve host ip-10-0-48-21
sudo: unable to resolve host ip-10-0-48-21
sudo: unable to resolve host ip-10-0-48-21
sudo: unable to resolve host ip-10-0-48-21
```

检测两个节点都是 7 个服务，正常后。查看出块。出块正常后，开始运行工具链。工具链一般运行在节点 0 上。


### 四、配置工具链，Demo

工具链一般运行在节点 0 上。因此下面的操作只需要在节点 0 上执行

#### 1. 安装需要的开发工具

工具链需要安装 git，make，node 等开发工具。其中 git，make 可以直接用 apt 安装：依次执行：

```shell
$ sudo apt install git

$ sudo apt install make
```

node 对版本有要求，建议用 v10.x.x 版本。
```shell
$ node -v
v10.16.3
$ npm -v
6.9.0
$ nvm --version
0.34.0
```
我这里版本是这样的，一起正常。之前 node 是 v13.x.x，无法安装 web3js 相关的库。

具体安装参考[How to Install Node.js and npm on Ubuntu 18.04](https://linuxize.com/post/how-to-install-node-js-on-ubuntu-18.04/#installing-nodejs-and-npm-using-nvm)，这样就能够指定版本了。


#### 2. 安装 rebirth 缓存服务器
rebirth 很重要，后续包括区块链浏览器，钱包都是从 rebirth 这里获取数据。这样也更科学，减少链的数据读取负担。

参考[官方文档](https://github.com/citahub/re-birth/blob/develop/README-CN.md)。总结一下就是依次执行：
```shell
$ git clone https://github.com/citahub/re-birth.git

$ make setup

$ docker-compose run --rm app bundle exec rails secret
Starting rebirth_redis_1 ... 
Starting rebirth_db_1 ... done
cc32aaccc7f93343c6fae4b7b394450f263a9731a66ca75380f11460971e7ae3fc348f953a90940ac4f8f39d003aae9ae82b9692144ed3009f52fd654c189c45

$ make up #（如果需要调试，那就用 make start 替代，make start 会将 stdout 在终端中打印出来）
```

一般来讲，这时 rebirth 服务就启动好了。可以通过浏览器里面访问 http://[rebirth所在服务器IP地址]:8888 来查看是否正常，正常情况如下图：

![rebirth](./rebirth.PNG)

**如果无法连接，可以查看云服务器的防火墙是否放开了 8888 端口。**

到这里还不够，因为需要申请证书，换成 HTTPS 服务。

需要准备的：一个域名；对应证书。

假设有了一个域名 xxxx.com，大概步骤是，
1. 申请一个免费证书，前缀就是自己定，比如 testnode 之类的。
2. 然后将这个证书绑定到 testnode.xxx.com。并将对应的证书文件下载下来。
3. 然后去域名控制台把 testnode.xxx.com 绑定到节点0（运行rebirth工具链的）服务器的 IP 地址。
4. 根据 [re-birth](https://github.com/flyq/re-birth/commit/f4c385d01e63085dbe150c95a99e46cb3bb94a4f) 里的改动，去修改 `docker/web/Dockerfile`，`docker/web/nginx.conf` 这两个文件。并且把下载的证书文件（两个，包括 `.pem` 与 `.key`）放在 `docker/web/` 文件夹下。
5. 停止网络，然后 make update 更新配置，然后再 make up 启动，一般来讲这是就可以用 https://testnode.xxx.com:8888 访问服务了。


要改的文件：  
Dockerfile:
```Dockerfile
# Copy Nginx config template
COPY docker/web/nginx.conf /tmp/docker.nginx
COPY docker/web/yyyy_testnode.xxx.com.pem /etc/nginx/yyyy_testnode.xxx.com.pem
COPY docker/web/yyyy_testnode.xxx.com.key /etc/nginx/yyyy_testnode.xxx.com.key
```
就是在某行下面添加两行。其中 yyyy 是代表一串数字。

nginx.conf:
```conf
   listen 80 ssl;
   ssl_certificate yyyy_testnode.xxx.com.pem;
   ssl_certificate_key yyyy_testnode.xxx.com.key;
   ssl_session_timeout 5m;
   ssl_ciphers ECDHE-RSA-AES128-GCM-SHA256:ECDHE:ECDH:AES:HIGH:!NULL:!aNULL:!MD5:!ADH:!RC4;
   ssl_protocols TLSv1 TLSv1.1 TLSv1.2;
   ssl_prefer_server_ciphers on;
```
添加以上几行。


#### 3. 安装区块链浏览器
参考 [micrscope V2 文档](https://github.com/citahub/microscope-v2/blob/develop/README-CN.md#%E5%BF%AB%E9%80%9F%E6%8B%A5%E6%9C%89%E4%B8%80%E4%B8%AAmicrscope)，直接部署在 GitHub Page 上。然后在右上角设置节点 https://testnode.xxx.com:8888 就可以查看当前链的数据了。大概效果是：

![micr](micr.png)



#### 4. 安装移动端钱包
从[官方文档](https://docs.citahub.com/zh-CN/toolchain/cyton/cyton-intro)下载对应的移动端钱包。安装好后按照如下设置：

设置：  
添加资产 -> 区块链选择 CITA 原生代币 -> 确定 -> 输入节点地址（https://testnode.xxx.com:8888）-> 查询。

还可以关闭其他无关资产显示：  
添加资产 -> 列表设置 -> 关闭 ETH，MBA 按钮，只留下 CTT（CITA 的 Token），退出即可。

导入 CITA 链的超级管理员的账户后，最后效果图：

![cyton](cyton.jpg)


#### 5. 安装 First-forever-demo

参考[官方文档](https://github.com/citahub/first-forever-demo)

node 13 会报错如下：
```shell
scrypt.target.mk:131: recipe for target 'Release/obj.target/scrypt/src/node-boilerplate/scrypt_params_async.o' failed
make: *** [Release/obj.target/scrypt/src/node-boilerplate/scrypt_params_async.o] Error 1
make: Leaving directory '/data/cita/tools/first-forever-demo/node_modules/scrypt/build'
gyp ERR! build error 
gyp ERR! stack Error: `make` failed with exit code: 2
gyp ERR! stack     at ChildProcess.onExit (/usr/lib/node_modules/npm/node_modules/node-gyp/lib/build.js:194:23)
gyp ERR! stack     at ChildProcess.emit (events.js:321:20)
gyp ERR! stack     at Process.ChildProcess._handle.onexit (internal/child_process.js:275:12)
gyp ERR! System Linux 4.15.0-1051-aws
gyp ERR! command "/usr/bin/node" "/usr/lib/node_modules/npm/node_modules/node-gyp/bin/node-gyp.js" "rebuild"
gyp ERR! cwd /data/cita/tools/first-forever-demo/node_modules/scrypt
gyp ERR! node -v v13.6.0
gyp ERR! node-gyp -v v5.0.5
gyp ERR! not ok 
npm WARN tsutils@3.17.1 requires a peer of typescript@>=2.8.0 || >= 3.2.0-dev || >= 3.3.0-dev || >= 3.4.0-dev || >= 3.5.0-dev || >= 3.6.0-dev || >= 3.6.0-beta || >= 3.7.0-dev || >= 3.7.0-beta but none is installed. You must install peer dependencies yourself.
npm WARN optional SKIPPING OPTIONAL DEPENDENCY: fsevents@1.2.11 (node_modules/jest-haste-map/node_modules/fsevents):
npm WARN notsup SKIPPING OPTIONAL DEPENDENCY: Unsupported platform for fsevents@1.2.11: wanted {"os":"darwin","arch":"any"} (current: {"os":"linux","arch":"x64"})
npm WARN optional SKIPPING OPTIONAL DEPENDENCY: fsevents@1.2.11 (node_modules/chokidar/node_modules/fsevents):
npm WARN notsup SKIPPING OPTIONAL DEPENDENCY: Unsupported platform for fsevents@1.2.11: wanted {"os":"darwin","arch":"any"} (current: {"os":"linux","arch":"x64"})
npm WARN optional SKIPPING OPTIONAL DEPENDENCY: fsevents@2.1.2 (node_modules/fsevents):
npm WARN notsup SKIPPING OPTIONAL DEPENDENCY: Unsupported platform for fsevents@2.1.2: wanted {"os":"darwin","arch":"any"} (current: {"os":"linux","arch":"x64"})

npm ERR! code ELIFECYCLE
npm ERR! errno 1
npm ERR! scrypt@6.0.3 install: `node-gyp rebuild`
npm ERR! Exit status 1
npm ERR! 
npm ERR! Failed at the scrypt@6.0.3 install script.
npm ERR! This is probably not a problem with npm. There is likely additional logging output above.

npm ERR! A complete log of this run can be found in:
npm ERR!     /home/ubuntu/.npm/_logs/2020-01-20T14_09_59_094Z-debug.log
```
建议使用 `v10.16.3` 版本。

部署步骤，依次执行：
```shell
$ git clone https://github.com/citahub/first-forever-demo.git

$ cd first-forever-demo/

$ cp .env.example .env
```
部署智能合约：
```shell
$ cp src/config.js.example src/config.js

$ emacs src/config.js # 进去之后修改 config 的 chain，privateKey，改成 https://testnode.xxx.com:8888 与你准备用来部署合约的账户的私钥。

$ npm install

$ npm run deploy

> first_forever@0.1.0 deploy /data/cita/tools/first-forever-demo
> node ./src/contracts/deploy.js

It working in undefined
contractAddress is: 0x3b6c6C8022c858bA316122A873A4EF974754D247
[ { constant: false,
    inputs: [ [Object], [Object] ],
    name: 'add',
    outputs: [],
    payable: false,
    stateMutability: 'nonpayable',
    type: 'function',
    signature: '0x36555b85' },
  { anonymous: false,
    inputs: [ [Object], [Object], [Object] ],
    name: 'Recorded',
    type: 'event',
    signature:
     '0xe4af93ca7e370881e6f1b57272e42a3d851d3cc6d951b4f4d2e7a963914468a2' },
  { constant: true,
    inputs: [ [Object] ],
    name: 'get',
    outputs: [ [Object] ],
    payable: false,
    stateMutability: 'view',
    type: 'function',
    signature: '0x9507d39a' },
  { constant: true,
    inputs: [],
    name: 'getList',
    outputs: [ [Object] ],
    payable: false,
    stateMutability: 'view',
    type: 'function',
    signature: '0x942b765a' } ]
```

最后显示合约部署成功，记下合约地址。

再次修改 config.js，把合约地址加上去：

```js
const config = {
  chain: '{addr of net you are using}',
  privateKey: '{your private key}',
  contractAddress: '{deployed contract address}',
}
module.exports = config
```

调试，部署前端：
```shell
$ npm start 
```
如果 3000 端口的防火墙允许了，就可以远程访问了。

调试完成后，可以部署：
```shell
$ npm run build

$ serve -l 3000 -s build/
```

最后的效果图：
![demo](demo.png)
![demo1](demo1.png)