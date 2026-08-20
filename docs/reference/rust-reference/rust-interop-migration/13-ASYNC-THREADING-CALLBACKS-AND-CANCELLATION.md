---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-interop-migration:async-threading-callbacks-cancellation
kind: guide
module: rust-interop-migration
section: computing-software
title: Async, Threading, Callbacks, and Cancellation
status: source-custody
source_custody: partial
current_path: rust-interop-migration/13-ASYNC-THREADING-CALLBACKS-AND-CANCELLATION.md
canonical_path: rust-interop-migration/13-ASYNC-THREADING-CALLBACKS-AND-CANCELLATION.md
backsource_ids: [proof-backfill:rust-interop-migration:13-async-threading-callbacks-cancellation]
concepts: [async interop, threading, callbacks, cancellation, reentrancy, executor, event loop, thread affinity]
root_concepts: [async interop]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Async, Threading, Callbacks, and Cancellation

Async abstractions do not compose automatically across runtimes. A Rust future,
.NET `Task`, Java `CompletableFuture`, JavaScript Promise, Python coroutine, and
WinRT async operation each have distinct start, scheduling, completion,
cancellation, and context rules. Bridge an operation state machine, not a type.

## The Big Picture

```
+============================================================================+
|                     CROSS-RUNTIME ASYNC OPERATION                          |
+============================================================================+
|  HOST CALL                                                                 |
|  create operation + retain inputs/context + return host handle/future      |
|      |                                                                     |
|      v                                                                     |
|  RUST EXECUTION                                                            |
|  dedicated thread | worker pool | Rust async runtime | remote service      |
|      |                                                                     |
|      v                                                                     |
|  TERMINAL OUTCOME: success, error, cancellation, or panic/crash            |
|  exactly one terminal transition in operation state                        |
|      |                                                                     |
|      v                                                                     |
|  COMPLETION ADAPTER                                                        |
|  marshal to valid host thread/context; invoke once; release retained state |
|                                                                            |
|  races to define: cancel vs complete, callback vs destroy, shutdown vs run |
+============================================================================+
```

## Define the Operation State Machine

```
  Created -> Running -> Succeeded
                    \-> Failed
                    \-> Cancelled

  cancel request does not imply Cancelled yet
  completion is exactly once
  destroy is legal only after ownership of callback/context is resolved
```

State whether cancellation is:

1. **pre-start** - queued work is removed;
2. **cooperative** - running work checks a token;
3. **transport** - request/stream is closed;
4. **result abandonment** - work continues but completion is ignored.

These have different resource and side-effect semantics. Do not label all four
"cancelled."

## Callback ABI

```rust
use std::ffi::c_void;

pub type RimCompletion =
    Option<unsafe extern "C" fn(ctx: *mut c_void, status: i32, value: u64)>;

#[repr(C)]
pub struct RimCallback {
    pub function: RimCompletion,
    pub context: *mut c_void,
}
```

The API contract must add what the type cannot express:

- whether callback is synchronous or later;
- exactly-once versus at-most-once delivery;
- callback thread and whether it may block;
- whether it may reenter the same handle;
- how long `context` remains valid;
- whether cancellation suppresses completion;
- who releases retained callback state;
- no panic/exception may escape the callback.

If the host requires main-thread/event-loop completion, Rust workers enqueue a
message through the host's supported dispatcher/thread-safe function; they do
not invoke host objects directly.

The raw context pointer is not proof of thread safety or liveness. Moving a
callback record to a worker requires an adapter with a documented safety proof
for the context, or a registry/token design that resolves the host object only
on its valid thread. Do not add an `unsafe impl Send` merely to satisfy a worker
queue.

## Runtime Ownership

| Pattern | Use | Hazard |
|---------|-----|--------|
| Host owns threads; Rust call is synchronous | Short bounded native work | Blocking host thread |
| Rust owns dedicated worker pool | CPU/native jobs from many hosts | Shutdown, oversubscription |
| Rust owns async runtime | Network/I/O-heavy Rust component | Nested runtime/blocking, lifecycle |
| Host executor calls Rust poll/step API | Tight host scheduling control | More adapter complexity |
| Separate service | Independent scheduler and scale | Protocol/distributed failures |

Creating one Tokio runtime per call is usually wrong. Define runtime singleton/
instance ownership, startup failure, fork behavior where relevant, shutdown/
drain, and what happens to outstanding operations when the host unloads.

## Reentrancy and Locks

```
  host -> Rust object (lock held)
               |
               +-> callback -> host -> same Rust object
                                      |
                                      v
                                   deadlock
```

Never call arbitrary foreign code while holding an internal lock unless
reentrancy is part of a proven protocol. Copy the callback target/state, release
the lock, then invoke. The same rule applies to COM events, Python callbacks,
JNI calls, Node thread-safe functions, and .NET delegates.

