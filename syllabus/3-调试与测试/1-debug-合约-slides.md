---
title: Vara Network Intro, Hello World
description: Introduction to Vara Network and upload your first application
duration: 30 min
---

# debug 合约

---

## 打印 debug 信息


- `gstd::debug!()` 
- `gstd::dbg!()`

---

## 构建时启用 debug

<pba-flex left>

```bash
cargo build
```

或

```bash
cargo build --release --features=debug
```

<br/>

查看 debug 信息:
- `RUST_LOG="gwasm=debug"` 启动本地节点
- `gtest` 单元测试

</pba-flex>

---

## `debug!()`

Similar to the standard library’s println! macro.


```rust
use gstd::debug;

#[no_mangle]
extern "C" fn handle() {
    debug!("String literal");
    // ^--- prints: DEBUG: [handle(0x38e5..6b6d)] 0x5b98..63c5: String literal
    //                             ^--- msg_id    ^--- program_id

    let value = 42;
    debug!("{value}");
    // ^--- prints: DEBUG: [handle(0x38e5..6b6d)] 0x5b98..63c5: 42

    debug!("Formatted: value = {value}");
    // ^--- prints: DEBUG: [handle(0x38e5..6b6d)] 0x5b98..63c5: Formatted: value = 42
}
```

<br/>

https://docs.gear.rs/gstd/macro.debug.html

---

## `dbg!()`

Similar to the standard library’s dbg! macro.

```rust
use gstd::dbg;

#[no_mangle]
extern "C" fn handle() {
    let a = 2;
    let b = dbg!(a * 2) + 1;
    //      ^--- prints: DEBUG: [handle(0x38e5..6b6d)] 0x5b98..63c5: \
    //                       [/root/template/src/lib.rs:32] a * 2 = 4
    assert_eq!(b, 5);
}
```


<br/>

https://docs.gear.rs/gstd/macro.dbg.html