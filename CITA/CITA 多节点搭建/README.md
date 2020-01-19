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


## 特性
节点分为普通节点，共识节点。均可以增加。所有节点最多 256 个节点。

## 配置步骤
### 一、安装对应环境
#### 登录服务器
```shell
$ ssh xxx@ip
```
其中 `xxx` 代表用户名，`ip` 代表要登陆的服务器的 ip 地址。

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
