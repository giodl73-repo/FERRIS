# Rust Async Portability

Date: 2026-08-09
Status: Complete
Question: ECOS-Q04
Decision: represent async portability as an operation-level runtime contract.
`Future` is executor-neutral, but spawning, timers, I/O traits, task ownership,
cancellation, blocking work, context, clocks, and shutdown can each introduce
independent runtime requirements.

## Decision supported

ECOS-Q04 determines which async assumptions may cross a library boundary
directly, which require an explicit runtime parameter or adapter, and which
must remain a documented environment precondition.

It does not select Tokio as a universal runtime, claim that every Tokio type
requires a Tokio executor, or treat dropping any future as semantically safe
cancellation.

## Runtime contract

Every async boundary must classify these dimensions independently:

| Dimension | Questions |
|---|---|
| Future | Is the returned future only poll/waker based, or does polling require runtime context? |
| Executor | Who polls the future, and is an executor handle captured or required? |
| Spawn | Which spawn function, `Send`/`'static` requirements, task ownership, result, panic, and shutdown rules apply? |
| I/O | Which `AsyncRead`/`AsyncWrite` identity, buffer model, readiness source, and adapter are required? |
| Time | Which clock, timer driver, resolution, pause/advance behavior, timeout, and interval semantics apply? |
| Cancellation | What happens when the future, task handle, or losing `select` branch is dropped or aborted? |
| Blocking | Where does blocking work run, how is it bounded, and can it be stopped during shutdown? |
| Synchronization | Which channel/lock/semaphore contract, capacity, fairness, closure, and wake behavior applies? |
| Context | How are task-local, thread-local, tracing, deadline, and request values scoped and propagated? |
| Platform | Does the contract require `std`, allocation, threads, sockets, OS readiness, WASM integration, or embedded interrupts? |

Runtime-compatible means every required dimension is observed. A portable
future type alone is insufficient.

## Measured probes

Ten exact-version fixtures were run with Rust and Cargo 1.95.0. Sources,
checksums, commands, and diagnostics are recorded in
[EXP-01](ecos-q04-async-portability/results/EXP-01-async-portability-probes.md).

| Probe | Result | Contract demonstrated |
|---|---|---|
| pure async function driven by futures-executor and Tokio | Pass | `Future` can be executor-neutral |
| Tokio timer polled by futures-executor without Tokio context | Expected panic | timer driver is an environmental precondition |
| `tokio::spawn` outside Tokio context | Expected panic | spawning is runtime-bound |
| Tokio duplex stream passed as futures-io `AsyncRead` | Expected E0277 | I/O traits are distinct identities |
| Tokio stream wrapped with tokio-util `compat()` | Pass | an explicit adapter can bridge the trait boundary |
| Tokio `JoinHandle` dropped before task completion | Pass; task still completed | dropping the handle detaches rather than cancels |
| Tokio task explicitly aborted | Pass; joined error was cancelled | abort is distinct from handle drop |
| non-`Send` future passed to `tokio::spawn` | Expected compile failure | multithread-capable spawn exposes `Send + 'static` |
| same non-`Send` work on Tokio `LocalSet` | Pass | local spawning is a different executor contract |
| Tokio task-local scope and oneshot channel polled by futures-executor | Pass | package namespace alone does not prove runtime coupling |

## API dispositions

| Surface | Disposition |
|---|---|
| `core::future::Future`, `Poll`, `Context`, `Waker` | Runtime-neutral language contract |
| `futures-core` stream/future traits | Runtime-neutral contract subject to the future's implementation |
| `tower-service::Service` | Executor-neutral trait with separate readiness and call-protocol semantics |
| `tokio::spawn`, Tokio runtime handles, Tokio timers and network drivers | Tokio environment contract |
| `tokio::io` and `futures-io` | Separate I/O contract families |
| `tokio-util::compat` | Explicit I/O adapter; exact trait bridge verified, behavioral profile still required |
| Tokio task-local scope and sync primitives | Evaluate API by API; selected primitives can be polled by another executor |
| Tokio `JoinHandle` | Tokio task ownership contract; drop detaches, abort is explicit |
| `spawn_blocking` | Separate blocking-work lifecycle; started work is not abortable |
| hyper `rt` traits | Positive example of executor, timer, and I/O capability injection |
| reqwest async client | Documentation explicitly requires Tokio |
| tonic transport | Default transport is built on Tokio, hyper, and tower |
| SQLx | Runtime chosen by features and current context; async use can panic with no runtime feature |
| Embassy executor | Separate embedded platform/spawn contract built on core futures |

