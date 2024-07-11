---
title: wait 与 wake
description: wait 与 wake
duration: 30 min
---

# wait 与 wake

---

## The problem of waiting

![alt text](image.png)

<br/>

🤔 如何避免同步等待/轮询?

---

## Waiting without wasting

![alt text](image-1.png)

<br/>

🤔 如何回复最初的消息?

---

## Waiting Queue

![alt text](image-2.png)

<br/>

waitlist - `wait`: 保存待处理的消息, `wake`: 将消息取出重新执行

---

## wait

- `fn wait() -> !` - Pause the current message handling.

<br/>

> Put the current message into the __waiting queue__ to be awakened using the correspondent `wake` function later.

<br/>

https://docs.gear.rs/gstd/exec/fn.wait.html

---

## `!`: the never type

<br/>

> Represents the type of computations which never resolve to any value at all.

> `break`, `continue` and `return` expressions all have type `!`

<br/>

https://doc.rust-lang.org/reference/types/never.html

---

### `std::process`
- `fn exit(code: i32) -> !` - Terminates the current process with the specified exit code.

### `gstd::exec`

- `fn exit(id: ActorId) -> !` - Terminate the execution of a program.
- `fn leave() -> !` - Break the current execution and save the state.
- `fn wait_for(duration: u32) -> !` - Same as wait, but delays handling for a specific number of blocks.
- `fn wait_up_to(duration: u32) -> !` - Same as wait, but delays handling for the maximum number of blocks that can be paid for and doesn’t exceed the given duration.

---

## wake

- `fn wake(message_id: MessageId) -> Result<(), Error>`

<br/>

> Resume previously paused message handling.

> `message_id` specifies a particular message to be taken out of the __waiting queue__ and put into the __processing queue__.

<br/>

https://docs.gear.rs/gstd/exec/fn.wake.html

---

```mermaid
sequenceDiagram
    participant User
    participant Handle as Controller::handle
    participant Handle_Reply as Controller::handle_reply
    participant Counter

    Note over Handle: Idle

    User->>Handle: msg::send (Action::Inc/Dec/Get)
    activate Handle

    Handle->>Counter: msg::send (inc/dec/get)
    activate Counter
    Note over Handle: MessageSent
    Handle-->>Handle: wait()
    deactivate Handle

    Counter->>Counter: Processing (x blocks, x ≥ 0)

    Counter-->>Handle_Reply: msg::reply (n)
    deactivate Counter
    activate Handle_Reply
    Note over Handle_Reply: MessageReceived
    Handle_Reply->>Handle_Reply: wake(user_msg_id)
    Handle_Reply-->>Handle: re-execution
    deactivate Handle_Reply

    activate Handle
    Handle-->>User: msg::reply (n)
    Note over Handle: Idle
    deactivate Handle
```

<br/>

使用 wait/wake 后的时序图: User - Controller - Counter