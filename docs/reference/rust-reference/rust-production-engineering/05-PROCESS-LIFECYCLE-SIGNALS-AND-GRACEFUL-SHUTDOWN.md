---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-production-engineering:process-lifecycle-signals-graceful-shutdown
kind: guide
module: rust-production-engineering
section: rust-production-engineering
title: Process Lifecycle, Signals, and Graceful Shutdown
status: source-custody
source_custody: partial
current_path: rust-production-engineering/05-PROCESS-LIFECYCLE-SIGNALS-AND-GRACEFUL-SHUTDOWN.md
canonical_path: rust-production-engineering/05-PROCESS-LIFECYCLE-SIGNALS-AND-GRACEFUL-SHUTDOWN.md
backsource_ids: [proof-backfill:rust-production-engineering:05-process-lifecycle-signals-graceful-shutdown]
concepts: [process lifecycle, signals, graceful shutdown, draining, readiness, cancellation, supervision]
root_concepts: [process lifecycle]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Process Lifecycle, Signals, and Graceful Shutdown

## The Big Picture

A service process is a state machine supervised by something outside itself.
Startup, readiness, draining, and termination must align with the supervisor's
deadlines. Graceful shutdown means "bounded preservation of declared
invariants," not "wait forever for every task."

```
+============================================================================+
|                         PROCESS LIFECYCLE                                  |
|                                                                            |
|  created --> initialize --> ready --> serving                              |
|                  |            |          |                                 |
|                  | fail       | unready  | stop request                    |
|                  v            v          v                                 |
|              exit nonzero   isolate    draining                            |
|                                         |                                  |
|                              stop admission                                |
|                              cancel optional work                          |
|                              finish/abort in-flight work                   |
|                              flush bounded evidence                        |
|                                         | deadline                         |
|                                         v                                  |
|                                      stopped                               |
+============================================================================+
```

The process should become unready before it stops accepting work, allowing
routers or dispatchers to converge. The exact ordering varies by protocol: a
queue consumer may pause delivery first; an HTTP server may close listeners and
finish established requests.

## Startup Contract

| Phase | Allowed work | Failure behavior |
|---|---|---|
| Parse/validate | local config and arguments | diagnostic + non-zero exit |
| Acquire identity | credential/token bootstrap | bounded retry or fail |
| Verify dependencies | only required startup capabilities | fail or enter explicit degraded mode |
| Initialize state | pools, caches, migrations by policy | never silently partial |
| Bind/admit | listeners or queue leases | readiness only after success |

Avoid hiding long, unbounded dependency retry loops before readiness. A
supervisor cannot distinguish "patiently starting" from "wedged" unless startup
state and deadlines are visible.

## Shutdown Phases

```
T0 signal/control received
 |
 +--> mark unready
 +--> stop new admission
 +--> cancel background refresh/poll work
 +--> drain requests up to DRAIN_DEADLINE
 +--> commit/abort state according to operation semantics
 +--> flush telemetry up to FLUSH_DEADLINE
 |
T1 clean exit, otherwise forced termination policy
```

Use one monotonic shutdown deadline, then allocate sub-budgets; independent full
timeouts can accidentally exceed the platform grace period.

## Executable Unix Signal Example

```toml
# Cargo.toml
[package]
name = "shutdown-example"
version = "0.1.0"
edition = "2021"

[target.'cfg(unix)'.dependencies]
signal-hook = "0.3"
```

```rust
#[cfg(unix)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use signal_hook::{
        consts::{SIGINT, SIGTERM},
        iterator::Signals,
    };
    use std::{
        sync::{
            atomic::{AtomicBool, Ordering},
            mpsc::{self, RecvTimeoutError},
            Arc,
        },
        thread,
        time::{Duration, Instant},
    };

    let stopping = Arc::new(AtomicBool::new(false));
    let worker_stop = Arc::clone(&stopping);
    let (done_tx, done_rx) = mpsc::sync_channel(1);
    let worker = thread::spawn(move || {
        while !worker_stop.load(Ordering::Acquire) {
            thread::sleep(Duration::from_millis(100));
        }
        // Finish only bounded, invariant-preserving work here.
        let _ = done_tx.send(());
    });

    let mut signals = Signals::new([SIGINT, SIGTERM])?;
    let signal = signals.forever().next().expect("signal iterator ended");
    eprintln!("shutdown requested by signal {signal}");
    let deadline = Instant::now() + Duration::from_secs(10);
    stopping.store(true, Ordering::Release);

    let remaining = deadline.saturating_duration_since(Instant::now());
    match done_rx.recv_timeout(remaining) {
        Ok(()) => worker.join().map_err(|_| "worker panicked")?,
        Err(RecvTimeoutError::Timeout) => {
            // Dropping the handle detaches the worker; returning from `main`
            // lets the supervisor observe failure and enforce termination.
            drop(worker);
            return Err("shutdown deadline exceeded".into());
        }
        Err(RecvTimeoutError::Disconnected) => {
            worker.join().map_err(|_| "worker panicked")?;
            return Err("worker exited without completion signal".into());
        }
    }
    Ok(())
}

#[cfg(not(unix))]
fn main() {
    eprintln!("This example is scoped to Unix SIGINT/SIGTERM.");
}
```

