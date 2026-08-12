---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-production-engineering:async-runtime-concurrency-capacity
kind: guide
module: rust-production-engineering
section: rust-production-engineering
title: Async Runtime, Concurrency, and Capacity
status: source-custody
source_custody: partial
current_path: rust-production-engineering/06-ASYNC-RUNTIME-CONCURRENCY-AND-CAPACITY.md
canonical_path: rust-production-engineering/06-ASYNC-RUNTIME-CONCURRENCY-AND-CAPACITY.md
backsource_ids: [mdloom-backfill:rust-production-engineering:06-async-runtime-concurrency-capacity]
concepts: [async runtime, concurrency, capacity, executors, threads, bounded queues, admission control, saturation]
root_concepts: [concurrency]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Async Runtime, Concurrency, and Capacity

## The Big Picture

Concurrency is a capacity allocation decision. Rust supplies threads,
synchronization primitives, and the `Future` contract, but no standard async
executor. A production design must choose how work is scheduled, where blocking
is allowed, and which finite resource rejects work first.

```
+============================================================================+
|                      CONCURRENCY AND CAPACITY                              |
|                                                                            |
|  demand --> admission --> bounded queue --> workers/tasks --> dependency   |
|               |              |                |                |           |
|               v              v                v                v           |
|           reject/shed     wait budget      CPU/memory        pool/socket   |
|                                                                            |
|  scheduler choices: OS threads | worker pool | async executor | hybrid     |
|  invariants: bounded work, visible saturation, cancellation, fair sharing  |
+============================================================================+
```

The executor does not create capacity. It changes the cost and scheduling model
of waiting work. CPU, memory, file descriptors, connections, and downstream
throughput remain finite.

## Choose the Execution Model from the Work

| Workload | Strong default | Why |
|---|---|---|
| Small CLI or batch | synchronous | minimal lifecycle and dependency surface |
| CPU-bound parallel stages | fixed worker pool or Rayon | explicit CPU parallelism |
| Many mostly-waiting sockets | async runtime | cheap suspended tasks and I/O integration |
| Blocking legacy/native API | dedicated bounded thread pool | isolate blocking from async workers |
| Mixed service | async I/O plus bounded CPU/blocking pools | separate capacity domains |

Do not migrate blocking work into `async fn` and assume it became non-blocking.
An executor worker blocked in a system call cannot poll unrelated tasks.

## Capacity Model

Little's Law gives a useful first-order check:

```
concurrency ~= throughput * time_in_system

500 requests/s * 0.200 s = about 100 in-flight requests
500 requests/s * 2.000 s = about 1,000 in-flight requests
```

Tail latency, retries, and queues increase residence time and therefore
concurrency. Define limits per scarce resource, not only one global request
limit.

| Capacity boundary | Evidence |
|---|---|
| Admission semaphore | active permits, rejected/waiting work |
| Queue | depth, oldest age, enqueue rejection |
| Worker pool | busy workers, queue wait, execution time |
| Connection pool | checked-out, waiters, acquisition timeout |
| Memory | per-request estimate, allocator/process working set |

## Executable Bounded Worker Pool

This standard-library example uses a zero-growth bounded channel. It proves that
admission can fail immediately instead of allocating an unbounded backlog.

```rust
use std::{
    sync::mpsc::{sync_channel, TrySendError},
    thread,
    time::Duration,
};

fn main() {
    let (tx, rx) = sync_channel::<u64>(2);
    let worker = thread::spawn(move || {
        while let Ok(job) = rx.recv() {
            thread::sleep(Duration::from_millis(100));
            println!("completed {job}");
        }
    });

    for job in 0..10 {
        match tx.try_send(job) {
            Ok(()) => println!("accepted {job}"),
            Err(TrySendError::Full(job)) => println!("shed {job}: queue full"),
            Err(TrySendError::Disconnected(job)) => {
                eprintln!("worker unavailable for {job}");
                break;
            }
        }
    }

    drop(tx);
    worker.join().expect("worker panicked");
}
```

Run with `rustc pool.rs && ./pool` (PowerShell:
`rustc pool.rs; .\pool.exe`). A production pool needs multiple workers,
per-job deadlines, panic supervision, metrics, and shutdown policy, but the
bounded-admission property is already testable.

## Async Runtime Selection

