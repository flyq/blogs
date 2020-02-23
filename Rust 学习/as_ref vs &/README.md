# as_ref & borrow ...
说实在的，Rust 里面又是引用又是借用又是字符串Slice又是智能指针等等各种指针相关的概念，让人挺困惑的，得理一理。

## `Contents`
- [as_ref & borrow ...](#asref--borrow)
  - [`Contents`](#contents)
  - [引用 references](#%e5%bc%95%e7%94%a8-references)
  - [借用 borrowing](#%e5%80%9f%e7%94%a8-borrowing)
  - [slice](#slice)
    - [&str（字符串 slice）](#str%e5%ad%97%e7%ac%a6%e4%b8%b2-slice)
    - [其他类型的 slice](#%e5%85%b6%e4%bb%96%e7%b1%bb%e5%9e%8b%e7%9a%84-slice)
  - [Borrow 和 AsRef 的区别](#borrow-%e5%92%8c-asref-%e7%9a%84%e5%8c%ba%e5%88%ab)
  - [智能指针](#%e6%99%ba%e8%83%bd%e6%8c%87%e9%92%88)


## 引用 references
在 [trpl](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#references-and-borrowing) 中，有过这样一段描述：
> in its definition, we take `&String` rather than `String`.
These ampersands are `references`, and they allow you to refer to some value without taking ownership of it. Figure 4-5 shows a diagram.
![&String](images/trpl04-05.svg)
Figure 4-5: A diagram of `&String s` pointing at `String s1`

其中，ampersands 就是连字符&。对比[这本书的中文版](https://rustlang-cn.org/office/rust/book/understanding-ownership/ch04-02-references-and-borrowing.html)，`references` 翻译成引用。而根据图示，一个命名为 s 的 String 类型引用，它的值指向了 String 类型的 s1。然后因为 String 类型也是一个指针类型，因此 s1 的指针 (ptr, pointer 缩写) 指向真正存储字符串的内存区域。

不过 s1 和 s 相比，除了名字，指针指向地址外，还有长度（len），容量（capacity）这两个描述指向内容的`元数据 (Metadata)`。因此有的上下文就把 String 这种称为胖指针(Fat pointer)。

然后，引用也不会获取它指向对象的所有权（ownership），比如 s 离开了作用域，被丢弃，但是 s1 不会受到影响。

引用规则：
* 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
* 引用必须总是有效。

## 借用 borrowing
We call having references as function parameters borrowing.
我们将获取引用作为函数参数称为 借用（borrowing）。

借用，就是指将参数传入新函数的时候，只是传一个引用进去。这样，参数的所有权没有传入进去，因此原函数里面还能使用这个参数。

这个参数在新函数里面能不能修改，取决于是否传入的可变引用 `&mut String`。  
在特定作用域中的特定数据有且只有一个可变引用。


```rust
use std::borrow::Borrow;
use std::fmt::Display;

fn foo<T: Borrow<i32> + Display>(a: T) {
    println!("a is borrowed: {}", a);
}

fn foo2<T: Borrow<i32> + Display>(a: &T) {
    println!("a is borrowed: {}", a);
}

fn foo3(a: i32) {
    println!("a is borrowed: {}", a);
}

fn foo4(a: &i32) {
    println!("a is borrowed: {}", a);
}

fn main() {
    let mut i = 5;

    foo(i);
    foo(&i);

    foo2(&i);
    foo2(&mut i);
    // foo2(i);  // error

    // foo3(&i); // error
    foo3(i);

    // foo4(i); // error
    foo4(&i);
}
```


## slice
另一个没有所有权的数据类型是 slice。slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。

### &str（字符串 slice）
**字符串 slice**（string slice）是 `String` 中一部分值的引用，它看起来像这样：
```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```
其中 world 的指针就是如图所示：

![slice](images/trpl04-06.svg)

此时不用担心 s 被清空，导致 world 指向的内存空间被清空。因为借用/引用规则。

总结：  
字符串 slice 是一种指针。类型是 &str，字符串字面值如：
```rust
    let s = "hello, world!";
```
其中 s 是一个 &str，并且是不可变的，它指向可执行文件里面保存字符串字面值的某个地方。   
除了直接从字符串字面值得到 &str 外，还可以通过
```rust
    let my_string = String::from("hello world");

    // first_word 中传入 `String` 的 slice
    let word = first_word(&my_string[..]);
```
得到 &str，这里 word 就是。

### 其他类型的 slice
```rust
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
```
其中 slice 的类型的 &[i32]

## Borrow 和 AsRef 的区别
这个可以参考：   
https://doc.rust-lang.org/1.5.0/book/borrow-and-asref.html

https://github.com/rust-lang/rust/issues/24140#issuecomment-90626264

https://kaisery.gitbooks.io/rust-book-chinese/content/content/Borrow%20and%20AsRef%20Borrow%20%E5%92%8C%20AsRef.html

http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/convert/trait.AsRef.html

https://www.jianshu.com/p/0f039b76aa09

说实在的，还不是很理解，还需要有更多具体实例。而且，参考 [rust-lang/rust 下面的讨论](https://github.com/rust-lang/rust/issues/24140)，很多人也是懵逼的。

* 总的印象就是如果只要用到一个对象的引用，就用 AsRef；
* 如果又要用到它的引用，又要用到它本身自身那个对象，就用 Borrow
* 如果你搞不清它是不是已经是引用了，只想得到某类型的引用，用 AsRef，多次 AsRef 仍然只有一层引用：

```rust
fn is_hello<T: AsRef<str>>(s: T) {
   assert_eq!("hello", s.as_ref());
}

let s1 = "hello";
is_hello(s1);

let s2 = "hello".to_string();
is_hello(s2);
```
比如以上，s1 已经是 &str，已经是有个引用了。但是 s1.as_ref() 仍然是 &str，而不是 &&str。
而 s2 是 String，s2.as_ref() 是 &str。

## 智能指针
参考 https://rustlang-cn.org/office/rust/book/smart-pointers/ch15-00-smart-pointers.html

总的来讲，根据深入浅出Rust里面的说法，可变不共享，共享不可变。

