---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-language:threads-send-sync-locks-and-atomics
kind: guide
module: rust-language
section: languages
title: Threads, Send, Sync, Locks, and Atomics
status: source-custody
source_custody: partial
current_path: rust-language/15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md
canonical_path: rust-language/15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md
backsource_ids: [mdloom-backfill:rust-language:15-threads-send-sync-locks-and-atomics]
concepts: [threads, Send, Sync, channels, Mutex, RwLock, atomics, memory ordering, scoped threads, rayon, fearless concurrency]
root_concepts: [concurrency]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Threads, Send, Sync, Locks, and Atomics

Rust's "fearless concurrency" is not a slogan — it is a direct corollary of the
borrow rules ([04](04-BORROWING-REFERENCES-AND-LIFETIMES.md)). Data races require
aliasing + mutation without synchronization; the aliasing-XOR-mutation rule
forbids exactly that, and two marker traits — **`Send`** and **`Sync`** — extend
the guarantee across thread boundaries. In safe Rust, backed by sound unsafe
implementations, data races are rejected rather than becoming runtime UB.
Incorrect unsafe code or an unsound manual `Send`/`Sync` impl can violate that
contract. This guide covers OS threads (async is
[14](14-ASYNC-FUTURES-AND-PINNING.md)).

```
+===============================================================================+
|                 SEND + SYNC = THREAD SAFETY AS A TYPE PROPERTY                |
+===============================================================================+

  Send : ownership of T can MOVE to another thread   (most types)
  Sync : &T can be SHARED across threads              (T is Sync iff &T is Send)
  -> auto-derived; a struct is Send/Sync iff all its fields are

  NOT Send/Sync              Send + Sync                THREAD-SAFE SHARING RECIPE
  -------------              -----------                --------------------------
  Rc<T>   (non-atomic rc)    Arc<T> if T: Send+Sync     Arc<Mutex<T>>  : shared + mutable
  RefCell (unsync int.mut.)  Mutex<T> RwLock<T>         Arc<T>         : shared + read-only
  *mut T  (raw pointer)      atomics (AtomicUsize..)    channels       : transfer ownership
                                                        (mpsc / crossbeam)

  COMMUNICATE                                MEMORY ORDERINGS (atomics)
  -----------                                --------------------------
  channels:  send/recv, MOVE ownership       Relaxed  : independent metrics only
    "share memory by communicating"          Acquire/Release : lock-like pairing
  shared state: Arc<Mutex<T>>                SeqCst   : total order among SeqCst ops
    "communicate by sharing memory"          (stronger order cannot fix a wrong protocol)
```

## Spawning Threads

```rust
use std::thread;
let handle = thread::spawn(move || {       // `move`: closure owns its captures
    heavy_work();
    42
});
let result = handle.join().unwrap();       // wait for it; propagate panic as Err
```

`thread::spawn` requires the closure to be `Send + 'static` — it may run after the
spawning function returns, so it cannot borrow locals. That is why you see `move`
and why data crossing the boundary must be owned or `Arc`-shared.

## Send and Sync: The Two Marker Traits

- **`Send`** — a type whose ownership may be transferred to another thread.
  Almost everything is `Send`; the notable exceptions are `Rc<T>` (its reference
  count is non-atomic) and raw pointers.
- **`Sync`** — a type `T` such that `&T` is `Send`, i.e. it is safe to share a
  reference across threads. `T: Sync` iff `&T: Send`. `RefCell<T>` is `Send` but
  **not** `Sync` (its borrow flags are non-atomic).

Both are **auto-traits**: the compiler derives them structurally — a struct is
`Send`/`Sync` iff all fields are. You almost never implement them by hand;
doing so is `unsafe` and asserts a safety invariant the compiler cannot check
([17](17-UNSAFE-RUST-FFI-AND-ABI.md)). The payoff: `thread::spawn`'s `Send +
'static` bound mechanically rejects sending a non-thread-safe type across
threads.

## Two Concurrency Philosophies

### Message Passing (channels)

"Do not communicate by sharing memory; share memory by communicating." A channel
**moves** ownership between threads, sidestepping shared-state bugs entirely.

```rust
use std::sync::mpsc;                       // multi-producer, single-consumer
let (tx, rx) = mpsc::channel();
for i in 0..4 {
    let tx = tx.clone();
    thread::spawn(move || tx.send(i * i).unwrap());
}
drop(tx);                                  // close the last sender
for got in rx { println!("{got}"); }       // iterates until all senders dropped
```