| Criterion | Questions |
|---|---|
| Protocol ecosystem | Do required clients/servers assume a runtime? |
| I/O model | Which OS targets and drivers are supported? |
| Scheduling | Work stealing, current-thread mode, fairness controls? |
| Time and cancellation | Are timer and shutdown semantics sufficient? |
| Blocking integration | Is there a bounded blocking path? |
| Operations | Runtime metrics, task dumps, tracing integration? |

Tokio has broad integration across current network-service libraries and is
often a pragmatic choice when required clients or servers already depend on
it. Smol-family runtimes, async-std in ecosystems that still require it,
Embassy for embedded targets, and custom executors address different
constraints. Verify current maintenance, target support, and dependency
compatibility rather than selecting from reputation alone. Runtime-neutral
libraries should expose futures and avoid starting a hidden global runtime
unless their contract requires one.

## Structured Concurrency and Task Ownership

Every spawned task needs an owner, completion policy, and failure path.

```
request scope
  |
  +--> child A --+
  +--> child B --+--> join, cancel siblings on required failure
  |
  +--> detached task? only with named process-level owner and supervision
```

Detached background tasks are process components, not incidental futures. Track
their handles, propagate shutdown, and decide whether a panic or unexpected exit
should stop the process.

## Library, Runtime, and Platform Choices

| Layer | Choices and boundary |
|---|---|
| Library | bounded channels, semaphores, Rayon, client/server abstractions |
| Runtime | Tokio/async-std/smol/Embassy/custom executor, timers, I/O driver |
| Platform | CPU/memory limits, process count, file descriptors, autoscaling |

Platform autoscaling reacts after signals and startup delay. It cannot replace
local admission control. Likewise, a semaphore protects only the resource it
actually surrounds.

## Old World -> New World Bridge

The universal bridge is from **thread count** to **work-in-system accounting**.
Async makes parked work cheaper, so task count can become enormous before CPU
looks saturated. Queue depth, memory, and downstream permits become first-class
capacity signals.

.NET's thread pool and `Task` scheduler provide a familiar comparison, but a
Rust future is lazy and the executor is explicitly selected. Windows IOCP,
Linux readiness APIs, and embedded interrupt executors are runtime/platform
implementations of the same scheduling contract.

## Decision Cheat Sheet

| Use | When |
|---|---|
| Synchronous code | concurrency is small or waiting is not dominant |
| Fixed OS-thread pool | blocking or CPU work has a clear bounded parallelism |
| Rayon | data-parallel CPU work fits fork/join semantics |
| Async runtime | many concurrent operations spend most time waiting |
| Current-thread executor | deterministic ownership or constrained environment matters |
| Semaphore | one scarce resource needs an in-flight cap |
| Bounded queue | buffering is useful but memory and wait must be capped |
| Immediate shedding | queued work would exceed caller value or system budget |

## Common Confusion Points

- **Tasks are not free.** Each retains state, allocations, references, and
  telemetry context.
- **`spawn_blocking` is not infinite capacity.** Bound calls before dispatching.
- **A global concurrency limit may still overload one dependency.** Isolate
  permits by resource and priority.
- **Async mutexes are not always better.** Use the primitive whose hold time and
  suspension behavior match the critical section.
- **Autoscaling is not backpressure.** It is delayed capacity adjustment.

## Primary Sources

- Rust `Future`: https://doc.rust-lang.org/std/future/trait.Future.html
- Rust concurrency primitives: https://doc.rust-lang.org/std/sync/
- Tokio runtime: https://docs.rs/tokio/latest/tokio/runtime/
- Tokio bridging sync and async: https://tokio.rs/tokio/topics/bridging
- Rayon: https://docs.rs/rayon/

## Related Guides

- Previous: [05-PROCESS-LIFECYCLE-SIGNALS-AND-GRACEFUL-SHUTDOWN.md](05-PROCESS-LIFECYCLE-SIGNALS-AND-GRACEFUL-SHUTDOWN.md)
- Next: [07-TIMEOUTS-RETRIES-BACKPRESSURE-AND-RESILIENCE.md](07-TIMEOUTS-RETRIES-BACKPRESSURE-AND-RESILIENCE.md)
- Language async model: [../rust-language/14-ASYNC-FUTURES-AND-PINNING.md](../rust-language/14-ASYNC-FUTURES-AND-PINNING.md)
- Capacity telemetry: [03-METRICS-HEALTH-AND-TELEMETRY-DESIGN.md](03-METRICS-HEALTH-AND-TELEMETRY-DESIGN.md)
