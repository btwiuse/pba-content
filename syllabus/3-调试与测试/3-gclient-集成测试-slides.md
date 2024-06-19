---
title: Vara Network Intro, Hello World
description: Introduction to Vara Network and upload your first application
duration: 30 min
---

# gclient 集成测试

---

## gclient

gclient 是一个支持在链上对合约进行端到端测试的工具

- 事件订阅
- Gas 计算
- 交易处理
- 异步支持
- ...

---

## 引入 gclient 库

```toml
[dev-dependencies]
gclient = { version = "1.4.1" }
tokio = { version = "1", features = ["full"] }
```

<br/>

安装节点

```bash
# 检查节点版本与 gclient 一致
./gear --version
```

---

## gclient 语法介绍

- 创建 API 实例
- 订阅事件
- Gas 计算
- 上传和初始化程序
- 发送消息
- 验证合约状态

---

## 创建 API 实例

```rust
// Create API instance
let api = GearApi::dev().await?;
```

---

## 订阅事件

```rust
let mut listener = api.subscribe().await?;
// Check that blocks are still running
assert!(listener.blocks_running().await?);
```

---

## Gas 计算

```rust
// Calculate gas amount needed for initialization
let gas_info = api
    .calculate_upload_gas(
        None,
        gclient::code_from_os(WASM_PATH)?,
        vec![],
        0,
        true,
    )
    .await?;

let payload = b"inc".to_vec();

// Calculate gas amount needed for handling the message
let gas_info = api
    .calculate_handle_gas(None, program_id, payload.clone(), 0, true)
    .await?;
```

---

## 上传和初始化程序

```rust
// Upload and init the program
let (message_id, program_id, _hash) = api
    .upload_program_bytes_by_path(
        WASM_PATH,
        gclient::now_micros().to_le_bytes(),
        vec![],
        gas_info.min_limit,
        0,
    )
    .await?;
assert!(listener.message_processed(message_id).await?.succeed());
```

---

## 发送消息

```rust
// Send the inc message
let (message_id, _hash) = api
    .send_message_bytes(program_id, payload, gas_info.min_limit, 0)
    .await?;
assert!(listener.message_processed(message_id).await?.succeed());
```

---

## 验证合约状态

```rust
// Listen and verify the returned message
if let (message_id, result, value) = listener.reply_bytes_on(message_id).await? {
    if let Ok(data) = result {
        println!("Data: {:?}", data);
        assert_eq!(data, b"1");
    } else if let Err(error) = result {
        println!("Error: {:?}", error);
    }
}
```