## Cancellation and Side Effects

Cancellation is safe only at defined points. A dropped Rust future can stop
mid-protocol; a cancelled host task may leave native work running. For writes:

| Operation | Cancellation design |
|-----------|---------------------|
| Pure compute | Cooperative flag; discard partial result |
| Idempotent remote request | Deadline plus idempotency key |
| File replace | Complete temp write or delete temp; publish atomically |
| Database transaction | Roll back before terminal cancellation where possible |
| Stream | Define partial delivery and resume token |
| External side effect | Record operation identity and reconciliation state |

## Boundary Hazard Register

| Hazard | Async/thread rule |
|--------|-------------------|
| ABI | Callback tables/functions use explicit C/system ABI; futures and trait objects never cross as durable ABI. |
| Allocator | Callback/context/operation owners release on a defined terminal path; no worker frees host memory with Rust allocator. |
| Panic/unwind | Catch/map Rust panics; catch host exceptions in callback adapters; completion still follows terminal-state policy. |
| Lifetime | Retain inputs, callbacks, contexts, runtime, and operation handles until completion/cancel teardown is finished. |
| Threading | Specify affinity, `Send`/`Sync` assumptions, executor, reentrancy, lock policy, and unload/shutdown races. |
| Target | Test scheduler/runtime behavior and callback ABI on each OS/arch/runtime; include single-threaded hosts and apartment models. |
| Packaging | Runtime libraries/features, thread permissions, native dependencies, and host integration glue ship as a coherent version. |

## Old World -> New World Bridge

| Prior async model | Rust migration reading |
|-------------------|------------------------|
| I/O completion ports / reactor | Event source wakes executor task |
| .NET `Task` + `CancellationToken` | Host projection over explicit Rust operation and cancel token |
| Java `CompletableFuture` | Exactly-once terminal completion marshaled through attached JVM context |
| JS Promise + `AbortSignal` | Promise is result projection; AbortSignal is explicit cancellation request |
| COM connection point | Callback subscription with retain/unadvise and apartment rules |
| C completion callback | Function pointer plus context and exact lifetime/thread contract |

## Common Confusion Points

- **"Rust `async fn` starts work when called."** A Rust future is lazy until
  polled/spawned; host tasks/promises may have different start semantics.
- **"Cancellation is an error code."** It is a concurrent state transition with
  races against completion and side effects.
- **"`Send` means callback-safe."** It does not define host runtime attachment,
  UI/event-loop affinity, COM apartments, or reentrancy.
- **"A `void*` context is safe to move to a worker."** It carries no ownership,
  lifetime, affinity, or synchronization guarantee.
- **"Exactly once callback is easy."** Panic, cancellation, shutdown, enqueue
  failure, and duplicate completion all need one atomic terminal transition.
- **"Drop cleans up async work."** Drop cannot await and may run on an
  inconvenient thread.
- **"Holding a lock across callback is faster."** It invites deadlock and
  invariant exposure through reentrancy.

## Decision Cheat Sheet

| Need | Pattern |
|------|---------|
| Short bounded native work | Synchronous call |
| CPU-heavy host request | Worker pool plus host-safe completion |
| Rust network engine | Long-lived Rust runtime owned by component/process |
| Host main-thread callback | Queue through host dispatcher |
| Cancellation | Explicit operation handle/token with documented checkpoints |
| Callback state | Retained context released after terminal completion/unsubscribe |
| Complex scheduler mismatch | Process/service boundary |

## Primary Sources

- Rust async book: https://rust-lang.github.io/async-book/
- Rust `Future`: https://doc.rust-lang.org/std/future/trait.Future.html
- Node-API thread-safe functions: https://nodejs.org/api/n-api.html#asynchronous-thread-safe-function-calls
- JNI invocation API: https://docs.oracle.com/en/java/javase/25/docs/specs/jni/invocation.html
- COM apartments: https://learn.microsoft.com/windows/win32/com/processes--threads--and-apartments

## Related Guides

- Previous: [12-OWNERSHIP-ALLOCATION-ERRORS-AND-UNWINDING-ACROSS-BOUNDARIES.md](12-OWNERSHIP-ALLOCATION-ERRORS-AND-UNWINDING-ACROSS-BOUNDARIES.md)
- Next: [14-PACKAGING-DEPLOYMENT-VERSIONING-AND-SUPPORT.md](14-PACKAGING-DEPLOYMENT-VERSIONING-AND-SUPPORT.md)
- Rust async model: [../rust-language/14-ASYNC-FUTURES-AND-PINNING.md](../rust-language/14-ASYNC-FUTURES-AND-PINNING.md)
