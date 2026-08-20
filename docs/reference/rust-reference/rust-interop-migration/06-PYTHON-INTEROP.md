---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-interop-migration:python-interop
kind: guide
module: rust-interop-migration
section: computing-software
title: Python Interop
status: source-custody
source_custody: partial
current_path: rust-interop-migration/06-PYTHON-INTEROP.md
canonical_path: rust-interop-migration/06-PYTHON-INTEROP.md
backsource_ids: [proof-backfill:rust-interop-migration:06-python-interop]
concepts: [Python interop, PyO3, maturin, CPython extension, abi3, GIL, Python wheel, buffer protocol]
root_concepts: [Python interop]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Python Interop

For CPython, the usual path is `PyO3` for bindings and `maturin` for wheels.
Keep the Rust core independent of Python, make conversion and GIL boundaries
visible, and treat wheel coverage as part of the API.

## The Big Picture

```
+============================================================================+
|                      PYTHON PACKAGE WITH RUST CORE                         |
+============================================================================+
|  Python users                                                              |
|  import maxim_parser -> classes/functions/exceptions                       |
|      |                                                                     |
|      v                                                                     |
|  PyO3 ADAPTER                                                              |
|  extract Python values | own Py objects | release/attach GIL | map errors  |
|      |                                                                     |
|      v                                                                     |
|  RUST CORE                                                                 |
|  host-neutral types, no PyObject in durable domain interfaces              |
|      |                                                                     |
|      v                                                                     |
|  NATIVE EXTENSION                                                          |
|  CPython ABI or abi3 subset, one artifact per supported platform/tag       |
|      |                                                                     |
|      v                                                                     |
|  WHEEL                                                                     |
|  maturin -> manylinux/musllinux/macOS/Windows + architecture tags          |
+============================================================================+
```

## A Small PyO3 Module

```rust
use pyo3::prelude::*;

#[pyfunction]
fn count_records(py: Python<'_>, data: &[u8]) -> PyResult<usize> {
    let owned = data.to_vec();
    let count = py.detach(move || {
        owned.iter().filter(|&&b| b == b'\n').count()
    });
    Ok(count)
}

#[pymodule]
fn maxim_parser(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(count_records, m)?)?;
    Ok(())
}
```

The exact `PyO3` API evolves; pin the crate and follow its migration guide. The
important contract is stable: conversion occurs at the adapter, CPU-heavy
host-independent work can run without holding Python's interpreter lock, and no
borrowed Python memory outlives the guard/call that validates it.

## Select the Integration Form

| Form | Use when | Trade |
|------|----------|-------|
| PyO3 native extension | Idiomatic CPython module/classes | CPython and wheel matrix coupling |
| `ctypes`/`cffi` over C ABI | Existing stable native C library or alternate Python implementations | More manual projection; simpler core contract |
| Separate service/process | Failure isolation, independent scale/rollout | Serialization and operations |
| WebAssembly in Python host | Sandboxing/portability is proven for workload | Runtime and WASI/component support |

If Python is one of several hosts, do not let `PyObject` become the canonical
domain model. Use a Rust core plus adapters or a C/message-shaped core contract.

## GIL, Threads, and Reentrancy

Python object access requires the interpreter attachment/lock rules of the
supported Python build. Long CPU or blocking work should not monopolize that
lock. Conversely, Rust worker threads that need to create/call Python objects
must attach through PyO3's supported API.

Free-threaded CPython is a separate support profile, not a reason to delete this
analysis. With no process-wide GIL serializing extension access, `#[pyclass]`
state, unsafe assumptions, callbacks, and native libraries need explicit thread
safety under the pinned PyO3/Python versions. A module that has not completed
that audit should declare that it uses the GIL through PyO3's supported module
configuration rather than claiming free-threaded compatibility.

Callbacks from Rust into Python are reentrant application calls, not mere
function pointers. Define:

- whether callbacks run on the initiating Python thread or a Rust worker;
- whether the interpreter lock is held;
- whether callbacks may call back into the same Rust object;
- how exceptions become Rust errors;
- how callback objects remain alive and are released.

## Values, Buffers, and Ownership

| Python surface | Rust adapter posture |
|----------------|----------------------|
| `bytes` | Borrow for a bounded call or copy before releasing interpreter protection |
| `bytearray`/buffer | Respect mutability, contiguity, format, and export lifetime |
| `str` | Convert with explicit Unicode/UTF-8 semantics |
| Python class wrapping Rust state | PyO3 class owns Rust fields or an opaque owner |
| Iterator/generator | Make state, error, and cancellation/drop behavior explicit |
| Exception | Map a Rust error enum/context to a stable Python exception hierarchy |

