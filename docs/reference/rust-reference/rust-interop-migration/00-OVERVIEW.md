---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-interop-migration:overview
kind: guide
module: rust-interop-migration
section: computing-software
title: Rust Interop Migration - Landscape and Reading Paths
status: source-custody
source_custody: partial
current_path: rust-interop-migration/00-OVERVIEW.md
canonical_path: rust-interop-migration/00-OVERVIEW.md
backsource_ids: [proof-backfill:rust-interop-migration:00-overview]
concepts: [rust interop, migration, boundary design, ABI, wire protocols, strangler pattern, reading paths]
root_concepts: [rust interop migration]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Rust Interop Migration - Landscape and Reading Paths

Rust migration is not a language-porting exercise. It is a sequence of
**boundary decisions** that let a Rust implementation coexist with an estate
whose callers, data, deployment systems, and operators must continue working.
The safest path starts with universal contracts - process boundaries, wire
protocols, schemas, and C-shaped ABIs - then adds ecosystem-specific bridges.

## The Big Picture

```
+============================================================================+
|                     RUST INTEROP MIGRATION LANDSCAPE                       |
+============================================================================+
|  EXISTING ESTATE                                                           |
|  C/C++ | .NET | Python | Node | JVM | COM/WinRT | DB/files | services      |
|      |                                                                     |
|      v                                                                     |
|  DISCOVER AND SELECT A SEAM                                      [01]      |
|  value, change rate, blast radius, latency, data gravity, reversibility    |
|      |                                                                     |
|      v                                                                     |
|  CHOOSE THE MOST DURABLE BOUNDARY                                [02]      |
|                                                                            |
|  process + protocol > component/WIT > C ABI > runtime bridge > Rust ABI    |
|  durable/broad       portable        native  convenient       NOT durable  |
|      |                      |                    |                       X |
|                 v                                                          |
|  LANGUAGE / PLATFORM ADAPTERS                                    [03-09]   |
|  C | C++ | .NET | Python | Node | JVM | COM/WinRT                          |
|                 |                                                          |
|                 v                                                          |
|  SHARED STATE AND EXECUTION                                      [10-13]   |
|  schemas/files/DB | IPC/network | ownership/errors | async/threading       |
|                 |                                                          |
|                 v                                                          |
|  SHIP, OPERATE, AND EXIT                                         [14-15]   |
|  target matrix | packaging | versioning | strangler | rollback | removal   |
+============================================================================+
```

The ordering is deliberate. A `pyo3`, `napi-rs`, JNI, or COM projection may be
excellent local machinery, but it is not the architecture. The architecture is
the contract that survives a runtime upgrade, target change, rollback, and
eventual removal of either side.

## The Boundary Ladder

| Boundary | Best property | Main cost | Typical lifetime |
|----------|---------------|-----------|------------------|
| Process plus versioned protocol | Failure and allocator isolation; polyglot | Serialization and operational overhead | Longest |
| File, database, or event schema | Loose coupling and replay | Consistency and schema evolution | Long |
| WebAssembly component plus WIT | Typed component contract; portable intent | Runtime/tooling maturity and target constraints | Medium to long |
| C ABI with opaque handles | Low overhead; universal native reach | Manual ownership, layout, unwind, and target discipline | Medium to long |
| Runtime bridge (`pyo3`, `napi-rs`, JNI, P/Invoke) | Native host ergonomics | Host/runtime/version coupling | Medium |
| C++ bridge (`cxx`) or COM/WinRT projection | Richer local integration | Toolchain/platform coupling | Medium |
| Rust ABI, `repr(Rust)`, trait object, compiler metadata | Convenient inside one build | No durable foreign ABI contract | Build-local only |

The last row is not a migration boundary. Rust's native ABI, default layout,
generic instantiations, trait-object vtables, and compiler metadata are not
stable foreign contracts. Distribute Rust-to-Rust libraries as source or pin
the complete toolchain inside one controlled build; expose foreign consumers
through an explicit boundary.

## Reading Paths

```
  PATH A - "Where should Rust enter this estate?"
  01 -> 02 -> 10 -> 11 -> 15
  economics, boundary choice, state, service seam, rollout

  PATH B - "I need an in-process native library"
  02 -> 03 -> 12 -> 13 -> 14
  C ABI, ownership/errors, callbacks/threads, packaging

  PATH C - "I have a managed or scripting host"
  02 -> one of 05/06/07/08 -> 12 -> 13 -> 14
  universal contract first, host adapter second

  PATH D - "Windows is the product boundary"
  02 -> 09 -> 12 -> 13 -> 14 -> 15
  C/system ABI before COM/WinRT projection and rollout

  PATH E - "I need the lowest rollback risk"
  01 -> 11 -> 10 -> 14 -> 15
  process isolation, schema compatibility, deployment, strangler exit
```

## Migration Control Plane

Every migration slice should be represented by one contract record:

| Field | Question |
|-------|----------|
| Consumer set | Which processes, runtimes, versions, and teams call it? |
| Boundary | C ABI, protocol, WIT, schema, or host-specific adapter? |
| Ownership | Who allocates, retains, releases, closes, and cancels? |
| Failure | How do ordinary errors, panics, exceptions, crashes, and timeouts map? |
| Concurrency | Which thread invokes, completes, calls back, or destroys? |
| Target | Which OS, architecture, libc/CRT, calling convention, and runtime versions? |
| Packaging | Which artifact contains the code, how is it discovered, and how is it signed? |
| Compatibility | What old/new producer-consumer combinations are supported? |
| Rollback | Can traffic or loading return to the old implementation without data repair? |
| Exit | What evidence permits deletion of the old path and bridge? |

