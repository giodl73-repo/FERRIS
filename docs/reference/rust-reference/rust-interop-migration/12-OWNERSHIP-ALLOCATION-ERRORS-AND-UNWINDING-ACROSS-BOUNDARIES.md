---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-interop-migration:ownership-allocation-errors-unwinding-across-boundaries
kind: guide
module: rust-interop-migration
section: computing-software
title: Ownership, Allocation, Errors, and Unwinding Across Boundaries
status: source-custody
source_custody: partial
current_path: rust-interop-migration/12-OWNERSHIP-ALLOCATION-ERRORS-AND-UNWINDING-ACROSS-BOUNDARIES.md
canonical_path: rust-interop-migration/12-OWNERSHIP-ALLOCATION-ERRORS-AND-UNWINDING-ACROSS-BOUNDARIES.md
backsource_ids: [mdloom-backfill:rust-interop-migration:12-ownership-allocation-errors-unwinding-across-boundaries]
concepts: [FFI ownership, allocator boundary, opaque handle, borrowed buffer, error mapping, panic containment, unwinding]
root_concepts: [boundary ownership]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Ownership, Allocation, Errors, and Unwinding Across Boundaries

Most interop defects are ownership defects wearing another name: a pointer
outlives its owner, a buffer is freed by the wrong heap, a callback races
destruction, or an exception crosses frames that do not share an unwind model.
Make the state machine explicit at the API.

## The Big Picture

```
+============================================================================+
|                    BOUNDARY VALUE LIFE CYCLE                               |
+============================================================================+
|  CREATE / BORROW                                                           |
|      |                                                                     |
|      v                                                                     |
|  USE under declared validity + aliasing + thread rules                     |
|      |                                                                     |
|      v                                                                     |
|  TERMINAL PATH: success -> return value                                    |
|  TERMINAL PATH: ordinary error -> translate status                         |
|  TERMINAL PATH: panic/exception/crash -> contain at boundary               |
|      |                                                                     |
|      v                                                                     |
|  RELEASE ON OWNING SIDE                                                    |
|  caller buffer | Rust free export | C++ destructor | SafeHandle | Release  |
|                                                                            |
|  forbidden: wrong-side free; unwind through unsupported foreign frames     |
+============================================================================+
```

## Four Ownership Patterns

| Pattern | Contract | Best use |
|---------|----------|----------|
| Call-scoped borrow | Pointer valid and immutable/mutable only during call | Inputs and small synchronous outputs |
| Caller-allocated output | Caller supplies buffer/capacity; callee reports required/written | Strings/bytes with one allocator |
| Opaque owned handle | Creator returns pointer/token; matching owner-side release | Stateful native objects |
| Shared resource with retain/release | Explicit reference counting and thread rules | COM/resources that genuinely need sharing |

Prefer the first three. Shared cross-runtime ownership is the most expensive to
reason about and support.

## Opaque Handle State Machine

```rust
use std::ptr;

pub struct Session {
    generation: u64,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rim_session_new(out: *mut *mut Session) -> i32 {
    if out.is_null() {
        return 1;
    }
    // SAFETY: the caller guarantees `out` is aligned and writable.
    unsafe { out.write(ptr::null_mut()) };
    let raw = Box::into_raw(Box::new(Session { generation: 1 }));
    // SAFETY: ownership of `raw` transfers to the caller.
    unsafe { out.write(raw) };
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rim_session_free(session: *mut Session) {
    if session.is_null() {
        return;
    }
    // SAFETY: contract permits exactly one free for a live handle created above.
    unsafe { drop(Box::from_raw(session)); }
}
```

`Box::from_raw` may be called exactly once for the matching allocation. The
foreign wrapper must prevent use-after-free, double-free, concurrent free/use,
and fabrication of handles. Production handles can add magic/version/generation
checks, but checks reduce accidental misuse; they do not make an arbitrary
pointer safe to dereference. The exports are `unsafe` for Rust callers because
the raw-pointer preconditions remain outside the Rust type system.

## Caller-Allocated Output

```
  call 1: output=NULL, capacity=0 -> required=N
  caller allocates N bytes in its own heap
  call 2: output=buffer, capacity=N -> written=N
```

This two-call pattern keeps allocation local to the consumer. Define whether
the required size includes a NUL terminator, whether output can change between
calls, and how races are handled. Another option is one-shot caller capacity
with a stable `BUFFER_TOO_SMALL` status.

If Rust returns an owned buffer, return pointer, length, capacity only to the
exact Rust release function that reconstructs the allocation. Do not ask a
foreign runtime to infer `Vec` layout or call its allocator.

## Error Taxonomy

| Failure | Boundary representation |
|---------|-------------------------|
| Invalid arguments | Stable status; no partial mutation |
| Domain failure | Stable code plus structured/copyable detail |
| Resource exhaustion | Distinct code; retry/backpressure policy |
| Cancellation/timeout | Distinct terminal state, not generic failure |
| Rust panic | Contained terminal status/log, or process abort by policy |
| Foreign exception | Caught in foreign adapter and translated |
| Process crash | Supervisor/transport failure with correlation evidence |

