---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-production-engineering:errors-panics-crashes-diagnostics
kind: guide
module: rust-production-engineering
section: rust-production-engineering
title: Errors, Panics, Crashes, and Diagnostics
status: source-custody
source_custody: partial
current_path: rust-production-engineering/04-ERRORS-PANICS-CRASHES-AND-DIAGNOSTICS.md
canonical_path: rust-production-engineering/04-ERRORS-PANICS-CRASHES-AND-DIAGNOSTICS.md
backsource_ids: [proof-backfill:rust-production-engineering:04-errors-panics-crashes-diagnostics]
concepts: [errors, panic, crash, diagnostics, backtrace, exit codes, core dumps, error taxonomy]
root_concepts: [failure handling]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Errors, Panics, Crashes, and Diagnostics

## The Big Picture

Production failures need classification before handling. Expected operational
failures belong in typed `Result` paths. Broken internal invariants may justify
a panic. Allocation failure, aborts, access violations in unsafe/native code,
and forced termination may bypass ordinary Rust cleanup entirely; production
code must not rely on orderly destruction in those paths.

```
+============================================================================+
|                         FAILURE TAXONOMY                                   |
|                                                                            |
|  expected failure --> Result<T, E> --> classify --> retry/reject/degrade   |
|  broken invariant --> panic ---------> hook -----> unwind or abort         |
|  process failure  --> alloc/signal/SEH --> supervisor + crash artifacts    |
|  host failure     --> machine/node ----> replica/failover/recovery         |
|                                                                            |
|  caller evidence: stable error kind, safe message, protocol status         |
|  operator evidence: chain, backtrace, release, context, crash dump         |
+============================================================================+
```

The caller and operator need different views. A client may receive
`temporarily_unavailable`; an operator needs the dependency, timeout budget,
attempt count, release, trace ID, and error chain - without secrets.

## Error Taxonomy and Context

| Category | Typical action | Examples |
|---|---|---|
| Invalid request | reject; no retry without change | malformed input, failed precondition |
| Resource absent | domain-specific response | missing record, deleted object |
| Contention | bounded retry or conflict response | optimistic version mismatch |
| Dependency transient | retry within budget or degrade | timeout, reset, temporary overload |
| Dependency permanent | fail fast; operator action | authentication failure, schema mismatch |
| Internal defect | stop unsafe operation; alert | impossible state, violated invariant |

Attach context at abstraction boundaries: "reading release manifest" is useful;
repeating "I/O error" five times is not. Preserve the source chain so low-level
evidence remains available.

## Executable Error and Panic Boundary

```rust
use std::{backtrace::Backtrace, error::Error, fmt, process::ExitCode};

#[derive(Debug)]
enum AppError {
    InvalidConfig(String),
    Dependency(std::io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfig(msg) => write!(f, "invalid configuration: {msg}"),
            Self::Dependency(_) => write!(f, "dependency I/O failed"),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Dependency(source) => Some(source),
            _ => None,
        }
    }
}

fn run() -> Result<(), AppError> {
    let path = std::env::var("APP_INPUT")
        .map_err(|_| AppError::InvalidConfig("APP_INPUT is required".into()))?;
    std::fs::read(path).map_err(AppError::Dependency)?;
    Ok(())
}

fn main() -> ExitCode {
    std::panic::set_hook(Box::new(|info| {
        eprintln!("panic: {info}\nbacktrace:\n{}", Backtrace::force_capture());
    }));

    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            let mut source = error.source();
            while let Some(item) = source {
                eprintln!("caused by: {item}");
                source = item.source();
            }
            ExitCode::FAILURE
        }
    }
}
```

Scope: recent stable Rust. Run with `rustc app.rs`; then set `APP_INPUT` to an
existing or missing file. The example does not log the file content and keeps
the public error text separate from the source chain.

The hook is pedagogical. Panic hooks run while another operation may already
hold locks or have violated an invariant; formatting and backtrace capture can
allocate, block, fail, or recurse. A production hook should be minimal,
bounded, avoid application locks, and complement platform crash capture rather
than being the only evidence path.

## Panic Policy

`panic = "unwind"` unwinds Rust frames where unwinding is supported and runs
destructors for those frames. `panic = "abort"` terminates without unwinding and
usually reduces binary/unwind overhead. Neither setting turns a panic into a
recoverable domain error.

```
panic
  |
  +--> hook emits bounded evidence
  |
  +--> unwind: destructors run until caught or process boundary
  |
  +--> abort: immediate process termination
```

