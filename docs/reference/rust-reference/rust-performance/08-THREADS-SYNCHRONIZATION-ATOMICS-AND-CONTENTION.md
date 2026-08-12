---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-performance:threads-synchronization-atomics-and-contention
kind: guide
module: rust-performance
section: rust-performance
title: Threads, Synchronization, Atomics, and Contention
status: source-custody
source_custody: partial
current_path: rust-performance/08-THREADS-SYNCHRONIZATION-ATOMICS-AND-CONTENTION.md
canonical_path: rust-performance/08-THREADS-SYNCHRONIZATION-ATOMICS-AND-CONTENTION.md
backsource_ids: [mdloom-backfill:rust-performance:08-threads-synchronization-atomics-and-contention]
concepts: [threads, synchronization, atomics, contention, mutexes, channels, false sharing]
root_concepts: [concurrency performance]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Threads, Synchronization, Atomics, and Contention

## The Big Picture

Rust prevents data races in safe code; it does not prevent lock convoys, false
sharing, oversubscription, starvation, or logically racy protocols.

```
+=============================================================================+
|                         PARALLEL WORK PATH                                  |
|                                                                             |
| input -> partition -> worker threads -> combine output                      |
|              |             |                 |                              |
|              |             +-> local work   +-> lock/channel/atomic         |
|              |                    |                  |                      |
|              |                    v                  v                      |
|              |                 useful CPU      synchronization cost         |
|              |                                       |                      |
|              +-> load balance                         +-> waiting/parking   |
|                                                      +-> cache-line traffic |
|                                                      +-> wakeups/fairness   |
|                                                                             |
| speedup <= serial work + imbalance + scheduling + communication + contention|
+=============================================================================+
```

## Start with Decomposition

Parallelism pays when each work unit is large enough to amortize scheduling and
communication, and partitions are balanced.

| Pattern | Strength | Failure mode |
|---------|----------|--------------|
| Data parallel (`rayon`) | Easy partitioning and work stealing | Tiny tasks, shared mutation, skew |
| Dedicated worker threads | Stable ownership and affinity | Idle threads, manual lifecycle |
| Message passing | Clear ownership transfer | Queueing, copies, backpressure |
| Shared lock-protected state | Simple central invariant | Contention and convoying |
| Atomics | Low-level counters/state machines | Cache-line ping-pong, ordering bugs |

Measure single-thread performance first. A parallel implementation that does
twice the total work may still look faster at one core count and collapse later.

## Locks: Cost Is Mostly Contention

An uncontended mutex is often cheap. The expensive path includes cache
coherence, parking, scheduler wakeups, and queueing behind long critical
sections.

```
thread A: [ acquire ][--------- critical section ---------][ release ]
thread B:           [ wait / spin / park ----------------][ acquire ]
thread C:                [ wait / spin / park ----------------------]
```

Improve in this order:

1. remove shared mutation;
2. shard state by key/worker;
3. move work outside the critical section;
4. batch updates;
5. choose a more suitable primitive;
6. tune implementation details only after measuring.

`RwLock` is not automatically better for read-heavy work. Reader bookkeeping,
writer fairness, and long readers can make it slower than a mutex. Standard
library fairness details are implementation/platform-sensitive.

## Channels and Ownership Transfer

Channels replace shared data with shared queues. They still synchronize.

| Choice | Benefit | Risk |
|--------|---------|------|
| Bounded channel | Backpressure and bounded memory | Producers block/fail when full |
| Unbounded channel | Simple burst absorption | Memory growth and hidden queue latency |
| Per-worker queue | Locality and less central contention | Work imbalance |
| Work stealing | Dynamic balance | Steal overhead and less deterministic locality |

Move buffers through channels instead of serializing/cloning them when ownership
allows. Batch small messages if queue operations dominate, but include batching
delay in tail-latency measurements.

## Atomics and Memory Ordering

Atomics are for synchronization protocols, not a generic "faster mutex."

| Ordering | Broad intent | Caution |
|----------|--------------|---------|
| `Relaxed` | Atomicity only; counters where ordering is irrelevant | Does not publish other memory |
| `Acquire` | Observe writes released before a synchronization event | Usually paired with release operation |
| `Release` | Publish prior writes before an event | Does not constrain later reads like seq-cst |
| `AcqRel` | Read-modify-write with both roles | Protocol still needs proof |
| `SeqCst` | Single global order among seq-cst operations | Stronger/easier to reason about, sometimes costlier |

This table is orientation, not a substitute for a proof. Use standard
concurrency patterns, document invariants, and test with tools such as Loom.

```rust
use std::sync::atomic::{AtomicU64, Ordering};

static REQUESTS: AtomicU64 = AtomicU64::new(0);

fn count_request() {
    REQUESTS.fetch_add(1, Ordering::Relaxed); // metric only; no data publication
}
```