Run on a Unix-like host with `cargo run`, then send Ctrl-C or
`kill -TERM <pid>`. This example demonstrates signal-to-coordination handoff;
real services also stop admission and track in-flight work. The completion
channel makes the wait bounded; a hung worker is not joined after the deadline.
Windows services receive Service Control Manager control events rather than
Unix signals, so use a Windows-service integration crate or platform host for
that lifecycle.

## Cancellation Is a Protocol

Dropping a Rust future requests cancellation by ceasing to poll it; whether that
is safe depends on the future's internal state. A database transaction object
may roll back on drop, while a remote request may already have committed.
Cancellation safety must be documented at each `.await` or blocking boundary.

| Work class | Shutdown policy |
|---|---|
| Idempotent read | cancel or finish within budget |
| Transactional local update | commit atomically or roll back |
| Non-idempotent remote call | track idempotency key and reconcile outcome |
| Telemetry export | best effort with a short explicit budget |
| Lease/lock ownership | release when possible; rely on expiry/fencing for crashes |

## Library, Runtime, and Platform Choices

| Layer | Choices and boundary |
|---|---|
| Library | atomics/channels, cancellation token, signal adapter, server drain API |
| Runtime | task cancellation, join sets, timer/deadline implementation |
| Platform | SIGTERM, Windows service controls, grace period, restart policy |

Tokio offers `tokio::signal` and runtime-specific cancellation patterns.
Synchronous applications can use OS threads and signal crates. Kubernetes,
systemd, and Windows SCM each impose different notification and timeout
mechanisms; design to the lifecycle contract, then adapt it.

## Old World -> New World Bridge

The universal bridge is from **destructor cleanup** to **distributed quiescence**.
RAII releases in-process resources when scopes end, but graceful shutdown first
has to stop new distributed work and settle operations whose effects cross the
process boundary.

Windows Service `Stop`, systemd `SIGTERM`, and a Kubernetes pod termination are
three encodings of a supervisor request. None guarantees sufficient time, so
the process must remain correct under abrupt termination as well.

## Decision Cheat Sheet

| Use | When |
|---|---|
| Clean startup failure | required invariant cannot be established |
| Degraded readiness | process is healthy but cannot safely accept some/all work |
| Shared cancellation token | many cooperative tasks need one stop signal |
| Bounded drain | in-flight work can preserve user-visible outcomes |
| Immediate abort | continuing risks corruption or the supervisor deadline is exhausted |
| Lease expiry/fencing | crash-safe ownership cannot depend on graceful cleanup |
| Runtime signal API | application already commits to that runtime |
| Platform service adapter | supervisor uses controls other than ordinary console signals |

## Common Confusion Points

- **Signal handlers cannot do arbitrary work safely.** Use a crate/runtime
  adapter that transfers notification into normal execution context.
- **Readiness false does not instantly stop traffic.** Allow for propagation and
  existing connections.
- **Dropping a future does not undo remote side effects.** Reconciliation may be
  required.
- **`Drop` is not guaranteed on process termination.** Correctness must survive
  abort, OOM, and host loss.
- **Multiple stop signals should be idempotent.** A second signal often means
  "stop waiting and terminate now."

## Primary Sources

- Rust `Drop`: https://doc.rust-lang.org/std/ops/trait.Drop.html
- `signal-hook`: https://docs.rs/signal-hook/
- Tokio graceful shutdown topic: https://tokio.rs/tokio/topics/shutdown
- Kubernetes pod termination: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#pod-termination
- systemd service behavior: https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html
- Windows service control handler: https://learn.microsoft.com/windows/win32/services/service-control-handler-function

## Related Guides

- Previous: [04-ERRORS-PANICS-CRASHES-AND-DIAGNOSTICS.md](04-ERRORS-PANICS-CRASHES-AND-DIAGNOSTICS.md)
- Next: [06-ASYNC-RUNTIME-CONCURRENCY-AND-CAPACITY.md](06-ASYNC-RUNTIME-CONCURRENCY-AND-CAPACITY.md)
- Platform adaptation: [10-CONTAINERS-KUBERNETES-SYSTEMD-AND-WINDOWS-SERVICES.md](10-CONTAINERS-KUBERNETES-SYSTEMD-AND-WINDOWS-SERVICES.md)
- Readiness gate: [15-OPERATIONAL-READINESS-AND-RELEASE-GATES.md](15-OPERATIONAL-READINESS-AND-RELEASE-GATES.md)
