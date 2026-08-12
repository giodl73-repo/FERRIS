---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-interop-migration:c-abi-wire-protocols-wit-components-process-boundaries
kind: guide
module: rust-interop-migration
section: computing-software
title: C ABI, Wire Protocols, WIT Components, and Process Boundaries
status: source-custody
source_custody: partial
current_path: rust-interop-migration/02-C-ABI-WIRE-PROTOCOLS-WIT-COMPONENTS-AND-PROCESS-BOUNDARIES.md
canonical_path: rust-interop-migration/02-C-ABI-WIRE-PROTOCOLS-WIT-COMPONENTS-AND-PROCESS-BOUNDARIES.md
backsource_ids: [mdloom-backfill:rust-interop-migration:02-c-abi-wire-protocols-wit-components-process-boundaries]
concepts: [C ABI, wire protocol, WIT, WebAssembly component model, process boundary, IPC, boundary decision]
root_concepts: [interop boundary]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# C ABI, Wire Protocols, WIT Components, and Process Boundaries

Choose the boundary before choosing the binding crate. The central trade is
between **isolation and generality** on one side and **latency and integration
cost** on the other.

## The Big Picture

```
+============================================================================+
|                    INTEROP BOUNDARY DECISION STACK                         |
+============================================================================+
|  MOST ISOLATED / MOST DURABLE                                              |
|                                                                            |
|  1. PROCESS / SERVICE                                                      |
|     HTTP/gRPC/messages; crash, heap, runtime, deploy isolation             |
|             |                                                              |
|             v                                                              |
|  2. WASM COMPONENT + WIT                                                   |
|     typed interface, canonical ABI, sandbox/capability host                |
|             |                                                              |
|             v                                                              |
|  3. C ABI + OPAQUE TYPES                                                   |
|     specified calling convention, low overhead, broad language reach       |
|             |                                                              |
|             v                                                              |
|  4. HOST-SPECIFIC BRIDGE                                                   |
|     pyo3/napi/JNI/PInvoke/cxx/COM; best ergonomics, host coupling          |
|             |                                                              |
|             v                                                              |
|  5. RUST ABI / TRAIT OBJECT                                                |
|     compiler-private; BUILD-LOCAL ONLY, NOT A FOREIGN CONTRACT             |
|  LOWEST CALL OVERHEAD / LEAST DURABLE                                      |
+============================================================================+
```

## C ABI: The Native Lingua Franca

A C ABI surface is appropriate when the call rate or data volume makes a
process hop unjustified and the contract can be reduced to scalars, pointer-plus-
length views, fixed-layout records, callbacks, and opaque handles.

```rust
#[repr(C)]
pub struct RimBytes {
    pub ptr: *const u8,
    pub len: usize,
}

#[unsafe(no_mangle)]
pub extern "C" fn rim_api_version() -> u32 {
    1
}
```

`#[repr(C)]` and `extern "C"` describe layout/calling convention for the exact
target. They do not make `String`, `Vec`, references, enums with unspecified
layout, closures, futures, or trait objects foreign-safe. Keep Rust internals
behind opaque handles and export functions, not data-rich object layouts. A
foreign pointer-plus-length value is still only a claim: the adapter must handle
the zero-length/null convention without constructing an invalid Rust slice and
must validate the documented readable/writable lifetime before dereference.

## Wire Protocols and Process Boundaries

```
  HOST PROCESS
  host allocator + host runtime + old component
          |
          | versioned request/response protocol
          | timeout + error + status
          v
  RUST PROCESS
  Rust allocator + Rust runtime + new capability

  Native stacks, heaps, and unwinds remain process-local.
  Shared external state still needs recovery.
```

Process boundaries are usually the strongest migration boundary because they
isolate heaps, runtimes, global state, and crash domains. The price is explicit:
serialization, transport, authentication, service discovery, deployment, and
distributed failure semantics.

| Protocol form | Good fit | Watch |
|---------------|----------|-------|
| HTTP/JSON | Broad reach, debuggability, moderate throughput | Schema looseness, text cost, retries |
| gRPC/protobuf | Typed service contracts, streaming, generated clients | Compatibility discipline, proxies, deadlines |
| Domain event stream | Asynchronous replacement and replay | Ordering, idempotency, schema registry |
| Local named pipe/Unix socket | Same-host isolation with lower exposure | Platform differences, lifecycle, permissions |
| Custom binary protocol | Proven throughput/latency need | Framing, versioning, tooling, security |

## WIT and the WebAssembly Component Model

WIT describes typed component interfaces independently of a source language:

```wit
package maxim:parser@1.0.0;

interface parse {
  record result-summary {
    records: u32,
    warnings: u32,
  }
  parse: func(input: list<u8>) -> result<result-summary, string>;
}

world parser {
  export parse;
}
```

The component model defines canonical lowering/lifting between component values
and core WebAssembly memories. It can give a cleaner portable contract than a
native ABI and supports capability-oriented hosts. It does **not** remove target
questions: the chosen component runtime, WASI surface, host embeddings, async
model, resource semantics, and deployment format must be tested. Treat support
as an explicit matrix, not as "WebAssembly runs everywhere."

## Decision Matrix

