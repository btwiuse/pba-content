---
title: Vara Network Intro, Hello World
description: Introduction to Vara Network and upload your first application
duration: 30 min
---

# gtest 单元测试

---

## gtest

是一个轻量级的测试框架，在链下环境模拟:

- 合约
- 用户
- 余额
- Mailbox
- ...

---

## 引入 gtest 库

```toml
[dev-dependencies]
gtest = { version = "1.4.1" }
```

---

## gtest 语法介绍

- 初始化系统环境
- 程序初始化
- 发送消息
- 处理执行结果
- 日志 (Log) 处理
- 时间控制
- 余额管理

---

## 初始化系统环境

```rust
let sys = System::new();
```

---

## 程序初始化

```rust
let program = Program::from_file(
    &sys,
    "./target/wasm32-unknown-unknown/release/demo_ping.wasm",
);
let program_id = program.id();
```

---

## 发送消息

```rust
let res = program.send_bytes(100001, "INIT MESSAGE");
```

---

## 处理执行结果

```rust
assert!(res.log().is_empty());
assert!(!res.main_failed());
```

---

## 日志 (Log) 处理

```rust
let expected_log = Log::builder()
    .source(ping_pong_id)
    .dest(100001)
    .payload_bytes("PONG");
assert!(res.contains(&expected_log));

assert!(!res.main_failed());
```

---

## 时间控制

```rust
sys.spend_blocks(150);
```

---

## 余额管理

```rust
sys.mint_to(42, 5000);
let prog = Program::current(&sys);
prog.mint(1000);
```