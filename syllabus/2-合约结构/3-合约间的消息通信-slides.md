---
title: Vara Network Intro, Hello World
description: Introduction to Vara Network and upload your first application
duration: 30 min
---

# 合约间的消息通信

---

## Request-Response 模式

<pba-flex center>

- 请求: A 发送消息 (init, handle) => B 接收消息 (handle)
- 应答: A 获取回复 (handle_reply) <= B 回复消息 (handle)

</pba-flex>

---

## 计数器

```rust
static mut COUNTER: i32 = 0;

#[no_mangle]
unsafe extern "C" fn handle() {
    let command = msg::load_bytes().expect("Invalid message");

    match command.as_slice() {
        b"inc" => COUNTER += 1,
        b"dec" => COUNTER -= 1,
        b"get" => {
            msg::reply(COUNTER, 0).expect("Unable to reply");
        }
        _ => (),
    }
}
```

---

## 控制器

```rust
static mut CONTRACT: Option<ActorId> = None;

#[no_mangle]
extern "C" fn init() {
    // TODO: 初始化 Counter 合约地址
}

#[no_mangle]
extern "C" fn handle() {
    // TODO: 向 Counter 地址发送 Inc/Dec/Get 消息
}

#[no_mangle]
extern "C" fn handle_reply() {
    // TODO: 接收 Get 消息的回复
}
```

Notes:

合约之间一般使用请求/应答的模式进行通信

A 在 init/handle 中向 B 发送消息，B 可以在 handle 中进行回复
然后 A 可以在 handle_reply 中获取收到的回复

下面我们通过一个基础的的例子来了解合约之间的消息通信


其中计数器，合约只定义了一个 handle 入口函数

根据二进制的消息内容来改变或者返回 counter 变量的值

它因为没有 Metadata, 所以直接使用起来不是特别的方便


我们下面的任务是编写一个控制器合约，代替我们直接向计数器发送消息

首先我们会在 Metadata 中定义输入输出的消息类型

在 init 函数中设置控制器合约的地址

然后在 handle 中向合约发送 Inc/Dec/Get 的消息

并且在 handle_reply 中获取 Get 消息的回复