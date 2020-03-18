# Preface
前言部分
## `Contents`
- [Preface](#preface)
  - [`Contents`](#contents)
  - [Rust 使命](#rust-%e4%bd%bf%e5%91%bd)
  - [谁应该读这本书](#%e8%b0%81%e5%ba%94%e8%af%a5%e8%af%bb%e8%bf%99%e6%9c%ac%e4%b9%a6)
  - [为什么写这本书](#%e4%b8%ba%e4%bb%80%e4%b9%88%e5%86%99%e8%bf%99%e6%9c%ac%e4%b9%a6)
  - [导读这本书](#%e5%af%bc%e8%af%bb%e8%bf%99%e6%9c%ac%e4%b9%a6)
  - [约定惯例](#%e7%ba%a6%e5%ae%9a%e6%83%af%e4%be%8b)
  - [代码示例](#%e4%bb%a3%e7%a0%81%e7%a4%ba%e4%be%8b)
  - [OReilly Safari](#oreilly-safari)
  - [怎么联系我们](#%e6%80%8e%e4%b9%88%e8%81%94%e7%b3%bb%e6%88%91%e4%bb%ac)
  - [致谢](#%e8%87%b4%e8%b0%a2)


## Rust 使命
Rust 是为系统编程而生。

系统编程 System programming：
* 操作系统
* 设备驱动
* 文件系统
* 数据库
* 工业级代码
* 密码学
* 流媒体编码（读写音频，视频，以及图片等）
* 流媒体处理（语言识别或者PS）
* 内存管理（比如实现一个垃圾回收系统）
* 文字渲染（文本字体转换成像素）
* 实现更高级的程序语言（JS，Python）
* 网络
* 虚拟化技术和软件容器
* 科学仿真
* 游戏

总之，系统编程是在资源受限情况下的编程，每个比特，每个CPU周期都非常重要。

这本书不会教你系统编程，但是有很多内存管理方面比较深入的内容。如果你对系统编程富有经验，你会发现 Rust 是非常优秀的：它在消除困惑业界几十年的重大问题上是一个非常友好的工具。

## 谁应该读这本书
阅读对象：
1. 系统程序员
2. 想找 C++ 的替代者的程序员
3. 任何其他程序员

需要学习：
1. rust 语言
2. 系统编程经验
3. 实操一些项目，着重体现 Rust 的速度，并发，以及安全。项目参考前面的那些主体

## 为什么写这本书
我希望我在学 Rust 的时候有这么一本书，把新概念，完整清晰的，有深度的，呈现给读者，从而杜绝混淆和理解错误

## 导读这本书
前两章，介绍rust，有个简短的tour

第三章，基本数据结构

四五章讲解ownership和references  
前五章按部就班的读

6-10，语言的基本内容：
6. 表达式
7. error handling 错误处理
8. crates and modules 包管理
9. 结构 structs
10. 枚举，模式匹配 enums，patterns

可以略读，但是不要略读 error handling

11. trait 和 generics，Trait 和泛型，整合你自己的类型到语言中
12. trait 的重载，operator overloading
13. 更多有用的 traits

理解trait 和泛型解锁了书籍的最后部分

14-15 闭包和迭代器，closures & iterators

剩下的可以打乱顺序读，或者需要是再看
16. 集合 collections
17. strings text 字符串和文本
18. 输入输出 input output
19. 并发 concurrency
20. 宏处理 macros
21. unsafe code。hack the rust

## 约定惯例
italic
Constant width

## 代码示例
https://github.com/ProgrammingRust


## OReilly Safari
广告推销

## 怎么联系我们
不需要

## 致谢
感谢作者