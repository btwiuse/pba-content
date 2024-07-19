---
title: Gas 预留
description: Gas 预留
duration: 30 min
---

# Gas 预留

---

## 合约的自动执行 (self-execution)

> Once initiated, smart contracts can execute the terms of the agreement automatically without further input from the parties involved.
> -- Nick Szabo, 1994

<br/>

<pba-flex left>

- Ethereum: Passive ⬅️
  - 外部触发
- Vara: Autonomous 🔄
  - Gas 预留 ⛽
  - 延迟消息 ⌛

</pba-flex>

---

## gstd::Reservation

> Stores additional data along with ReservationId to track its state.

```
struct Reservation {
    id: ReservationId,
    amount: u64,
    valid_until: u32,
}

impl Reservation {
    fn reserve(amount: u64, duration: u32) -> Result<Self>
    fn unreserve(self) -> Result<u64>
    fn id(&self) -> ReservationId
    fn amount(&self) -> u64
    fn valid_until(&self) -> u32
}
```

<br/>

https://docs.gear.rs/gstd/struct.Reservation.html

---

## gstd::ReservationId

> Reservation identifier. The identifier is used to reserve and unreserve gas amount for program execution later.

```
pub fn send_from_reservation<E: Encode>(
    id: ReservationId,
    program: ActorId,
    payload: E,
    value: u128
) -> Result<MessageId>
```

<br/>

https://docs.gear.rs/gstd/struct.ReservationId.html

---

## exec::reserve_gas

```rust
fn reserve_gas(amount: u64, duration: u32) -> Result<ReservationId, Error>
```

> Reserve the `amount` of gas for further usage.
> `duration` is the block count within which the reserve must be used.
> This function returns `ReservationId`, which one can use for gas unreserving.

<br/>

https://docs.gear.rs/gstd/exec/fn.reserve_gas.html

---

## exec::unreserve_gas

```rust
fn unreserve_gas(id: ReservationId) -> Result<u64, Error>
```

> Unreserve gas identified by `ReservationId`.
> If successful, it returns the reserved amount of gas.

<br/>

https://docs.gear.rs/gstd/exec/fn.unreserve_gas.html

---

## 示例

Reserve 50 million of gas for seven blocks:

```rust
use gstd::{exec, ReservationId};

static mut RESERVED: ReservationId = ReservationId::zero();

#[no_mangle]
extern "C" fn init() {
    unsafe { RESERVED = exec::reserve_gas(50_000_000, 7).unwrap() };
}

#[no_mangle]
extern "C" fn handle() {
    exec::unreserve_gas(unsafe { RESERVED }).expect("Unable to unreserve");
}
```

---

## 相关 API (gstd::msg)

- [send_from_reservation](https://docs.gear.rs/gstd/msg/fn.send_from_reservation.html)
- [send_from_reservation_for_reply](https://docs.gear.rs/gstd/msg/fn.send_from_reservation_for_reply.html)
- [send_from_reservation_for_reply_as](https://docs.gear.rs/gstd/msg/fn.send_from_reservation_for_reply_as.html)
- [reply_from_reservation](https://docs.gear.rs/gstd/msg/fn.reply_from_reservation.html)
- [send_bytes_from_reservation](https://docs.gear.rs/gstd/msg/fn.send_bytes_from_reservation.html)
- [send_bytes_from_reservation_for_reply](https://docs.gear.rs/gstd/msg/fn.send_bytes_from_reservation_for_reply.html)
- [send_bytes_from_reservation_for_reply_as](https://docs.gear.rs/gstd/msg/fn.send_bytes_from_reservation_for_reply_as.html)
- [reply_bytes_from_reservation](https://docs.gear.rs/gstd/msg/fn.reply_bytes_from_reservation.html)
- [send_delayed_from_reservation](https://docs.gear.rs/gstd/msg/fn.send_delayed_from_reservation.html)

---

## Reservation Manager

The manager is used to control multiple gas reservations across executions. It can be used when you only care about reserved amounts and not concrete ReservationIds.

<br/>

https://docs.gear.rs/gstd/struct.Reservations.html

---

```rust
use gstd::{msg, prelude::*, Reservations};

static mut RESERVATIONS: Reservations = Reservations::new();

#[no_mangle]
extern "C" fn init() {
    unsafe {
        RESERVATIONS
            .reserve(200_000, 50)
            .expect("failed to reserve gas");
        RESERVATIONS
            .reserve(100_000, 100)
            .expect("failed to reserve gas");
        RESERVATIONS
            .reserve(50_000, 30)
            .expect("failed to reserve gas");
    }
}
```

---

```rust
#[no_mangle]
extern "C" fn handle() {
    let reservation = unsafe { RESERVATIONS.try_take_reservation(100_000) };
    if let Some(reservation) = reservation {
        msg::send_bytes_from_reservation(
            reservation.id(),
            msg::source(),
            "send_bytes_from_reservation",
            0,
        )
        .expect("Failed to send message from reservation");
    } else {
        msg::send_bytes(msg::source(), "send_bytes", 0).expect("Failed to send message");
    }
}
```
