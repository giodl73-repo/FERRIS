---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-language:async-futures-and-pinning
kind: guide
module: rust-language
section: languages
title: Async, Futures, and Pinning
status: source-custody
source_custody: partial
current_path: rust-language/14-ASYNC-FUTURES-AND-PINNING.md
canonical_path: rust-language/14-ASYNC-FUTURES-AND-PINNING.md
backsource_ids: [mdloom-backfill:rust-language:14-async-futures-and-pinning]
concepts: [async, await, futures, poll, executors, wakers, cancellation, Pin, Unpin, Send futures, tokio]
root_concepts: [async]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Async, Futures, and Pinning

Rust's async is a **cooperative, poll-based** model with no built-in runtime. An
`async fn` compiles to a state machine implementing the `Future` trait; that
future does nothing until an **executor** (which you choose — usually `tokio`)
polls it. It runs synchronously between `.await` points and can yield when a
polled child future returns `Pending`. This differs from a thread and from C#
async methods: a C# async method starts
executing synchronously on its caller and returns a hot `Task` when it reaches
an incomplete `await`; that task need not represent thread-pool work. A Rust
future is inert data until polled. The advanced hazard — `Pin` — exists because
generated state machines may require a stable address.

```
+===============================================================================+
|          ASYNC STATE MACHINE + TYPICAL TOKIO-STYLE I/O RUNTIME                |
+===============================================================================+

  async fn f() { a().await; b().await; }   compiles to ~
  +------------------------------------------------------------------+
  | enum FState { Start, AwaitingA, AwaitingB, Done }                |
  | impl Future for FState {                                         |
  |   fn poll(self, cx) -> Poll<()> {                                |
  |     match state {                                                |
  |       Start      => match a().poll(cx) { Pending=>return Pending |
  |                                           Ready=>state=AwaitingB |
  |       AwaitingB  => ... Poll::Ready(())                          |
  |     } } }                                                        |
  +------------------------------------------------------------------+

  THE LOOP (who drives it)                 Poll RESULT
  ------------------------                 -----------
  EXECUTOR (tokio) --poll--> FUTURE        Poll::Ready(v)   done, value v
     ^                          |          Poll::Pending    not ready; I registered
     |                          v                           a WAKER, call me later
     +----- waker.wake() <-- REACTOR (one common readiness source)

  KEY CONTRASTS
  -------------
  future is LAZY: nothing runs until .await/spawn drives it (unlike a hot Task)
  .await is a SUSPENSION POINT: the only place a task yields
  NO runtime in std: pick tokio / async-std / smol / embassy
```

Only the `Future`/`Poll`/`Waker` contract is universal. Reactors backed by
epoll, kqueue, or IOCP are common for network runtimes; timers, channels,
embedded interrupts, completion APIs, and custom futures can arrange wakeups
differently.

## The Mental Model: Lazy State Machines

`async fn foo() -> T` does not run when called; it returns an `impl Future<Output
= T>` — a state machine value. `.await` on a future means "poll it; if `Pending`,
yield control back up to the executor and resume here when woken." The executor
polls the top-level future repeatedly, and a **reactor** (OS event source: epoll
on Linux, kqueue on BSD/macOS, IOCP on Windows) wakes tasks when their I/O is
ready. Between `.await` points the code runs synchronously on the executor
thread.

```rust
async fn fetch(url: &str) -> Result<String, reqwest::Error> {
    let resp = reqwest::get(url).await?;   // suspend until headers arrive
    let body = resp.text().await?;         // suspend until body arrives
    Ok(body)
}

#[tokio::main]                              // macro sets up the runtime + block_on
async fn main() {
    let body = fetch("https://example.com").await.unwrap();
    println!("{}", body.len());
}
```

## The `Future` Trait, Poll, and Wakers

```rust
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
enum Poll<T> { Ready(T), Pending }
```

When a future cannot progress (a socket is not readable yet), it stores the
`Waker` from the `Context` and returns `Pending`. The reactor, upon the I/O
event, calls `waker.wake()`, which tells the executor to re-poll this task. This
is the whole cooperative-scheduling machinery: **no preemption**; a task yields
only by returning `Pending` at an `.await`. A long CPU-bound loop with no `.await`
**blocks the executor thread** — offload such work to
`tokio::task::spawn_blocking` or a thread ([15](15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md)).