This record is the interop equivalent of an interface definition plus an
operations runbook. If a field is implicit, it will become an incident.

## Boundary Hazard Register

Every guide in this module makes the same seven hazards explicit:

| Hazard | Default posture |
|--------|-----------------|
| ABI | Use a specified C/system ABI, protocol, schema, or WIT contract; never promise the unstable Rust ABI or trait-object layout. |
| Allocator | The allocating side frees, or the API exposes a matched release function; never assume heaps or CRTs are interchangeable. |
| Panic/unwind | Catch and translate Rust panics where unwind is enabled; never let unwinding cross an ordinary foreign boundary. |
| Lifetime | Prefer copied values or opaque owned handles; every borrowed pointer has a stated validity interval and thread rule. |
| Threading | State affinity, reentrancy, callback thread, synchronization, and cancellation semantics. |
| Target | Test the exact OS/architecture/libc-or-CRT/calling-convention/runtime matrix. |
| Packaging | Treat library discovery, loader paths, symbols, signing, debug artifacts, and dependency closure as contract work. |

## Old World -> New World Bridge

| Prior practice | Migration interpretation |
|----------------|--------------------------|
| CORBA/COM IDL or RPC schema | Contract-first interop: generated projections are adapters, not the canonical behavior |
| DLL export table | C ABI facade around opaque Rust state |
| Service strangler | Route one capability to Rust while preserving the old implementation as rollback |
| Database compatibility window | Expand/contract schema changes that permit mixed old/new binaries |
| Managed safe-handle pattern | Opaque native handle plus deterministic release export |
| Plugin SDK | Versioned host contract, capability negotiation, and load isolation |

Microsoft bridges are useful examples - P/Invoke, COM, WinRT, MSIX, Windows
Service Control Manager - but they are supplemental. The universal problem is
still contract, ownership, failure, scheduling, target, and distribution.

## Common Confusion Points

- **"Interop means FFI."** FFI is one option. A process boundary is often more
  durable and safer because it isolates allocators, crashes, and runtimes.
- **"`repr(C)` makes Rust types portable."** It controls part of layout for a
  target; it does not stabilize `String`, `Vec`, trait objects, references, or
  ownership.
- **"A binding generator defines the contract."** It projects a contract.
  Headers, WIT, protobuf/OpenAPI, or an explicit schema remain the authority.
- **"Same machine means same target."** MSVC versus GNU, libc differences,
  architecture, CRT linkage, and loader policy can invalidate that assumption.
- **"Dual run proves rollback."** Only a tested switch back, with compatible
  state and packaging, proves rollback.
- **"Rust safety covers the boundary."** Rust's compiler cannot prove foreign
  pointer validity, callback behavior, allocator identity, or external schema
  compatibility.

## Decision Cheat Sheet

| Need | Start with | Then read |
|------|------------|-----------|
| Select the first migration slice | Estate economics and seam analysis | [01](01-ESTATE-DISCOVERY-MIGRATION-ECONOMICS-AND-BOUNDARY-SELECTION.md) |
| Compare durable boundary forms | Boundary decision matrix | [02](02-C-ABI-WIRE-PROTOCOLS-WIT-COMPONENTS-AND-PROCESS-BOUNDARIES.md) |
| Embed Rust in a native host | Opaque C ABI facade | [03](03-C-INTEROP.md), [12](12-OWNERSHIP-ALLOCATION-ERRORS-AND-UNWINDING-ACROSS-BOUNDARIES.md) |
| Integrate one language/runtime | Read its adapter guide after 02 | [04](04-CPP-INTEROP.md) through [09](09-COM-WINRT-AND-WINDOWS-NATIVE-BOUNDARIES.md) |
| Share persistent data | Version schema before implementation | [10](10-DATABASES-FILES-SCHEMAS-AND-DATA-FORMATS.md) |
| Maximize rollback isolation | Use a process/service seam | [11](11-PROCESSES-SERVICES-IPC-AND-NETWORKING.md), [15](15-STRANGLER-ROLLOUT-ROLLBACK-AND-EXIT.md) |
| Ship and support native artifacts | Define target and package matrices | [14](14-PACKAGING-DEPLOYMENT-VERSIONING-AND-SUPPORT.md) |

## Primary Sources

- Rust Reference, type layout: https://doc.rust-lang.org/reference/type-layout.html
- Rust Reference, external blocks and ABIs: https://doc.rust-lang.org/reference/items/external-blocks.html
- Rustonomicon, FFI: https://doc.rust-lang.org/nomicon/ffi.html
- WebAssembly Component Model: https://component-model.bytecodealliance.org/
- Microsoft REST API Guidelines: https://github.com/microsoft/api-guidelines

## Related Guides

- Next: [01-ESTATE-DISCOVERY-MIGRATION-ECONOMICS-AND-BOUNDARY-SELECTION.md](01-ESTATE-DISCOVERY-MIGRATION-ECONOMICS-AND-BOUNDARY-SELECTION.md)
- Rust FFI foundations: [../rust-language/17-UNSAFE-RUST-FFI-AND-ABI.md](../rust-language/17-UNSAFE-RUST-FFI-AND-ABI.md)
- Rust artifact model: [../rust-architecture/13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md](../rust-architecture/13-ARTIFACTS-METADATA-LINKING-AND-DEBUG-INFO.md)
- Module status: [STATUS.md](STATUS.md)
