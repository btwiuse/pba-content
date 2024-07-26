---
title: 读取状态
description: 读取状态
duration: 30 min
---

# 读取状态

---

## GearApi

- 解析合约 Metadata
- 查询状态
- 发送交易 (消息)
- 预估 Gas
- 订阅事件
- 部署合约
- ...

<br/>

文档: https://github.com/gear-tech/gear-js/tree/main/api

---

## Polkadot.js 基础

<iframe width="560" height="315" src="https://www.youtube.com/embed/Cns17Mkzq3U?si=Rn6DAQpxyCpuB11Q" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

- GearApi 是 polkadot.js ApiPromise 的子类
- ApiPromise 示例: https://polkadot.js.org/docs/api/examples/promise/

---

## @gear-js/api

```javascript
import { GearApi } from '@gear-js/api';

const api = await GearApi.create({
  providerAddress: 'wss://rpc.vara.network',
});
```

---

## 常用 RPC 端点

<pba-flex center>

### Vara Mainnet

- wss://rpc.vara.network
- wss://archive-rpc.vara.network

### Vara Testnet

- wss://testnet.vara.network
- wss://testnet-archive.vara.network

### Local Node

- ws://localhost:9944

</pba-flex>

---

## 解码 Metadata

```javascript
import { ProgramMetadata } from '@gear-js/api';

const meta = ProgramMetadata.from(`0x...`);
```

---

## 查询状态

```javascript
api.programState.read({
    programId,
    payload,
  },
  meta,
)
```

---

## 示例 1：完整状态 (Metadata + JavaScript)

```rust
impl Metadata for DemoPingMetadata {
    ...
    type State = Out<Vec<String>>;
}
```

<br/>

```javascript
let result = api.programState.read({
    programId: '0x54045e2dc35baf015f325b80b23a6d7d0942eaa9b2ecb87e56f934f7c3a71f5f',
    payload: '0x',
  },
  meta,
);

console.log("result:", JSON.stringify(result.toHuman()));
```

---

## 示例 2: 部分状态 (Metadata)

```rust
impl Metadata for ContractMetadata {
    ...
    type State = InOut<Query, Reply>;
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum Query {
    All,
    Key(String),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum Reply {
    All(Contract),
    Value(Option<String>),
}
```

---

## 示例 2: 部分状态 (JavaScript)

```javascript
let all = api.programState.read({
    programId: '0xec9658323437ab33ca204f54ed7e6a0b3199972bc753f24133a1442989d02eba',
    payload: {All: null},
  },
  meta,
);

let value = api.programState.read({
    programId: '0xec9658323437ab33ca204f54ed7e6a0b3199972bc753f24133a1442989d02eba',
    payload: {Key: "foo"},
  },
  meta,
);

console.log("all:", JSON.stringify(all.toHuman()));
console.log("value:", JSON.stringify(value.toHuman()));
```

---

## 历史状态索引 (GraphQL)

### SubQuery

- https://subquery.network
- https://wiki.vara.network/docs/indexers/subquery/

### Subsquid

- https://subsquid.io
- https://wiki.vara.network/docs/examples/NFTs/nft-marketplace/subsquid-marketplace/