| Criterion | Process/protocol | WIT/component | C ABI | Host bridge |
|-----------|------------------|---------------|-------|-------------|
| Crash isolation | Strong | Sandbox/runtime dependent | None | None |
| Allocator isolation | Strong | Strong across canonical values/resources | Manual | Manual/runtime-specific |
| Per-call overhead | Highest | Medium | Lowest | Low to medium |
| Language reach | Broad | Growing | Broad | One host family |
| Target portability | Protocol dependent | Runtime dependent | Rebuild/test per target | Host/toolchain dependent |
| Rich host ergonomics | Generated client | Generated bindings | Manual wrappers | Best |
| Independent deployment | Yes | Often | Usually coupled | Coupled |
| Rollback routing | Strong | Host-controlled | Loader/feature switch | Host-controlled |

## Version the Contract, Not the Implementation

Use an explicit compatibility mechanism:

- C ABI: version query, size-tagged structs, reserved fields, or `create_v2`.
- Protocol: schema evolution rules, capability negotiation, and unknown-field
  tolerance where the format supports it.
- WIT: versioned packages/worlds and explicit resource lifecycle.
- Process: readiness and minimum/maximum protocol version at startup.

Do not use Rust crate versions or compiler versions as the foreign compatibility
contract. They may be useful build metadata, but consumers need a contract they
can query and test.

## Boundary Hazard Register

| Hazard | Process/protocol | WIT/component | C ABI/host bridge |
|--------|------------------|---------------|-------------------|
| ABI | Wire schema, not machine ABI | Canonical ABI mediated by runtime | Exact calling convention and layout; never Rust ABI or trait objects |
| Allocator | Isolated; bytes copied/streamed | Runtime/resource ownership | Matched allocate/free or caller buffers |
| Panic/unwind | Convert to process error/crash signal | Trap/error mapping | Catch/translate; never cross ordinary FFI |
| Lifetime | Request/message/resource lifetime | Resource handle contract | Borrow duration or opaque handle release |
| Threading | Concurrent requests, ordering, backpressure | Host/runtime scheduling | Affinity, reentrancy, callback thread |
| Target | Transport and server deployment matrix | Runtime/WASI/architecture matrix | OS/arch/libc/CRT/toolchain matrix |
| Packaging | Service/image plus schema/client | Component plus runtime/config | Native artifact plus host package and loader rules |

## Old World -> New World Bridge

| Familiar boundary | Modern migration reading |
|-------------------|--------------------------|
| RPC/IDL | Protocol or WIT schema is canonical; generated stubs are replaceable |
| Shared library API | C ABI facade plus opaque handles |
| AppDomain/process isolation | Sidecar/service seam with explicit failure and deployment |
| COM interface versioning | Capability/version negotiation rather than exposing implementation layout |
| Plugin sandbox | Component runtime with explicit imports/exports and capabilities |

## Common Confusion Points

- **"C ABI is stable everywhere."** It is defined by a platform/toolchain ABI.
  Layout and calling convention still vary by target.
- **"WIT is a Rust ABI replacement."** It is a component-interface system with
  runtime and canonical-ABI machinery, not native Rust-to-Rust dynamic linking.
- **"Process boundaries are always too slow."** Measure batching, streaming,
  locality, and actual latency budgets before assuming this.
- **"Zero copy is automatically better."** It expands lifetime and allocator
  coupling; copies are often the cheapest safety and rollback mechanism.
- **"A host bridge should expose idiomatic Rust types."** The bridge should
  expose idiomatic host types backed by a stable narrow core contract.
- **"Trait objects are equivalent to COM interfaces."** Their vtable layout and
  Rust ABI are not stable foreign contracts. Use explicit function tables with
  a versioned C layout if that pattern is required.

## Decision Cheat Sheet

| Need | Use |
|------|-----|
| Independent rollout, failure isolation, easy routing rollback | Process plus versioned protocol |
| Sandboxed portable component and supported host runtime | WIT/component model |
| Tight in-process latency and broad native reach | C ABI with opaque handles |
| Best ergonomics in one managed/scripting runtime | Host bridge over a narrow universal core |
| Shared rich Rust types between separately built binaries | Do not; compile together or redesign the boundary |
| Extensible plugin function table | Versioned `repr(C)` table with size/version fields, never a Rust trait object |

## Primary Sources

- Rust Reference, external blocks: https://doc.rust-lang.org/reference/items/external-blocks.html
- Rust Reference, type layout: https://doc.rust-lang.org/reference/type-layout.html
- Rustonomicon, FFI: https://doc.rust-lang.org/nomicon/ffi.html
- WebAssembly Component Model guide: https://component-model.bytecodealliance.org/
- WIT reference: https://component-model.bytecodealliance.org/design/wit.html
- Protocol Buffers language guide: https://protobuf.dev/programming-guides/proto3/

## Related Guides

- Previous: [01-ESTATE-DISCOVERY-MIGRATION-ECONOMICS-AND-BOUNDARY-SELECTION.md](01-ESTATE-DISCOVERY-MIGRATION-ECONOMICS-AND-BOUNDARY-SELECTION.md)
- Next: [03-C-INTEROP.md](03-C-INTEROP.md)
- Services and IPC: [11-PROCESSES-SERVICES-IPC-AND-NETWORKING.md](11-PROCESSES-SERVICES-IPC-AND-NETWORKING.md)
