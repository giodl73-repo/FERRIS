---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-security-assurance:concurrency-resource-exhaustion-and-denial-of-service
kind: guide
module: rust-security-assurance
section: security-engineering
title: Concurrency, Resource Exhaustion, and Denial of Service
status: source-custody
source_custody: partial
current_path: rust-security-assurance/08-CONCURRENCY-RESOURCE-EXHAUSTION-AND-DENIAL-OF-SERVICE.md
canonical_path: rust-security-assurance/08-CONCURRENCY-RESOURCE-EXHAUSTION-AND-DENIAL-OF-SERVICE.md
backsource_ids: [proof-backfill:rust-security-assurance:08-concurrency-resource-exhaustion-and-denial-of-service]
concepts: [concurrency, denial of service, resource exhaustion, backpressure, cancellation]
root_concepts: [denial of service]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Concurrency, Resource Exhaustion, and Denial of Service

Assuming the reachable unsafe foundation is sound, Rust's type system prevents
data races in well-formed safe code. It does not prevent deadlock, starvation,
priority inversion, task leaks, unbounded queues, algorithmic complexity
attacks, or a valid request consuming all capacity. Availability assurance
turns every resource into a budget with admission, backpressure, cancellation,
observability, and recovery.

## The Big Picture

```
+============================================================================+
|                         AVAILABILITY CONTROL LOOP                          |
+============================================================================+
| request -> authenticate -> admit -> queue -> execute -> emit response      |
|              |            |       |         |                              |
|              |         rate/quota |      CPU/time/IO budget                |
|              |                    |                                        |
|              +--> identity cost   +--> bounded capacity/backpressure       |
+----------------------------------------------------------------------------+
| observe saturation -> shed/degrade -> cancel/reclaim -> recover/rebalance  |
+============================================================================+
```

A timeout at the outer edge is not enough if cancelled work keeps running or
resources are retained behind it.

## Build a Resource Ledger

| Resource | Typical exhaustion path | Control |
|----------|-------------------------|---------|
| Heap | attacker-sized buffers, decompression, retained cache | pre-allocation caps, quotas, eviction |
| CPU | expensive parse/hash/regex/compression | work budget, restricted algorithm, admission |
| Threads/tasks | spawn per request, blocked workers, task leak | bounded executor/semaphore, structured ownership |
| Queue | producer faster than consumer | bounded channel and explicit full behavior |
| File descriptors/sockets | slow peers, leaks, fan-out | concurrency cap, deadline, RAII, OS limits |
| Locks | long critical section, cycle, priority inversion | hierarchy, short scope, partitioning, try/timeout where meaningful |
| Logs/metrics | attacker-controlled cardinality or volume | sampling, field limits, cardinality budget |
| External service | retry storm and fan-out | retry budget, jitter, circuit breaking, bulkhead |

Quantify budgets per request, identity/tenant, process, and fleet. "Bounded" with
no number or owner is not a release criterion.

Attacker-controlled map keys deserve a hasher review. Rust's default `HashMap`
hasher is designed to resist collision attacks, but alternative fast hashers may
make different guarantees and no hasher bounds cardinality or allocation.
Record the chosen `BuildHasher`, input trust, item cap, and target performance.

## Bounded Concurrency

The standard library provides a bounded synchronous channel:

```rust
use std::sync::mpsc::sync_channel;

let (tx, rx) = sync_channel::<Vec<u8>>(128);
// The channel buffers at most 128 values. send blocks when that buffer is full;
// choose blocking behavior only when it cannot deadlock the execution model.
drop((tx, rx));
```

Async runtimes provide bounded channels and semaphores, but exact APIs and
cancellation semantics are runtime/version-specific. Pin the runtime, test the
full/closed paths, and document whether producers block, fail, drop, or shed.

```
unbounded spawn                         bounded admission
request -> task -> task -> task         request -> permit? -> task
             |                                      |
             v                                      +-- no -> reject/defer
        scheduler/memory collapse                    +-- yes -> release on drop
```

## Deadlock and Liveness

Safe locks can deadlock. Avoid holding a synchronous mutex guard across an
`.await` point unless the lock type and design explicitly support that pattern;
the guard can block unrelated progress and may violate executor expectations.

| Liveness failure | Design response |
|------------------|-----------------|
| Lock cycle | global lock ordering or eliminate nested acquisition |
| Starvation | fairer scheduling/partitioning; bound monopolization |
| Priority inversion | minimize shared lock and isolate priority classes |
| Lost wakeup/custom atomic bug | established primitives, Loom/model tests |
| Blocking call on async worker | dedicated blocking pool with bounded permits |
| Cancellation leak | ownership tree and drop-path tests |

Thread safety is not liveness. A system can be perfectly data-race-free while
making no progress.

## Deadlines, Cancellation, and Retries