## Executors: You Must Choose One

The standard library defines `Future`, `Poll`, `Pin`, and `Waker` but **ships no
executor**. You pick a runtime:

| Runtime | Character |
|---------|-----------|
| **tokio** | de-facto standard; multi-threaded work-stealing scheduler, full I/O/timers/sync |
| **async-std** | std-like API (less active now) |
| **smol** | small, composable |
| **embassy** | `no_std` / embedded async |

The runtime provides `block_on` (drive a future to completion from sync code),
`spawn` (run a task concurrently), timers, async I/O, channels, and async-aware
`Mutex`. `#[tokio::main]` and `#[tokio::test]` are attribute macros that wrap your
`async fn` with runtime setup.

## Dropping Futures, Aborting Tasks, and Cancellation Safety

An owned future stops making progress when it is dropped. `tokio::select!` and
`timeout` commonly drop a losing or incomplete branch. Spawned tasks add a
runtime-specific ownership layer: dropping a Tokio `JoinHandle` **detaches** the
task, while `JoinHandle::abort` requests cancellation and the runtime drops the
task future when it processes that request.

Drop cleanup is synchronous: it cannot `.await`, undo an external side effect,
or automatically roll back a half-written protocol or transaction. A future can
be dropped while suspended at any `.await`, so async APIs document whether
repeated or cancelled operations are **cancellation-safe**.

## Pin and Unpin

Here is the deep part. The compiler-generated future may hold a reference *into
its own data* (a borrow that spans an `.await`). If such a future were moved in
memory, that internal pointer would dangle. **`Pin<P>` is a wrapper that promises
the pointee will not move** for the rest of its life, making self-references
sound. That is why `Future::poll` takes `self: Pin<&mut Self>`.

- **`Unpin`** is an auto-trait meaning that pinning adds no movement
  restriction for this type. Most ordinary types are `Unpin`.
- **`!Unpin`** means a pinned value must obey the stable-address contract; it
  does not prove that the value is self-referential. Compiler-generated async
  futures are conservatively `!Unpin`, including bodies that happen not to form
  a self-reference.
- Any `!Unpin` future must be pinned before direct polling. Executors and
  `.await` handle this for you; you meet `Pin` when manually implementing a
  `Future`, storing one behind an abstraction, or using `Box::pin` / `pin!`.

```rust
use std::pin::pin;
let fut = some_async_op();       // a !Unpin future
let mut fut = pin!(fut);         // pin it on the stack so it can be polled manually
```

You do not need `Pin` for everyday `async/await`; it surfaces when you write
runtime-level code. Treat it as the stable-address contract required by futures
whose internal invariants would be invalidated by movement.

## async fn in Traits and Send Futures

`async fn` in traits (**AFIT**) stabilized in **Rust 1.75** ([07](07-DISPATCH-TRAIT-OBJECTS-AND-IMPL-TRAIT.md)).
The remaining sharp edge: a plain `async fn` in a trait produces a future whose
`Send`-ness cannot be bounded at the trait definition in the simplest syntax, so
using such a trait with a multi-threaded executor that requires `Send` tasks can
be awkward. Workarounds on stable: return `impl Future<Output = T> + Send`
explicitly, or use the `trait-variant` crate to generate a `Send` variant. For
`dyn` async traits, the older `#[async_trait]` crate (which boxes the future)
still has its place. This is an area of active ergonomic improvement — check
current release notes.

## Old World -> New World Bridge

