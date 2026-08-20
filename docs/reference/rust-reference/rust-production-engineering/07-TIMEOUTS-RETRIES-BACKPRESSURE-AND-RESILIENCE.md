---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-production-engineering:timeouts-retries-backpressure-resilience
kind: guide
module: rust-production-engineering
section: rust-production-engineering
title: Timeouts, Retries, Backpressure, and Resilience
status: source-custody
source_custody: partial
current_path: rust-production-engineering/07-TIMEOUTS-RETRIES-BACKPRESSURE-AND-RESILIENCE.md
canonical_path: rust-production-engineering/07-TIMEOUTS-RETRIES-BACKPRESSURE-AND-RESILIENCE.md
backsource_ids: [proof-backfill:rust-production-engineering:07-timeouts-retries-backpressure-resilience]
concepts: [timeouts, deadlines, retries, backpressure, circuit breakers, bulkheads, idempotency, resilience]
root_concepts: [resilience]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Timeouts, Retries, Backpressure, and Resilience

## The Big Picture

Resilience prevents one slow or failing component from consuming the capacity of
its callers. The core mechanisms are budgets, bounded attempts, admission
control, isolation, and safe duplicate handling. Adding retries without those
constraints usually amplifies an outage.

```
+============================================================================+
|                         RESILIENCE CONTROL LOOP                            |
|                                                                            |
| request deadline                                                           |
|       |                                                                    |
|       v                                                                    |
| admission --> attempt --> classify outcome --> success                     |
|    |           |             |                                             |
|    | reject    | timeout     +--> permanent failure --> stop               |
|    |           |             +--> transient + safe --> backoff/jitter --+  |
|    v           v                                                      |    |
| backpressure  cancel/abandon <------------------------------------------+  |
|                                                                            |
| bulkheads isolate capacity; breakers suppress predictably futile attempts  |
+============================================================================+
```

The deadline belongs to the whole operation. Per-attempt timeouts are
sub-budgets, not fresh clocks.

## Deadlines Before Timeouts

| Concept | Meaning |
|---|---|
| Deadline | absolute end of caller value |
| Timeout | maximum duration for one local wait/attempt |
| Budget | remaining time and attempts available |

Use a monotonic clock for elapsed-time decisions. Propagate remaining budget
across internal calls, but validate untrusted inbound deadline headers to avoid
negative or absurd values.

```
total budget: 1,000 ms
  admission/queue: 100 ms
  attempt 1:       300 ms
  backoff:          75 ms
  attempt 2:       300 ms
  response margin: 225 ms
```

## Retry Preconditions

Retry only when all are true:

1. The failure is plausibly transient.
2. The operation is idempotent, deduplicated, or known not to have started.
3. Remaining time and attempt budgets are sufficient.
4. Additional load will not worsen the failure.
5. The retry occurs at one deliberate layer.

Network timeout means "outcome unknown," not "operation did not happen."
Idempotency keys or transactional deduplication convert that uncertainty into a
replay-safe protocol.

| Idempotency element | Contract |
|---|---|
| Key scope | one caller and one named operation |
| Request binding | reuse with a different payload is rejected |
| Effect | concurrent duplicates produce one semantic mutation |
| Result | completed outcome can be returned consistently |
| Retention | key/result lives at least through the retry and replay horizon |

## Executable Bounded Retry

```rust
use std::{
    num::NonZeroU32,
    thread,
    time::{Duration, Instant},
};

#[derive(Debug)]
enum AttemptError {
    Transient,
    Permanent,
    BudgetExhausted,
}

fn retry<T>(
    deadline: Instant,
    max_attempts: NonZeroU32,
    mut operation: impl FnMut(u32, Duration) -> Result<T, AttemptError>,
) -> Result<T, AttemptError> {
    for attempt in 1..=max_attempts.get() {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err(AttemptError::BudgetExhausted);
        }

        match operation(attempt, remaining) {
            Ok(value) => return Ok(value),
            Err(AttemptError::Permanent) => return Err(AttemptError::Permanent),
            Err(AttemptError::BudgetExhausted) => return Err(AttemptError::BudgetExhausted),
            Err(AttemptError::Transient) if attempt < max_attempts.get() => {
                let delay = Duration::from_millis(20 * u64::from(attempt));
                let remaining = deadline.saturating_duration_since(Instant::now());
                if remaining <= delay {
                    return Err(AttemptError::BudgetExhausted);
                }
                thread::sleep(delay);
            }
            Err(error) => return Err(error),
        }
    }
    unreachable!("max_attempts must be non-zero")
}

fn main() {
    let deadline = Instant::now() + Duration::from_secs(1);
    let max_attempts = NonZeroU32::new(3).expect("constant is non-zero");
    let result = retry(deadline, max_attempts, |attempt, remaining| {
        println!("attempt={attempt} remaining_ms={}", remaining.as_millis());
        if std::env::var_os("PERMANENT_FAILURE").is_some() {
            Err(AttemptError::Permanent)
        } else if attempt < 3 {
            Err(AttemptError::Transient)
        } else {
            Ok("completed")
        }
    });
    println!("{result:?}");
}
```

