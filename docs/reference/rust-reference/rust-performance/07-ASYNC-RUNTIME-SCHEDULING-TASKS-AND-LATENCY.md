---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-performance:async-runtime-scheduling-tasks-and-latency
kind: guide
module: rust-performance
section: rust-performance
title: Async Runtime Scheduling, Tasks, and Latency
status: source-custody
source_custody: partial
current_path: rust-performance/07-ASYNC-RUNTIME-SCHEDULING-TASKS-AND-LATENCY.md
canonical_path: rust-performance/07-ASYNC-RUNTIME-SCHEDULING-TASKS-AND-LATENCY.md
backsource_ids: [proof-backfill:rust-performance:07-async-runtime-scheduling-tasks-and-latency]
concepts: [async rust, runtime scheduling, tasks, latency, executors, blocking, backpressure]
root_concepts: [async performance]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Async Runtime Scheduling, Tasks, and Latency

## The Big Picture

Rust's `async` syntax defines futures; an ecosystem runtime supplies executors,
reactors, timers, task queues, and often networking. Performance depends on the
runtime configuration and workload, not on `async` as a universal speed feature.

```
+=============================================================================+
|                          ASYNC REQUEST PATH                                 |
|                                                                             |
| reactor completion -> runnable queue -> executor worker -> poll             |
|                                      |                                      |
|                                      +-> Pending: park until wake           |
|                                      +-> Ready: emit response               |
|                                      +-> blocking/long poll: delay peers    |
|                                                                             |
| latency = queueing + scheduling + polls + CPU work + I/O + downstream waits |
+=============================================================================+
```

## What Async Optimizes

Async is primarily a concurrency and resource-utilization model for many
operations that spend time waiting. It can reduce the thread count required to
hold many in-flight operations. It does not make CPU work cheaper.

| Workload | Async fit |
|----------|-----------|
| Many network connections with intermittent I/O | Strong |
| A few long CPU-bound jobs | Weak alone; use bounded worker threads/parallelism |
| Mixed I/O and short CPU work | Strong if CPU sections yield promptly |
| Blocking legacy library calls | Requires isolation from executor workers |
| Strict low-latency with small fixed concurrency | Threads or async may win; measure |

## Polling and Cooperative Scheduling

A future makes progress when polled. It must return `Pending` when blocked and
arrange a wakeup. Executor workers rely on tasks returning control.

```
good task:
poll -> small bounded work -> Pending/Ready -> scheduler can run peers

bad task:
poll -> long CPU loop / blocking syscall ---------------------> returns late
                 all tasks sharing that worker wait
```

Some runtimes implement cooperative budgets or explicit yield mechanisms.
Exact fairness and work-stealing behavior is runtime/version-specific. Treat it
as an implementation contract of the chosen runtime, not a property of Rust
`Future`.

## Blocking Work

For Tokio, the canonical pattern is to isolate blocking work:

```rust
let parsed = tokio::task::spawn_blocking(move || parse_large_file(bytes))
    .await??;
```

`spawn_blocking` uses a runtime-managed blocking pool with its own limits and
shutdown behavior. Once a blocking closure has started, aborting or dropping its
join handle does not generally stop that closure. It is not unbounded free
capacity: acquire a bounded permit before spawning CPU-heavy work, or use a
dedicated/Rayon pool or service-level admission control. Other runtimes expose
different APIs; do not paste Tokio-specific guidance into a runtime-neutral
architecture without labeling it.

## Queueing, Backpressure, and Tail Latency

```
arrival rate
     |
     v
[accept] -> [bounded queue] -> [workers/tasks] -> [downstream]
               |                    |
               | full               | saturated
               v                    v
       reject/defer/backoff      queue delay grows
```

Unbounded spawning converts overload into memory growth and tail latency. Use:

- bounded channels and semaphores;
- explicit concurrency limits per downstream;
- deadlines propagated through the call tree;
- cancellation-safe resource ownership;
- load shedding before queues become self-amplifying;
- batch sizes that balance amortization against wait time.

Measure time in queue separately from service time. A fast handler behind a long
run queue is still a slow service.

## Task Granularity

| Too fine | Too coarse |
|----------|------------|
| scheduling and wake overhead dominates | one poll monopolizes a worker |
| channel traffic and allocation increase | poor fairness and long cancellation delay |
| more shared counters/queues | less parallelism |

Batching can amortize scheduling and syscalls, but it intentionally waits for
more work. Evaluate throughput and tails together.

## Instrumentation

Useful dimensions:

