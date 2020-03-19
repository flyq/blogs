# Chapter 1. Why Rust

## `Contents`
- [Chapter 1. Why Rust](#chapter-1-why-rust)
  - [`Contents`](#contents)
  - [特点](#%e7%89%b9%e7%82%b9)
  - [类型安全](#%e7%b1%bb%e5%9e%8b%e5%ae%89%e5%85%a8)

## 特点
系统编程源远流长。
自从人类开始使用高级语言写操作系统开始，事实证明，尤其有两个问题很难破解。
* 很难写出安全的代码，尤其是正确的内存管理代码。人类为安全漏洞付出了足够多的代价了。
* 写出多线程代码，简直难于上青天。但是这是充分压榨现代机器的唯一方式。即使老鸟，面对多线程代码也得小心翼翼。并发一不小心就会引入新的大量bugs，而且还难以复现

进入 Rust 世界，你就拥有了一个安全的并发语言，和媲美于C和C++的性能

足够接近机器，能预测其代码的开销，零成本抽象

并且有
* 内存安全
* 值的信赖的并发。

因为 Rust 提供了 ownership，moves，borrows，这些都是在编译时进行检查。
还有灵活的 静态类型系统

* ownership 系统：为每一个变量建立清晰的生命周期，从而不需要垃圾回收机制
* 拥有健全灵活的接口来管理其他各种资源。比如文件 handler 以及sockets
* moves：value from one owner to another.
* borrowing: use value temporarily without affecting its ownership
* ownership 也是 Rust 值得信赖的并发模型的基础


* 有面向对象元素，不是真正的面向对象语言

## 类型安全
