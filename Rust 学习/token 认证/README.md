# Rust Actix-Web 验证 Auth Web 微服务

参考： https://zhuanlan.zhihu.com/p/51497187

https://gill.net.in/posts/auth-microservice-rust-actix-web-diesel-complete-tutorial-part-2/

## `Contents`
- [Rust Actix-Web 验证 Auth Web 微服务](#rust-actix-web-%e9%aa%8c%e8%af%81-auth-web-%e5%be%ae%e6%9c%8d%e5%8a%a1)
  - [`Contents`](#contents)
  - [安装环境](#%e5%ae%89%e8%a3%85%e7%8e%af%e5%a2%83)
    - [安装 postgres](#%e5%ae%89%e8%a3%85-postgres)
    - [安装 diesel-cli](#%e5%ae%89%e8%a3%85-diesel-cli)

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

ubuntu=> \list
                              List of databases
    Name    |  Owner   | Encoding | Collate |  Ctype  |   Access privileges
------------+----------+----------+---------+---------+-----------------------
 my_db      | ubuntu   | UTF8     | C.UTF-8 | C.UTF-8 |
 my_db_1    | ubuntu   | UTF8     | C.UTF-8 | C.UTF-8 |
 my_db_2    | ubuntu   | UTF8     | C.UTF-8 | C.UTF-8 |
 my_test_db | ubuntu   | UTF8     | C.UTF-8 | C.UTF-8 |
 postgres   | postgres | UTF8     | C.UTF-8 | C.UTF-8 |
 template0  | postgres | UTF8     | C.UTF-8 | C.UTF-8 | =c/postgres          +
            |          |          |         |         | postgres=CTc/postgres
 template1  | postgres | UTF8     | C.UTF-8 | C.UTF-8 | =c/postgres          +
            |          |          |         |         | postgres=CTc/postgres
 ubuntu     | ubuntu   | UTF8     | C.UTF-8 | C.UTF-8 |
(8 rows)

```

可以看出，成功在 postgres 里面创建 Ubuntu 的账号。


如果需要创建对应的 USER 以及 Role：
```shell
sudo -u postgres createuser ubuntu
sudo -u postgres createdb -O ubuntu ubuntu



in your terminal to get into postgres

postgres=#
Run

CREATE USER new_username;

# Note: Replace new_username with the user you want to create, in your case that will be tom.

postgres=# CREATE USER new_username;
CREATE ROLE
```
### 安装 diesel-cli
参考：  
http://diesel.rs/guides/getting-started/

http://diesel.rs/guides/getting-started/

这个文档假设了 PostgreSQL 已经安装好了，前面我们已经安装好了。

安装 diesel-cli：
```shell
cargo install diesel_cli
```

有问题，因为虽然我按照了mysql，但是没有启动服务。

```shell
 = note: /usr/bin/ld: cannot find -lmysqlclient
          collect2: error: ld returned 1 exit status
          

error: aborting due to previous error

error: failed to compile `diesel_cli v1.4.0`, intermediate artifacts can be found at `/tmp/cargo-installkyrElE`

Caused by:
  could not compile `diesel_cli`.

To learn more, run the command again with --verbose.
```

算了，我们只用 postgres，就只安装它的feature:
```shell
cargo install diesel_cli --no-default-features --features postgres
```
显示成功安装：
```shell
  Compiling migrations_internals v1.4.0
   Compiling toml v0.4.10
   Compiling diesel_cli v1.4.0
    Finished release [optimized] target(s) in 2m 25s
  Installing /home/ubuntu/.cargo/bin/diesel
   Installed package `diesel_cli v1.4.0` (executable `diesel`)
```

设置数据库路径：
```shell
echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env


```
毫无疑问，我们之前是用 `sudo -u postgres createuser ubuntu` 创建的 user,密码都没有，那就重置一下 ubuntu 的密码：
```shell
# 先进入到 postgres 的 User 下面，运行
psql

ALTER USER ubuntu PASSWORD 'newpassword';
```

结果运行：
```shell
diesel setup
```
显示没权限：
```shell
diesel setup Creating database: diesel_demo "permission" denied to create database
```

```shell
# 先进入到 postgres 的 User 下面，运行
psql

ALTER USER ubuntu WITH CREATEDB;
```
这下好了：
```shell
$ diesel setup
Creating database: diesel_demo
```


经过修改对应的rust 代码，成功运行：
```shell
   Compiling diesel_derives v1.4.1
   Compiling diesel v1.4.3
   Compiling diesel_demo v0.1.0 (/home/ubuntu/workspaces/diesel_demo)
    Finished dev [unoptimized + debuginfo] target(s) in 50.15s
     Running `target/debug/write_post`
What would you like your title to be?
testtaile

Ok! Let's write testtaile (Press CTRL+D when finished)

aslptu
dfaslopfhjwa0fiewhjfasfjpsaf

Saved draft testtaile with id 1
ubuntu@ip-10-0-48-20:~/workspaces/diesel_demo$ 
```
