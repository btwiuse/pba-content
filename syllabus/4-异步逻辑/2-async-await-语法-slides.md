---
title: async/.await 语法
description: async/.await 语法
duration: 30 min
---

# async/.await 语法

---

## Async APIs

```rust
pub fn sleep_for(block_count: u32) -> impl Future<Output = ()>
```

> Delays message execution in asynchronous way for the specified number of blocks.

https://docs.gear.rs/gstd/exec/fn.sleep_for.html

---


```rust
pub fn send_bytes_for_reply<T: AsRef<[u8]>>(
    program: ActorId,
    payload: T,
    value: u128,
    reply_deposit: u64
) -> Result<MessageFuture>
```

> Same as `send_bytes`, but the program will interrupt until the reply is received.

https://docs.gear.rs/gstd/msg/fn.send_bytes_for_reply.html

---

## Future trait

```rust
pub trait Future {
    type Output;

    // Required method
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

> A future represents an asynchronous computation obtained by use of async.
> When using a future, you generally won’t call poll directly, but instead .await the value.

https://docs.gear.rs/gstd/prelude/future/trait.Future.html

---

## Example: send_bytes_for_reply

```rust
use gstd::msg::{self, MessageFuture};

/* 
impl Future for MessageFuture {
  type Output = Result<Vec<u8>, Error> 
} 
*/

#[gstd::async_main]
async fn main() {
    let future: MessageFuture =
        msg::send_bytes_for_reply(dest, b"PING", 0, 0).expect("Unable to send");
    let reply: Vec<u8> = future.await.expect("Unable to get a reply");
}
```

---

```rust
pub fn send_for_reply_as<E: Encode, D: Decode>(
    program: ActorId,
    payload: E,
    value: u128,
    reply_deposit: u64
) -> Result<CodecMessageFuture<D>>

/*
impl<D: Decode> Future for CodecMessageFuture<D> {
  type Output = Result<D, Error>
}
*/
```

> Same as `send`, but the program will interrupt until the reply is received.

https://docs.gear.rs/gstd/msg/fn.send_bytes_for_reply_as.html

---

## Example: send_for_reply_as

```rust
let reply: SomeEvent = msg::send_for_reply_as(
    receiver_id,
    SomeAction {
        command: 42,
    },
    0,
    0,
).expect("Unable to send message")
 .await
 .expect("Error in receiving reply");
```

---

```rust
#[gstd::async_init]
async fn init() { ... }

#[gstd::async_main]
async fn main() { ... }
```

=>

```
#[no_mangle] extern fn init() {
    gstd::message_loop(async #init_body);
}

#[no_mangle] extern fn handle() { 
    gstd::message_loop(async #main_body);
 }

#[no_mangle] extern fn handle_reply() { ... }
#[no_mangle] extern fn handle_signal() { ... }
```