`std::sync::mpsc` is single-consumer; for multi-consumer or higher performance
use the `crossbeam-channel` crate (which also gives `select`).

### Shared State (Arc + Mutex/RwLock)

When threads must share the *same* mutable data, wrap it: `Arc` provides shared
ownership (atomic refcount), `Mutex`/`RwLock` provides synchronized access.

```rust
use std::sync::{Arc, Mutex};
let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];
for _ in 0..8 {
    let c = Arc::clone(&counter);
    handles.push(thread::spawn(move || {
        let mut n = c.lock().unwrap();     // MutexGuard; unlocks on drop (RAII)
        *n += 1;
    }));                                   // guard dropped here -> lock released
}
for h in handles { h.join().unwrap(); }
assert_eq!(*counter.lock().unwrap(), 8);
```

`Mutex<T>` in Rust *wraps the data*, so you cannot access the data without
holding the lock — the "forgot to lock" bug is structurally impossible. The
`MutexGuard` releases the lock on `Drop`, so there is no manual unlock to forget.
`RwLock<T>` allows many readers or one writer. Rust `Mutex` is **not reentrant**;
a second same-thread `lock()` is specified not to return normally, but the
platform implementation may deadlock or panic. Do not rely on either outcome.
Lock **poisoning**: if a thread panics while holding the lock, `.lock()` returns
`Err` so other threads learn the data may be inconsistent (recover with
`into_inner`/`get_mut` or `unwrap`).

## Atomics and Memory Ordering

For simple shared counters/flags, atomics (`AtomicUsize`, `AtomicBool`, ...) avoid
lock overhead. Each operation takes an `Ordering`:

| Ordering | Guarantee | Use for |
|----------|-----------|---------|
| `Relaxed` | atomicity only, no cross-variable ordering | independent counters, statistics |
| `Acquire` (loads) / `Release` (stores) | pair to publish/consume data | lock-free handoff of other data |
| `AcqRel` | both, for read-modify-write | `fetch_*` that both publishes and consumes |
| `SeqCst` | single total order across all `SeqCst` ops | simplify a protocol already proved correct |

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
static COUNT: AtomicUsize = AtomicUsize::new(0);
COUNT.fetch_add(1, Ordering::Relaxed);     // fine: just a counter
```

Rust's atomics follow the **C++20 memory model**. Getting orderings wrong is a
classic source of subtle, platform-dependent bugs — if you are not certain,
reach for a `Mutex` or channel. `SeqCst` simplifies reasoning about a protocol
that is already correct; it cannot repair missing synchronization or a logically
invalid lock-free algorithm. Validate nontrivial protocols with `loom` (a
concurrency model checker) and appropriate dynamic tools. Do not hand-roll
lock-free structures without that rigor.

## Scoped Threads and Data Parallelism

**Scoped threads** (`std::thread::scope`, stable since Rust 1.63) let child
threads **borrow** local data because the scope guarantees they all finish before
it returns — no `Arc`, no `'static` needed:

```rust
let data = vec![1, 2, 3];
std::thread::scope(|s| {
    s.spawn(|| println!("{:?}", &data));   // borrows `data` directly — allowed
    s.spawn(|| println!("{}", data.len()));
});                                        // all scoped threads joined here
```

**rayon** turns sequential iterators into parallel ones by changing `iter()` to
`par_iter()` — data parallelism with a work-stealing pool and no manual thread
management:

```rust
use rayon::prelude::*;
let sum: i64 = (1..=1_000_000).into_par_iter().map(|x| x * x).sum();
```

## Old World -> New World Bridge

| Old world | Rust | Difference |
|-----------|------|-----------|
| `System.Threading.Thread` | `std::thread` | Data crossing must be `Send` |
| `lock (obj) { }` (.NET) | `Mutex<T>` + guard | The lock *wraps the data*; can't access unlocked |
| `Monitor`/reentrant lock | `Mutex` (**non-reentrant**) | Same-thread re-lock does not return normally; may deadlock or panic |
| `ReaderWriterLockSlim` | `RwLock<T>` | Same many-readers/one-writer |
| `ConcurrentQueue` / channels | `mpsc` / `crossbeam-channel` | Channels move ownership |
| `Interlocked.*` | `Atomic*` + `Ordering` | Explicit memory ordering |
| `Parallel.For` / PLINQ | `rayon` `par_iter()` | Drop-in parallel iterators |
| data race = runtime UB (C/C++) | safe Rust rejects data races | Sound `Send`/`Sync` boundaries make it static |
| `shared_ptr` (atomic refcount) | `Arc<T>` | `Rc` is the non-atomic single-thread version |