| Dimension | Signal |
|-----------|--------|
| Runnable queue | tasks ready but not scheduled |
| Poll duration | tasks doing too much per poll |
| Wake count | wake storms or redundant notifications |
| Task lifetime | leaked/stuck tasks and cancellation delays |
| Blocking pool | queued/running blocking jobs |
| I/O latency | reactor/downstream waits |
| Channel depth | backpressure and saturation |

Tokio applications can use `tracing` spans and the external `tokio-console`
ecosystem when compiled/configured appropriately. Runtime metrics APIs and feature
flags can evolve; pin versions and account for instrumentation overhead.

```
# POSIX shell; application must install an EnvFilter/tracing subscriber.
RUST_LOG=my_service=trace cargo run --profile profiling --bin my_service

# PowerShell equivalent:
$env:RUST_LOG="my_service=trace"
cargo run --profile profiling --bin my_service
Remove-Item Env:RUST_LOG

# External tool installation; version compatibility matters.
cargo install tokio-console --locked
tokio-console
```

The console requires subscriber instrumentation in the application; the command
alone does not discover arbitrary async tasks.

## Runtime Configuration

| Knob | Potential benefit | Risk |
|------|-------------------|------|
| Worker-thread count | Match runnable CPU work and host quota | Oversubscription or underutilization |
| Blocking-thread limits | Isolate blocking operations | Excess threads/memory or queueing |
| Timer/I/O driver placement | Runtime-specific efficiency | Version-specific behavior |
| Channel capacity | Bounded memory and explicit pressure | Too small can reduce throughput |
| Task-local allocations | Convenience | Per-request overhead and retention |

In containers and Azure, configure against actual CPU quota, not only visible
logical processor count. Some environments expose host CPUs while enforcing a
smaller cgroup/job limit; verify runtime detection and set explicit policy when
necessary.

## Old World -> New World Bridge

| Familiar model | Rust async |
|----------------|------------|
| .NET `Task` + async/await | Rust `Future` + async/await, but executor/runtime is chosen separately |
| ThreadPool work item | Spawned async task; should not block its executor worker |
| IOCP/epoll reactor | Runtime I/O driver/reactor |
| TPL Dataflow bounded block | Bounded async channel/semaphore pipeline |
| ASP.NET request queue and thread starvation | Async run queues, blocking-pool saturation, downstream limits |
| `ConfigureAwait` context concerns | Different model; runtime scheduling and `Send` bounds matter instead |

The universal bridge is cooperative scheduling: code between suspension points
must remain bounded if peers share workers.

## Common Confusion Points

- **Async is not parallel CPU execution by itself.**
- **`await` is not guaranteed to yield if the awaited future is immediately
  ready.**
- **Blocking a runtime worker damages unrelated tasks.**
- **Spawning more tasks does not create capacity.**
- **Runtime behavior is not a Rust language guarantee.**
- **Average latency can improve while p99 worsens from queueing.**
- **Cancellation drops futures; partial side effects still need explicit
  transactional/idempotent design.**

## Decision Cheat Sheet

| Symptom | Investigation/action |
|---------|----------------------|
| High latency, low CPU | Trace I/O, timers, downstream waits, and run queues |
| High CPU on runtime workers | CPU profile poll bodies; split/bound CPU work |
| Executor stalls | Find blocking syscalls/locks; use blocking isolation |
| Memory grows with load | Bound spawn/channel concurrency and inspect task retention |
| p99 spikes under burst | Measure queue depth, admission control, and batch policy |
| Blocking pool saturated | Bound callers or create a dedicated CPU/blocking service |
| Unsure threads vs async | Benchmark representative concurrency and latency SLO |
| Production diagnosis | Combine runtime spans/metrics with [14](14-PRODUCTION-PROFILING-TELEMETRY-CAPACITY-AND-COST.md) |

## Primary Sources

- Rust Async Book: https://rust-lang.github.io/async-book/
- `Future`: https://doc.rust-lang.org/std/future/trait.Future.html
- Tokio runtime: https://docs.rs/tokio/latest/tokio/runtime/
- Tokio bridging with synchronous code: https://tokio.rs/tokio/topics/bridging
- Tokio Console: https://github.com/tokio-rs/console

## Related Guides

- Threading/locks: [08-THREADS-SYNCHRONIZATION-ATOMICS-AND-CONTENTION.md](08-THREADS-SYNCHRONIZATION-ATOMICS-AND-CONTENTION.md)
- I/O behavior: [09-FILES-NETWORKING-BUFFERING-AND-IO.md](09-FILES-NETWORKING-BUFFERING-AND-IO.md)
- Language model: [../rust-language/14-ASYNC-FUTURES-AND-PINNING.md](../rust-language/14-ASYNC-FUTURES-AND-PINNING.md)