## Findings

### FERRIUM-545: `Future` is an inert protocol, not a runtime

**Sources:** `std::future::Future` documentation and the dual-executor passing
fixture.

**Observed behavior:** The same pure async function completed under
futures-executor and a Tokio current-thread runtime. The standard Future
contract defines polling and wakeup; futures make progress only when an
executor polls them.

**Implication:** Public APIs can return futures without selecting a runtime only
when their implementation avoids hidden runtime-specific operations.

**Confidence:** High.

### FERRIUM-546: runtime coupling is an operation property, not a crate-name property

**Sources:** Tokio timer/spawn failures and the passing task-local/oneshot
fixture on futures-executor.

**Observed behavior:** `tokio::spawn` and `tokio::time::sleep` failed outside a
Tokio runtime, while a Tokio task-local scope and oneshot receiver completed
when polled by futures-executor.

**Implication:** FERRIUM must classify individual APIs and enabled features;
marking an entire package runtime-bound or runtime-neutral is too coarse.

**Confidence:** High.

### FERRIUM-547: missing runtime context can be a runtime panic

**Sources:** Tokio spawn and sleep documentation plus expected-panic fixtures.

**Observed behavior:** Both fixtures compiled. At execution they panicked with
"there is no reactor running" because no Tokio runtime/timer context existed.

**Implication:** Compile success cannot establish async portability. Negative
runtime-context probes are mandatory for runtime-sensitive APIs.

**Confidence:** High.

### FERRIUM-548: Rust has multiple nominal async I/O contracts

**Sources:** tokio::io, futures-io, tokio-util compat documentation, and I/O
fixtures.

**Observed behavior:** Tokio's `DuplexStream` implemented
`tokio::io::AsyncRead` but not `futures_io::AsyncRead`; rustc emitted E0277.
Wrapping the stream with tokio-util's `compat()` satisfied the futures-io bound.

**Implication:** Async I/O identity, direction, wrapper, buffer model, and
additional trait preservation must be explicit at every boundary.

**Confidence:** High.

### FERRIUM-549: an I/O adapter proves trait availability, not full behavioral equivalence

**Sources:** passing compat fixture and tokio-util documentation.

**Observed behavior:** The compile probe established the target trait
implementation. It did not measure partial reads, vectored I/O, buffering,
readiness, cancellation, errors, seeking, buffered reads, or performance.

**Implication:** Adapter evidence must separate compile compatibility from
behavioral and performance profiles.

**Confidence:** High.

### FERRIUM-550: spawn signatures expose scheduling and ownership policy

**Sources:** Tokio spawn documentation and Send/local-spawn fixtures.

**Observed behavior:** `tokio::spawn` required a `Future + Send + 'static`; a
future retaining `Rc` across an await failed compilation. The same work
completed through `LocalSet::spawn_local`.

**Implication:** Spawn contracts must record `Send`, lifetime, thread mobility,
local-executor, output, panic, and runtime-context requirements.

**Confidence:** High.

### FERRIUM-551: task-handle drop and task cancellation are distinct

**Sources:** Tokio JoinHandle documentation and drop/abort runtime fixtures.

**Observed behavior:** Dropping a JoinHandle detached the task and its oneshot
result still arrived. Calling `abort()` produced a cancelled JoinError when the
handle was awaited.

**Implication:** Libraries must not use handle drop as an implicit cancellation
protocol. Task ownership and shutdown need explicit handles, groups, tokens, or
caller-owned orchestration.

**Confidence:** High.

### FERRIUM-552: cancellation safety belongs to each awaited operation

**Sources:** Tokio `select!` documentation.

**Observed behavior:** `select!` drops non-winning branch futures. Tokio defines
cancellation safety in terms of whether dropping and recreating a pending
future preserves correct progress, and documents safe and unsafe operations.