```
end-to-end deadline
   |
   +-- parser budget
   +-- queue wait budget
   +-- downstream call budget
   +-- retry budget (inside original deadline)
   +-- cleanup/reclamation obligation
```

Propagate a deadline rather than resetting a fresh timeout at every hop. Retry
only operations with a clear idempotency or deduplication contract. Apply
exponential backoff with jitter according to the surrounding protocol; cap
attempts and total elapsed time.

Cancellation safety is operation-specific. Dropping a future stops polling it,
but external side effects may already have occurred. Document commit points and
compensation/idempotency behavior.

## Panic, OOM, and Process Strategy

| Failure | Decision needed |
|---------|-----------------|
| Request panic | isolate if unwind profile and boundary permit; do not assume arbitrary state is recoverable |
| `panic = "abort"` | supervisor/process restart strategy |
| Allocation failure | infallible allocation paths commonly abort through the allocation-error handler; use fallible reserve APIs where recovery is a real design and control capacity before this point |
| Poisoned lock | decide whether invariant may be repaired or process must fail |
| Repeated crash input | quarantine/rate-limit plus patch; avoid endless restart loop |

Product recovery claims must match the panic profile, allocator, runtime, and
supervisor actually deployed.

## Test Availability as a Security Property

Challenge:

- maximum encoded and expanded inputs;
- queue-full, permit exhaustion, and slow-consumer paths;
- downstream timeout and retry amplification;
- lock-order and cancellation interleavings;
- per-tenant fairness under skewed load;
- restart, drain, and degraded-mode behavior;
- telemetry cardinality under attacker-chosen identifiers.

Record throughput and latency near saturation, not only at average load.

## Old World -> New World Bridge

| Established practice | Rust expression |
|----------------------|-----------------|
| Thread-pool max concurrency | bounded Tokio/rayon/custom pool or semaphore |
| TPL cancellation token | runtime-specific cancellation plus drop/commit semantics |
| Bounded producer-consumer queue | `sync_channel` or bounded async channel |
| Circuit breaker/bulkhead | same resilience patterns; Rust ownership helps release permits |
| CLR thread-safe collection | `Send`/`Sync` establishes data-race safety, not workload fairness |

Azure service quotas, API Management rate limits, Front Door/WAF controls, and
platform autoscaling can absorb or reject load. They supplement in-process
budgets; autoscaling cannot repair algorithmic amplification or an unbounded
per-request allocation.

## Common Confusion Points

- **"Safe concurrency means no races."** No data races; logical races and
  liveness failures remain.
- **"Async is cheap, so tasks can be unbounded."** Every task retains state and
  scheduler work.
- **"Timeout cancels the operation."** It may only stop waiting; verify
  propagation and cleanup.
- **"Backpressure means block."** Blocking can deadlock; reject, shed, or defer
  may be the correct policy.
- **"Autoscaling prevents DoS."** It can amplify cost and lag behind attacks.
- **"OOM is just an error path."** Many deployments cannot reliably recover
  inside the same process.

## Decision Cheat Sheet

| Situation | Do |
|-----------|----|
| Work arrives faster than service rate | Bound queue and define full behavior |
| Expensive per-request operation | Authenticate/admit first; apply CPU/time quota |
| Async code calls blocking API | Move to bounded blocking pool |
| Retries proposed | Require idempotency/deduplication and one total retry budget |
| Shared lock contention | shorten, partition, order, or redesign ownership |
| Timeout exists | verify cancellation reaches work and resources are reclaimed |
| Multi-tenant service | enforce per-tenant and global fairness/capacity budgets |

## Primary Sources

- Rust std synchronization documentation: https://doc.rust-lang.org/std/sync/
- Rust Async Book: https://rust-lang.github.io/async-book/
- Tokio, Bridges with synchronous code:
  https://tokio.rs/tokio/topics/bridging
- CWE-400 Uncontrolled Resource Consumption:
  https://cwe.mitre.org/data/definitions/400.html
- NIST SP 800-160 Vol. 1, Systems Security Engineering:
  https://csrc.nist.gov/publications/detail/sp/800-160/vol-1/rev-1/final

## Related Guides

- Previous: [07-PARSING-DESERIALIZATION-INPUT-VALIDATION-AND-PROTOCOL-ABUSE.md](07-PARSING-DESERIALIZATION-INPUT-VALIDATION-AND-PROTOCOL-ABUSE.md)
- Next: [09-FFI-NATIVE-LIBRARIES-KERNELS-AND-SANDBOX-BOUNDARIES.md](09-FFI-NATIVE-LIBRARIES-KERNELS-AND-SANDBOX-BOUNDARIES.md)
- Concurrency language mechanics: [../rust-language/15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md](../rust-language/15-THREADS-SEND-SYNC-LOCKS-AND-ATOMICS.md)