| Old world | Rust async | Difference |
|-----------|-----------|-----------|
| .NET `Task<T>` / `async`/`await` | `Future` / `async`/`await` | Rust futures are **lazy**; `Task` is hot (already running) |
| .NET task scheduling | `tokio::spawn` | Schedules on the chosen executor; not one OS thread per task |
| `CancellationToken` | drop an owned future / `select!` / task-handle abort | Branch drop and spawned-task cancellation are distinct |
| `await` yields to scheduler | `.await` yields to executor | Cooperative; a CPU loop blocks the worker |
| JS event loop / Promises | executor + reactor | Promises are eager; Rust futures are lazy |
| `ConfigureAwait(false)` nuances | `Send` bounds on futures | Different mechanism, same "which thread?" concern |
| green threads (Go goroutines) | async tasks | Rust async is stackless state machines, not stackful |

The single most important correction for a .NET reader: a Rust `Future` is
**inert until polled**. Calling an `async fn` and not `.await`-ing (or spawning)
it does *nothing* — no work starts, and the compiler warns "unused future." A C#
`Task` from an `async` method is already running.

## Common Confusion Points

- **Futures are lazy.** Calling `foo()` on an `async fn` starts nothing. You must
  `.await` it or `spawn` it. "unused `Future` that must be used" is the warning.
- **No runtime in std.** You get a link/compile-time confusion if you `.await`
  with no executor. Add `tokio` and `#[tokio::main]`.
- **Blocking the executor.** `std::thread::sleep`, big CPU loops, or blocking I/O
  inside an async task stall the whole worker. Use async equivalents or
  `spawn_blocking`.
- **Cancellation ownership matters.** Dropping an owned branch stops it;
  dropping a Tokio `JoinHandle` detaches. Use `abort` when you intend to cancel
  a spawned task, and design awaited operations for cancellation safety.
- **`Pin` panic when hand-rolling.** You only need `Pin`/`pin!`/`Box::pin` for
  manual `Future` impls or storing futures; everyday `async/await` handles it.
- **`async fn` in traits + `Send`.** Multi-threaded executors want `Send`
  futures; plain AFIT does not express that — return `impl Future + Send` or use
  `trait-variant`/`async_trait`.
- **`Mutex` across `.await`.** Do not hold a `std::sync::Mutex` guard across an
  `.await`; use `tokio::sync::Mutex` ([15](15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md)).

## Decision Cheat Sheet

| Situation | Do |
|-----------|-----|
| Run async code from `main` | `#[tokio::main] async fn main()` |
| Run two futures concurrently | `tokio::join!` (both) / `select!` (first) |
| Fan out many tasks | `tokio::spawn` per task (needs `Send + 'static`) |
| CPU-bound work in async context | `spawn_blocking` or a thread |
| Time-limit an operation | `tokio::time::timeout` (drops on expiry) |
| Lock shared state across `.await` | `tokio::sync::Mutex` (async-aware) |
| Lock only within sync sections | `std::sync::Mutex` (never across `.await`) |
| Async trait method whose future must be `Send` | return `impl Future + Send` or use `trait-variant` |
| Cancel a spawned Tokio task | retain its `JoinHandle` and call `.abort()` |
| Store/poll a future manually | `Box::pin` / `pin!` |
| Learn threads first | [15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md](15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md) |

## Primary Sources

- Asynchronous Programming in Rust (the async book): https://rust-lang.github.io/async-book/
- std::future::Future: https://doc.rust-lang.org/std/future/trait.Future.html
- std::task::Poll / Context / Waker: https://doc.rust-lang.org/std/task/index.html
- std::pin (Pin/Unpin): https://doc.rust-lang.org/std/pin/index.html
- The Book, Ch. 17 (Async and Await): https://doc.rust-lang.org/book/ch17-00-async-await.html

## Related Guides

- Previous: [13-MACROS-ATTRIBUTES-AND-CODE-GENERATION.md](13-MACROS-ATTRIBUTES-AND-CODE-GENERATION.md)
- Next: [15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md](15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md)
- async fn in traits / dispatch: [07-DISPATCH-TRAIT-OBJECTS-AND-IMPL-TRAIT.md](07-DISPATCH-TRAIT-OBJECTS-AND-IMPL-TRAIT.md)
- Shared state & locks: [16-SMART-POINTERS-INTERIOR-MUTABILITY-AND-SELF-REFERENCE.md](16-SMART-POINTERS-INTERIOR-MUTABILITY-AND-SELF-REFERENCE.md)