**Implication:** "Cancelable" is not one future trait. Profiles must record
drop behavior, partial progress, queue position, message/data loss, cleanup,
idempotence, and restart safety.

**Confidence:** High.

### FERRIUM-553: blocking work has a different cancellation domain

**Sources:** Tokio `spawn_blocking` documentation.

**Observed behavior:** Started blocking tasks cannot be aborted. Runtime
shutdown can wait indefinitely for them; `shutdown_timeout` stops waiting but
does not cancel the work.

**Implication:** Blocking operations require bounded duration, concurrency,
cooperative cancellation, process/thread ownership, and shutdown evidence
separate from async task cancellation.

**Confidence:** High.

### FERRIUM-554: timers expose a clock and driver contract

**Sources:** Tokio sleep documentation and timer-context fixture.

**Observed behavior:** Tokio sleep requires a current timer driver, has
platform-specific resolution, and is cancelled by dropping its Sleep future.
Construction/polling location relative to runtime entry changes whether it
panics.

**Implication:** Timeout and scheduling APIs must record clock identity,
runtime driver, resolution, test-clock behavior, construction context, and drop
semantics.

**Confidence:** High.

### FERRIUM-555: context propagation requires explicit scope

**Sources:** Tokio LocalKey documentation and cross-executor task-local fixture.

**Observed behavior:** A scoped Tokio task-local value remained available when
its scope future was polled by futures-executor. Access outside a set scope can
panic or return AccessError.

**Implication:** Context portability depends on scope construction and polling,
not merely the executor brand. APIs should prefer explicit values where
context absence is unacceptable.

**Confidence:** High.

### FERRIUM-556: hyper demonstrates capability injection

**Sources:** hyper 1.11 runtime module and hyper-util TokioIo documentation.

**Observed behavior:** Hyper defines executor, timer, and I/O traits to remain
runtime-agnostic; hyper-util supplies Tokio-specific adapters and wrappers.

**Implication:** FERRIUM should adopt the pattern—small capability contracts
plus replaceable runtime adapters—without copying hyper-specific types into a
universal API.

**Confidence:** High.

### FERRIUM-557: application libraries expose runtime choice through different mechanisms

**Sources:** reqwest 0.13.4, tonic 0.14.6, and SQLx 0.9.0 documentation.

**Observed behavior:** Reqwest states its async client requires Tokio. Tonic's
default transport is based on Tokio, hyper, and tower. SQLx exposes Tokio and
async-std runtime features and chooses Tokio when both are enabled and a Tokio
context is current; nearly all async APIs can panic if no runtime feature is
enabled.

**Implication:** Runtime dependency can be a hard requirement, a feature-
selected provider, a context-detected choice, or a separable transport layer.
The ledger must distinguish them.

**Confidence:** High.

### FERRIUM-558: embedded executors share Future but not desktop runtime services

**Sources:** embassy-executor 0.10.0 platform documentation.

**Observed behavior:** Embassy supports distinct std, Cortex, RISC-V, WASM,
AVR, and spin platform implementations with target-specific privilege,
interrupt, sleep, and core constraints.

**Implication:** Core Future compatibility does not provide Tokio spawn, timer,
network, allocation, thread, or OS contracts. Embedded is a separate runtime
profile, not a reduced desktop profile.

**Confidence:** High.

### FERRIUM-559: task lifecycle must be a first-class compatibility record

**Sources:** spawn, JoinHandle, select, spawn_blocking, and runtime-shutdown
documentation.

**Observed behavior:** Tasks may detach, abort at suspension points, be dropped
on runtime shutdown, lose branch progress, or continue as blocking work after
shutdown waiting ends.

**Implication:** OSPREY needs task parent, owner, handle, result, panic,
cancellation, deadline, cleanup, blocking, and shutdown-outcome evidence.

**Confidence:** High.

### FERRIUM-560: runtime portability requires negative controls

**Sources:** all ten fixtures.

**Observed behavior:** Several boundaries compiled successfully but failed only
when executed without context. Others failed at nominal trait or Send bounds.