`Relaxed` is valid here only because the value is an approximate independent
counter and does not guard access to other data. The atomic operation itself is
not approximate, but readers can observe a stale snapshot and the counter wraps
on integer overflow; neither property may be used for synchronization.

## False Sharing

Independent atomics can contend if they share a cache line:

```
cache line:
+----------------------+----------------------+
| worker_0_counter     | worker_1_counter     |
+----------------------+----------------------+
      core 0 writes <---- coherence ----> core 1 writes
```

Sharding counters and padding/alignment can help, but cache-line size and
alignment are target-specific. Aggregate less frequently and benchmark memory
cost. Do not assume a hard-coded line size is universal.

## Oversubscription and Affinity

Combining an async runtime, Rayon, native libraries, and blocking pools can
create several thread pools:

```
runtime workers + rayon workers + blocking pool + library threads > CPU quota
```

Context switching and cache churn can dominate. Inventory every pool and size
against actual container/VM CPU quota. Affinity can reduce migration in
specialized workloads, but it can also defeat OS load balancing and interact
badly with NUMA. Treat it as a measured deployment policy.

## Observing Contention

```
# Linux examples; privileges/kernel support vary.
perf lock record -- ./target/profiling/my_app
perf lock report

perf sched record -- ./target/profiling/my_app
perf sched timehist
```

On Windows, WPR/WPA and PerfView can correlate CPU sampling with context
switches, waits, and thread activity. Standard profiler stacks may show lock
implementation frames but not the full queueing story; capture a scheduler
timeline. ThreadSanitizer detects some data races, not performance contention,
and Rust sanitizer workflows are toolchain/target sensitive.

## Validating Lock-Free Protocols

The Loom crate explores possible thread interleavings under a modeled
synchronization environment:

```rust
loom::model(|| {
    // Construct Loom synchronization primitives and assert protocol invariants.
});
```

Loom is an external test dependency and requires substituting its modeled
primitives. It supports correctness evidence, not native performance
measurement. Benchmark the production primitives separately.

## Old World -> New World Bridge

| Prior art | Rust |
|-----------|------|
| `lock`/Monitor, `Mutex`, SRWLock | `std::sync::Mutex` / `RwLock` or ecosystem alternatives |
| TPL/PLINQ | Rayon data parallelism |
| `Interlocked` | `std::sync::atomic` |
| ConcurrentQueue / channels | `std::sync::mpsc` and ecosystem bounded channels |
| ETW contention/context switches | Same OS-level analysis for Rust native threads |
| ThreadPool starvation | Oversubscribed executor/pool and blocking work |

Rust's type system establishes `Send`/`Sync` boundaries. Throughput and fairness
remain runtime engineering questions.

## Common Confusion Points

- **Data-race freedom is not contention freedom.**
- **Lock-free is not wait-free and not automatically faster.**
- **`RwLock` can lose to `Mutex`.**
- **Atomics can serialize through cache coherence.**
- **More threads can reduce throughput.**
- **A short critical section can still convoy at high arrival rates.**
- **ThreadSanitizer does not measure contention.**
- **Results depend on core topology, NUMA, OS scheduler, and quota.**

## Decision Cheat Sheet

| Symptom | First response |
|---------|----------------|
| Scaling flattens with cores | Measure serial fraction, imbalance, and shared-state contention |
| Lock dominates profile | Shorten/shard/remove critical section before swapping mutex implementation |
| Queue memory grows | Bound the channel and define overload behavior |
| Atomics dominate CPU | Shard/batch updates and inspect false sharing |
| Tail spikes under load | Capture waits/context switches; look for convoys |
| Many thread pools | Consolidate or size explicitly to quota |
| Lock-free algorithm planned | Prove ordering/invariants with Loom/modeling, then benchmark |
| Need simple CPU parallelism | Start with Rayon and immutable partitions |

## Primary Sources

- Rust atomics and locks: https://doc.rust-lang.org/std/sync/
- Rustonomicon, atomics: https://doc.rust-lang.org/nomicon/atomics.html
- Rayon: https://docs.rs/rayon/
- Loom: https://docs.rs/loom/
- Linux perf lock: https://man7.org/linux/man-pages/man1/perf-lock.1.html

## Related Guides

- Async scheduling: [07-ASYNC-RUNTIME-SCHEDULING-TASKS-AND-LATENCY.md](07-ASYNC-RUNTIME-SCHEDULING-TASKS-AND-LATENCY.md)
- Data locality: [05-DATA-LAYOUT-CACHE-LOCALITY-SIMD-AND-VECTORIZATION.md](05-DATA-LAYOUT-CACHE-LOCALITY-SIMD-AND-VECTORIZATION.md)
- Language model: [../rust-language/15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md](../rust-language/15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md)