Do not return a pointer to thread-local or temporary error text unless its
lifetime is deliberately documented. Prefer caller-provided buffers or an
owned error handle with a release function.

## Panic and Unwind Policy

Ordinary `extern "C"` is a non-unwind boundary. If an unwinding panic attempts
to leave it, Rust aborts rather than unwinding through foreign frames. Wrap
exported bodies with `catch_unwind` when the build uses unwinding and the API
requires translation:

```rust
use std::panic::{catch_unwind, AssertUnwindSafe};

fn boundary(body: impl FnOnce() -> i32) -> i32 {
    catch_unwind(AssertUnwindSafe(body)).unwrap_or(255)
}
```

`catch_unwind` is not a general crash catcher. It cannot catch `panic=abort`,
undefined behavior, process termination, or foreign exceptions. `AssertUnwindSafe`
is a proof obligation about captured state, not boilerplate.

Rust also defines unwind-capable ABI strings such as `"C-unwind"` for deliberate
interoperation where both languages, targets, and toolchains support the unwind
contract. This is specialized machinery, not the default migration path. Error
translation at the adapter remains more portable and operable.

## Strings and Encodings

| Form | Contract |
|------|----------|
| UTF-8 pointer plus length | Embedded NUL allowed; validate UTF-8 if text is required |
| C NUL string | No embedded NUL; encoding separately specified |
| UTF-16 pointer plus length | Code units; define treatment of unpaired surrogates |
| `BSTR`/`HSTRING` | Platform owner/release semantics |
| Host string object | Convert in host adapter; do not expose runtime object to Rust core |

Length must be in bytes or code units explicitly. "Characters" is not a memory
extent.

## Boundary Hazard Register

| Hazard | Required invariant |
|--------|--------------------|
| ABI | Every exported type/call has specified representation/calling convention; Rust ABI and trait objects stay private. |
| Allocator | Each allocation names its freeing side and exact release API; no cross-CRT/heap guess. |
| Panic/unwind | Ordinary errors are values; panics/exceptions are caught and translated or process policy aborts. |
| Lifetime | Borrow validity, retention, aliasing, handle state, and callback teardown are stated. |
| Threading | Use/free/callback races are prohibited or synchronized; final release thread is defined where relevant. |
| Target | Pointer width, alignment, enum/integer sizes, CRT/libc, exception model, and atomic guarantees are tested. |
| Packaging | The release function and error/version APIs ship in every artifact version; symbols cannot be stripped accidentally. |

## Old World -> New World Bridge

| Prior model | Rust boundary equivalent |
|-------------|--------------------------|
| RAII / `IDisposable` / `AutoCloseable` | Host owner around opaque handle and matching release |
| HRESULT plus `IErrorInfo` | Stable code plus copied/owned detail |
| COM AddRef/Release | Explicit shared ownership protocol |
| SAL pointer annotations | Written nullability, extent, retention, and thread contract |
| Arena ownership | One owner releases a family of borrowed values together |
| SEH/exception firewall | Catch at adapter; return status across universal boundary |

## Common Confusion Points

- **"The same process has one heap."** Multiple CRTs, runtimes, custom
  allocators, and modules can own incompatible heaps.
- **"Returning pointer and length transfers ownership."** Ownership must be
  stated independently; it may still be a borrow.
- **"Reference counting solves lifetime."** It does not solve cycles, thread
  affinity, callbacks during final release, or ABI layout.
- **"`catch_unwind` catches all failures."** It catches eligible Rust panics
  only.
- **"A status code is enough."** Operators and callers need stable categories,
  detail retrieval, correlation, and retry semantics.
- **"`repr(C)` makes a struct safe."** Fields, pointers, initialization,
  validity, ownership, and target layout still need a contract.

## Decision Cheat Sheet

| Need | Pattern |
|------|---------|
| Synchronous input | Call-scoped pointer-plus-length borrow |
| Variable output | Caller buffer/two-call sizing |
| Stateful object | Opaque handle plus exactly one owner-side free |
| Shared foreign object | Explicit retain/release only when unavoidable |
| Error detail | Stable code plus caller buffer or owned error handle |
| Panic policy | Catch/map in unwind builds or deliberately abort; never cross ordinary boundary |
| Cross-language exceptions | Catch in originating language adapter and translate |

## Primary Sources

- Rustonomicon, FFI: https://doc.rust-lang.org/nomicon/ffi.html
- Rust Reference, behavior considered undefined: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
- `catch_unwind`: https://doc.rust-lang.org/std/panic/fn.catch_unwind.html
- Rust Reference, type layout: https://doc.rust-lang.org/reference/type-layout.html

## Related Guides

- Previous: [11-PROCESSES-SERVICES-IPC-AND-NETWORKING.md](11-PROCESSES-SERVICES-IPC-AND-NETWORKING.md)
- Next: [13-ASYNC-THREADING-CALLBACKS-AND-CANCELLATION.md](13-ASYNC-THREADING-CALLBACKS-AND-CANCELLATION.md)
- C patterns: [03-C-INTEROP.md](03-C-INTEROP.md)