**Implication:** Compatibility profiles must include expected compile failures,
expected context panics, cancellation outcomes, and shutdown cases—not only
happy-path execution.

**Confidence:** High.

## Decision

### Adopt now

- Adopt the operation-level runtime contract.
- Keep Future, executor, spawn, I/O, time, cancellation, blocking,
  synchronization, context, and platform as separate dimensions.
- Prefer runtime-neutral futures at public library boundaries.
- Accept runtime-bound APIs only with an explicit environment contract.
- Treat task ownership and cancellation as designed protocols.
- Record exact I/O trait family and adapter direction.
- Use capability injection patterns for executors, timers, and transports where
  multiple runtimes are a real consumer requirement.

Owner: FERRIUM.

Expected validation: ECOS-Q07 target and no_std builds, ECOS-Q08 feature and
duplicate-runtime cost, ECOS-Q09 native I/O/provider effects, and ECOS-Q11
server, CLI, WASM, embedded, and native-integration profiles.

Non-goals: selecting one runtime, building a FERRIUM executor, replacing Tokio,
promising universal portability, or publishing adapters.

### Prototype behind a compatibility boundary

- I/O adapter behavioral and performance tests.
- Executor/timer capability-injection fixtures.
- Task-group ownership, explicit cancellation, and shutdown probes.
- Runtime-context negative tests.
- Tokio versus alternate-runtime application profiles only where a real
  consumer requires both.
- Embedded Future-only boundaries separated from desktop runtime services.

### Reject or defer

- hidden `tokio::spawn` or timer assumptions in runtime-neutral library APIs;
- exposing runtime-specific JoinHandle or timer types without declaring the
  environment contract;
- assuming handle drop cancels a task;
- assuming all futures are cancellation-safe;
- assuming every Tokio primitive requires Tokio execution;
- assuming I/O adapters preserve all additional traits and behavior;
- unbounded or indefinitely running `spawn_blocking` work; and
- a universal async abstraction layer before representative profiles prove the
  need.

## Role review

### Rust Safety Steward

Accepts explicit task ownership, cancellation, and blocking boundaries.
Requires cleanup, partial-progress, panic, unsafe I/O, and shutdown invariants
to remain visible.

### Compiler Performance Engineer

Accepts the contract split. Requires ECOS-Q08 to measure duplicate runtimes,
macro/features, adapter code, compile time, binary size, and executor overhead
before consolidation claims.

### Interop Boundary Auditor

Accepts negative context and trait-identity probes. Requires each adapter to
record direction, buffer model, traits preserved, cancellation, errors, and
runtime prerequisites.

### AI Assurance Skeptic

Accepts exact expected failures and source-reviewed unknowns. Rejects generated
runtime adapters or cancellation claims without negative behavioral tests.

### Ecosystem Strategist

Accepts capability injection and upstream adapter patterns. Rejects a FERRIUM
runtime or universal facade absent demonstrated consumer demand.

### Rust Maintainer

Accepts runtime-neutral defaults and explicit runtime-bound modules. Requires
ordinary Cargo features, understandable errors, and removable adapters.

### Native Platform Adopter

Accepts timer, driver, blocking, shutdown, and embedded distinctions. Requires
measured Windows, Linux, macOS, container, WASM, and embedded behavior in later
profiles.

### Scope Keeper

Accepts Q04 as runtime-contract research only. Runtime implementation,
security, stewardship, platform breadth, fragmentation cost, and stack
selection remain closed.

### Validation Checker

Accepts ten fixtures, exact releases, checksums, and documented controls.
Requires future profiles to retain context-panic, compile-fail, abort, detach,
and cancellation-safety cases.

## Limitations

- One host and toolchain were measured.
- I/O compatibility was compile-tested, not throughput-tested.
- No real sockets, files, DNS, TLS, process, or signal APIs were exercised.
- No alternate desktop runtime was installed or benchmarked.
- SQLx, reqwest, tonic, hyper, and Embassy behavior was source-reviewed only.
- `select!` cancellation-safe and unsafe operations were not exhaustively
  executed.
- `spawn_blocking` shutdown behavior was not executed to avoid leaving work
  beyond the fixture.
- No WASM, embedded, or `no_std` target was compiled.