Zero-copy use of the buffer protocol is valuable only if the producer keeps the
export valid and neither side mutates in violation of the contract. Copying at
the adapter often buys simpler lifetime and GIL behavior.

## Wheel and ABI Strategy

`abi3` can reduce the number of CPython-version-specific wheels by targeting
Python's stable ABI subset, subject to PyO3 feature support and the APIs you use.
It does not collapse OS, architecture, libc, or minimum-platform policy.

```
  Python versions x OS x architecture x libc policy
          |
          +-- CPython-specific wheels: more artifacts, broader API
          |
          +-- abi3 wheels: fewer Python-version variants, constrained API
```

Build Linux wheels in an environment that satisfies the selected
manylinux/musllinux policy; do not relabel an arbitrary developer build.
Validate import in a clean environment, not only `cargo test`.

## Boundary Hazard Register

| Hazard | Python boundary rule |
|--------|----------------------|
| ABI | Use supported CPython/PyO3 ABI or a C ABI; `abi3` only for covered APIs; never expose Rust ABI or trait objects. |
| Allocator | Python owns Python objects; Rust owns Rust allocations; expose owner-side destructors for C ABI buffers/handles. |
| Panic/unwind | Convert Rust panic/error to Python exception or terminal error; no unwind crosses CPython frames. |
| Lifetime | Bind borrowed Python data to GIL/guard scope; copy before detached work unless ownership is explicit. |
| Threading | State interpreter-lock, worker-thread attachment, callback thread, reentrancy, and finalization rules. |
| Target | Test Python versions, implementation, OS, architecture, libc policy, and debug/release constraints claimed. |
| Packaging | Build/tag/audit wheels correctly; include dependent native libraries legally and test clean imports. |

## Old World -> New World Bridge

| Python/native prior art | Rust migration mapping |
|-------------------------|------------------------|
| CPython C extension | PyO3 supplies a safer projection over the same runtime boundary |
| NumPy buffer view | Borrowed pointer/shape/stride contract with explicit export lifetime |
| Cython wrapper | Host adapter that should remain separate from core domain code |
| GIL release around C work | Detach Python while host-neutral Rust work executes |
| Wheel platform tag | Declared native compatibility envelope |

## Common Confusion Points

- **"The GIL makes native code thread-safe."** It protects Python runtime
  invariants, not arbitrary Rust state or foreign libraries.
- **"A free-threaded build removes synchronization concerns."** It removes the
  process-wide GIL; extension state now needs stronger explicit synchronization
  and a separately tested support claim.
- **"`&[u8]` from Python is forever valid."** It is a bounded borrow tied to the
  Python object and adapter guard.
- **"`abi3` means one universal wheel."** OS, architecture, libc, and minimum
  platform still produce distinct artifacts.
- **"A Rust panic becomes a normal Python exception automatically."** Use the
  binding's supported mapping and still design a panic boundary.
- **"Manylinux is just a tag."** It is a build/runtime compatibility policy.
- **"Python classes can expose Rust trait objects."** Expose a Python class over
  concrete opaque Rust state; trait-object ABI is not stable.

## Decision Cheat Sheet

| Need | Use |
|------|-----|
| Idiomatic CPython package | PyO3 plus maturin |
| Fewer CPython-version wheels | `abi3`, after verifying API constraints |
| Multi-language native core | C ABI core plus thin Python adapter |
| CPU-heavy operation | Convert/copy needed inputs, release interpreter lock, compute |
| Large borrowed buffer | Buffer protocol only with explicit contiguity/lifetime rules |
| Failure isolation and easy rollback | Separate process/service |
| Reproducible distribution | CI wheel matrix plus clean-environment import tests |

## Primary Sources

- PyO3 guide: https://pyo3.rs/
- maturin documentation: https://www.maturin.rs/
- Python/C API stable ABI: https://docs.python.org/3/c-api/stable.html
- Python buffer protocol: https://docs.python.org/3/c-api/buffer.html
- Python packaging platform compatibility tags: https://packaging.python.org/specifications/platform-compatibility-tags/

## Related Guides

- Previous: [05-DOTNET-CSHARP-INTEROP.md](05-DOTNET-CSHARP-INTEROP.md)
- Next: [07-NODEJS-JAVASCRIPT-INTEROP.md](07-NODEJS-JAVASCRIPT-INTEROP.md)
- Async and callbacks: [13-ASYNC-THREADING-CALLBACKS-AND-CANCELLATION.md](13-ASYNC-THREADING-CALLBACKS-AND-CANCELLATION.md)
