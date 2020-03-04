# Rust Actix-Web 验证 Auth Web 微服务

参考： https://zhuanlan.zhihu.com/p/51497187

## 安装环境

### 安装 postgres
参考： https://tecadmin.net/install-postgresql-server-on-ubuntu/

分步执行：
```shell
sudo apt-get install wget ca-certificates

wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo apt-key add -

sudo sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt/ `lsb_release -cs`-pgdg main" >> /etc/apt/sources.list.d/pgdg.list'

sudo apt update

sudo apt upgrade

sudo apt-get install postgresql postgresql-contrib
```

前面一般没什么问题。

按照步骤按照好 postgresql 之后，本身 PostgresQL 会创建一个默认 user：`postgres`，它有个 role：`postgres`。然后它已经创建了一个系统账号：`postgres`。这个是可以连接数据库的。

进入 postgres 系统账号。

```shell
sudo su - postgres

psql

postgres-# CREATE ROLE ubuntu WITH LOGIN CREATEDB ENCRYPTED PASSWORD 'xxxxxx（password）';

postgres-# \q

sudo su - ubuntu

createdb my_db

psql
rahul=> \list
rahul=> \list
```
## 安装 diesel-cli
