---
title: 合约调用
description: 合约调用
duration: 30 min
---

# 合约调用

---

## Gear-JS 消息 API

- api.message.send
- api.message.sendReply
- api.message.calculateReply

<br/>

文档: https://github.com/gear-tech/gear-js/tree/main/api

---

## api.message.send 示例

```javascript
const message = {
  destination: destination, // programId
  payload: somePayload,
  gasLimit: 10000000,
  value: 1000,
  keepAlive: true, // if set to true the account is protected against removal due to low balances.
};
// In that case payload will be encoded using meta.handle_input type
let extrinsic = await gearApi.message.send(message, meta);
await extrinsic.signAndSend(keyring, (event) => {
  console.log(events.toHuman());
});
```

---

## api.message.sendReply 示例

```javascript
// To read the mailbox use api.mailbox.read method.
const mailbox = await api.mailbox.read('5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY');
console.log(mailbox);

const reply = {
  replyToId: messageId,
  payload: somePayload,
  gasLimit: 10000000,
  value: 1000,
  keepAlive: true,
};
const extrinsic = await gearApi.message.sendReply(reply, meta);
await extrinsic(keyring, (events) => {
  console.log(events.toHuman());
});
```

---

## api.message.calculateReply 示例

The `api.message.calculateReply` method can be used to send a message to the program and get the reply without transaction. 

```javascript
const result = await api.message.calculateReply({
  origin,
  destination: programId,
  payload: { myPayload: [] },
  value: 0
}, meta);

console.log(result.toJSON());
console.log('reply payload:', meta.createType(meta.types.handle.output, result.payload).toJSON());
```