The defining difference from every GC or C++ world is that safe Rust makes
thread transfer and shared access type-checked. `Arc<Mutex<T>>` is the canonical
"shared mutable state" idiom, and the guard's RAII drop means there is no manual
unlock call to forget.

## Common Confusion Points

- **`Rc` vs `Arc`.** `Rc` is single-thread (non-atomic count, faster); `Arc` is
  atomically reference-counted, but cross-thread use still requires the inner
  `T` to satisfy `Send`/`Sync`. Sending an `Rc` across threads is a compile error
  — switch to an appropriate `Arc<T>`
  ([16](16-SMART-POINTERS-INTERIOR-MUTABILITY-AND-SELF-REFERENCE.md)).
- **Mutex is non-reentrant.** A second same-thread `lock()` does not return
  normally; the implementation may deadlock or panic. There is no recursive
  mutex in std.
- **Lock poisoning.** A panic while holding a lock poisons it; subsequent
  `.lock()` returns `Err`. Handle or `unwrap` deliberately.
- **`Arc<Mutex<T>>` vs channel.** If you are transferring ownership of work
  items, a channel is cleaner than shared state; use `Arc<Mutex<T>>` for genuine
  shared mutable state.
- **Memory orderings are subtle.** `Relaxed` gives no cross-variable ordering.
  A stronger ordering does not invent a correct protocol; when unsure, use a
  lock or channel and verify lock-free code with `loom`.
- **Don't hold a std `Mutex` across `.await`.** In async, use
  `tokio::sync::Mutex` ([14](14-ASYNC-FUTURES-AND-PINNING.md)).
- **Scoped threads remove the `'static` tax** for fork-join borrowing.

## Decision Cheat Sheet

| Situation | Use |
|-----------|-----|
| Share ownership across threads | `Arc<T>` |
| Share ownership within one thread | `Rc<T>` |
| Shared mutable state | `Arc<Mutex<T>>` (or `Arc<RwLock<T>>` for read-heavy) |
| Transfer work items between threads | channel (`mpsc` / `crossbeam-channel`) |
| Independent statistics counter | atomic `fetch_add(..., Relaxed)` after confirming it publishes no other data |
| Readiness/publication flag guarding other data | a proven Release-store/Acquire-load protocol; prefer a lock/channel when unsure |
| Unsure which atomic ordering is correct | use a lock/channel; do not substitute `SeqCst` for a missing protocol |
| Fork-join borrowing locals | `std::thread::scope` |
| Parallelize a data pipeline | `rayon` `par_iter()` |
| Async shared state | `tokio::sync::Mutex` ([14](14-ASYNC-FUTURES-AND-PINNING.md)) |
| Assert thread-safety unsafely | manual `Send`/`Sync` impl (needs `unsafe`, rare) |

## Primary Sources

- The Book, Ch. 16 (Fearless Concurrency): https://doc.rust-lang.org/book/ch16-00-concurrency.html
- std::thread (incl. scope): https://doc.rust-lang.org/std/thread/index.html
- std::sync (Arc, Mutex, RwLock, mpsc): https://doc.rust-lang.org/std/sync/index.html
- std::sync::atomic (orderings): https://doc.rust-lang.org/std/sync/atomic/index.html
- std::marker::Send / Sync: https://doc.rust-lang.org/std/marker/trait.Send.html
- The Rustonomicon — Concurrency: https://doc.rust-lang.org/nomicon/concurrency.html

## Related Guides

- Previous: [14-ASYNC-FUTURES-AND-PINNING.md](14-ASYNC-FUTURES-AND-PINNING.md)
- Next: [16-SMART-POINTERS-INTERIOR-MUTABILITY-AND-SELF-REFERENCE.md](16-SMART-POINTERS-INTERIOR-MUTABILITY-AND-SELF-REFERENCE.md)
- Borrow rules underpin all of this: [04-BORROWING-REFERENCES-AND-LIFETIMES.md](04-BORROWING-REFERENCES-AND-LIFETIMES.md)
- Unsafe Send/Sync assertions: [17-UNSAFE-RUST-FFI-AND-ABI.md](17-UNSAFE-RUST-FFI-AND-ABI.md)