Run with `rustc retry.rs && ./retry` (PowerShell:
`rustc retry.rs; .\retry.exe`). The callback receives the remaining budget but
must enforce it on the actual I/O operation; this synchronous wrapper cannot
preempt a blocking call. Production code should use randomized jitter across
clients, record each attempt, and couple timeout/cancellation to the operation.
The deterministic delay here keeps the example reproducible. Set
`PERMANENT_FAILURE=1` (or the PowerShell equivalent) to exercise immediate
non-retryable classification.

## Backpressure and Load Shedding

| Mechanism | Protects | Caller experience |
|---|---|---|
| Bounded queue | memory and wait time | wait or rejection |
| Semaphore | scarce in-flight resource | wait with deadline or rejection |
| Rate limit | service/dependency throughput | explicit throttling |
| Load shedding | whole service under overload | fast failure |
| Priority isolation | critical traffic | lower classes delayed/rejected |

Backpressure must reach the producer. If a bounded internal queue is fed by an
unbounded broker prefetch or HTTP buffer, the actual system remains unbounded.

## Breakers, Bulkheads, and Hedging

Circuit breakers suppress attempts while a dependency is predictably failing;
they need careful half-open probing and per-destination scope. Bulkheads reserve
separate capacity so one dependency or tenant class cannot consume all workers.
Hedged requests reduce tail latency by issuing a duplicate after a delay, but
they increase load and require safe cancellation/deduplication.

| Pattern | Use when | Avoid when |
|---|---|---|
| Breaker | failures are correlated and attempts are expensive | natural deadlines and shedding already suffice |
| Bulkhead | workloads have distinct failure/capacity domains | fragmentation would waste scarce capacity |
| Hedge | read-like operation has rare long tails and spare capacity | writes, overload, or expensive requests |

## Library, Runtime, and Platform Choices

| Layer | Choices and boundary |
|---|---|
| Library | retry policy, middleware, semaphore, rate limiter, idempotency store |
| Runtime | timeout timer, cancellation, task selection, sleep |
| Platform | proxy timeouts, queue redelivery, autoscaling, load balancer retries |

Inventory resilience at every layer. A client library, service mesh, ingress,
and application each retrying three times can produce dozens of attempts.
Kubernetes restarts are process recovery, not request retries. Cloud SDK defaults
must be reconciled with the application's end-to-end budget.

## Old World -> New World Bridge

The universal bridge is from **error handling** to **load-control theory**.
Retries change offered load; timeouts consume capacity until cancellation takes
effect; queues exchange rejection for latency. These are control decisions, not
exception-handling conveniences.

Polly in .NET, Envoy policies, and cloud SDK retry modes are familiar
implementations. Rust middleware stacks can express the same patterns, but
ownership of the policy must remain explicit.

## Decision Cheat Sheet

| Use | When |
|---|---|
| End-to-end deadline | caller value expires after a known time |
| Per-attempt timeout | one dependency wait needs a sub-budget |
| Retry | transient failure and duplicate safety are established |
| Exponential backoff + jitter | many clients could synchronize retries |
| Bounded queue | short buffering improves throughput without violating value |
| Immediate rejection | waiting would exceed budget or deepen overload |
| Bulkhead | failure/capacity domains require isolation |
| Circuit breaker | repeated attempts are predictably futile and costly |
| Hedging | idempotent read tails dominate and spare capacity is proven |

## Common Confusion Points

- **Timeout does not cancel by itself.** Ensure the underlying operation stops
  or account for orphaned work.
- **Exponential backoff without jitter still synchronizes clients.**
- **A breaker can hide recovery or partition capacity.** Measure and scope it.
- **Retries at multiple layers multiply.** Choose one owner per operation.
- **Queueing is not resilience when requests have already lost value.**

## Primary Sources

- AWS Builders' Library on timeouts and retries: https://aws.amazon.com/builders-library/timeouts-retries-and-backoff-with-jitter/
- Google SRE overload handling: https://sre.google/sre-book/handling-overload/
- IETF RateLimit fields: https://www.rfc-editor.org/rfc/rfc9333
- Tokio timeouts: https://docs.rs/tokio/latest/tokio/time/fn.timeout.html
- Tower resilience middleware: https://docs.rs/tower/

## Related Guides

- Previous: [06-ASYNC-RUNTIME-CONCURRENCY-AND-CAPACITY.md](06-ASYNC-RUNTIME-CONCURRENCY-AND-CAPACITY.md)
- Next: [08-PERSISTENCE-TRANSACTIONS-AND-DATA-ACCESS.md](08-PERSISTENCE-TRANSACTIONS-AND-DATA-ACCESS.md)
- Error taxonomy: [04-ERRORS-PANICS-CRASHES-AND-DIAGNOSTICS.md](04-ERRORS-PANICS-CRASHES-AND-DIAGNOSTICS.md)
- Recovery testing: [12-TESTING-STAGING-FAULT-INJECTION-AND-RECOVERY.md](12-TESTING-STAGING-FAULT-INJECTION-AND-RECOVERY.md)