Use `catch_unwind` only around a deliberate in-process unwind boundary, such as
a callback API whose state can be discarded and whose invariants can be
re-established. It is not a security boundary for untrusted code; use process
or sandbox isolation for that. It catches unwinding panics, not aborts,
allocator-termination paths, native faults, or asynchronous termination. Do
not let an unwind cross an FFI boundary unless that ABI explicitly permits it.
Mutex poisoning is evidence that a protected invariant may be incomplete;
blindly calling `into_inner()` suppresses that signal.

## Crash Artifacts

| Platform capability | Useful artifact |
|---|---|
| Unix-like host | core dump, journal/stderr, executable, debug symbols, maps |
| Windows | minidump/full dump, Windows Event Log, PDB, executable |
| Container platform | termination reason, previous logs, node events, dump volume |
| Any platform | release ID, source revision, dependency lockfile, config fingerprint |

Separate symbols from stripped release packages only if the mapping is durable
and access-controlled. A dump without the exact binary and symbols may be
nearly useless. Core dumps can contain secrets and personal data; collection,
retention, and access are security decisions.

## Library, Runtime, and Platform Choices

| Layer | Choices and boundary |
|---|---|
| Library | domain enums, `thiserror`, report/context crates, panic hook |
| Runtime | task panic propagation and join behavior; runtime-specific supervision |
| Platform | exit interpretation, restart policy, dump capture, symbol service |

An async runtime may surface a task panic through a join handle while the
process remains alive. Decide whether loss of that task violates a process-level
invariant. Do not assume the runtime will restart critical tasks safely.

## Old World -> New World Bridge

The universal bridge is from **exceptions as control flow** to **typed expected
failure plus explicit process-failure policy**. Rust makes recoverable errors
visible in signatures, but operational taxonomy and diagnostic context still
require design.

.NET exception filters, unhandled-exception handlers, Windows Error Reporting,
and PDB symbolication have close analogues. The difference is not that Rust
cannot crash; it is that many ordinary failure paths are values rather than
stack unwinds.

## Decision Cheat Sheet

| Use | When |
|---|---|
| Domain error enum | callers need stable programmatic distinctions |
| Opaque report with context | application boundary needs rich operator evidence |
| Panic | internal invariant is broken and continuing locally is unsafe |
| `catch_unwind` | a narrow unwind boundary can restore invariants and isolation |
| `panic = "abort"` | process replacement is the policy and unwind is unnecessary |
| Core/minidump | native state or impossible crash needs postmortem analysis |
| Clean non-zero exit | startup/configuration failure is known and reportable |

## Common Confusion Points

- **`unwrap` is not always forbidden.** It is acceptable when the invariant is
  local, obvious, and failure truly indicates a defect; make that invariant
  visible.
- **A panic hook does not make abort cleanup-safe.** It is for evidence, not
  recovery.
- **Backtraces are not automatically useful.** Build settings, symbols, and the
  exact artifact matter.
- **Retryability is not a property of an error string.** It depends on operation
  semantics, attempt budget, and whether progress may already have occurred.
- **Catching every panic can preserve corruption.** Containment requires an
  invariant argument, not optimism.
- **Allocation failure is not a normal `Result` path by default.** Behavior
  depends on allocator and build/runtime choices; assume cleanup may not run.

## Primary Sources

- Rust error handling: https://doc.rust-lang.org/book/ch09-00-error-handling.html
- `std::error::Error`: https://doc.rust-lang.org/std/error/trait.Error.html
- `std::panic`: https://doc.rust-lang.org/std/panic/
- Cargo panic profile setting: https://doc.rust-lang.org/cargo/reference/profiles.html#panic
- Rust backtraces: https://doc.rust-lang.org/std/backtrace/

## Related Guides

- Previous: [03-METRICS-HEALTH-AND-TELEMETRY-DESIGN.md](03-METRICS-HEALTH-AND-TELEMETRY-DESIGN.md)
- Next: [05-PROCESS-LIFECYCLE-SIGNALS-AND-GRACEFUL-SHUTDOWN.md](05-PROCESS-LIFECYCLE-SIGNALS-AND-GRACEFUL-SHUTDOWN.md)
- Retry classification: [07-TIMEOUTS-RETRIES-BACKPRESSURE-AND-RESILIENCE.md](07-TIMEOUTS-RETRIES-BACKPRESSURE-AND-RESILIENCE.md)
- Incident diagnostics: [13-PRODUCTION-DEBUGGING-AND-INCIDENT-RESPONSE.md](13-PRODUCTION-DEBUGGING-AND-INCIDENT-RESPONSE.md)
