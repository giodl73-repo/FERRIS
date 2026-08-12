---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-interop-migration:estate-discovery-migration-economics-boundary-selection
kind: guide
module: rust-interop-migration
section: computing-software
title: Estate Discovery, Migration Economics, and Boundary Selection
status: source-custody
source_custody: partial
current_path: rust-interop-migration/01-ESTATE-DISCOVERY-MIGRATION-ECONOMICS-AND-BOUNDARY-SELECTION.md
canonical_path: rust-interop-migration/01-ESTATE-DISCOVERY-MIGRATION-ECONOMICS-AND-BOUNDARY-SELECTION.md
backsource_ids: [mdloom-backfill:rust-interop-migration:01-estate-discovery-migration-economics-boundary-selection]
concepts: [estate discovery, migration economics, boundary selection, dependency graph, data gravity, blast radius, reversibility]
root_concepts: [migration boundary selection]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Estate Discovery, Migration Economics, and Boundary Selection

The first Rust component should not be the module with the most embarrassing
code. It should be the seam where improved safety or performance has meaningful
value **and** coexistence, observation, rollback, and eventual deletion are
economically credible.

## The Big Picture

```
+============================================================================+
|                   FROM ESTATE MAP TO FIRST RUST SLICE                      |
+============================================================================+
|  INVENTORY                                                                 |
|  callers -> binaries -> libraries -> runtimes -> data -> deployment        |
|      |                                                                     |
|      v                                                                     |
|  TRACE REAL COUPLING                                                       |
|  call graph | ownership | callback | thread | schema | loader | operator   |
|      |                                                                     |
|      v                                                                     |
|  SCORE CANDIDATE SEAMS                                                     |
|  value + pain + isolation + testability + reversibility                    |
|       minus change rate + data gravity + fan-out + target/package burden   |
|      |                                                                     |
|      v                                                                     |
|  SELECT BOUNDARY                                                           |
|  process/protocol | WIT/component | C ABI | runtime-specific adapter       |
|      |                                                                     |
|      v                                                                     |
|  DEFINE PROOF                                                              |
|  compatibility matrix + shadow evidence + rollback drill + exit criteria   |
+============================================================================+
```

## Inventory the Estate, Not Just the Repository

A repository graph misses deployment and operational coupling. Build an estate
inventory with one row per deployable or loadable artifact:

| Dimension | Evidence to collect |
|-----------|---------------------|
| Callers | Static references, dynamic loads, RPC clients, scripts, scheduled jobs |
| Runtime | Native, CLR, CPython, Node/V8, JVM, COM apartment, container, WASM host |
| Data | Database tables, files, queues, caches, object stores, in-memory ownership |
| Execution | Threading model, callbacks, async runtime, reentrancy, cancellation |
| Target | OS, architecture, libc/CRT, compiler family, runtime version |
| Packaging | Installer, image, wheel, NuGet/npm/JAR, system package, sidecar |
| Operations | Health checks, dashboards, support owner, rollout mechanism, rollback |
| Change | Release cadence, active feature work, defect rate, compatibility promises |

Start with observed production topology, then reconcile it with source. Dynamic
loading, reflection, plugin folders, COM registration, Python imports, JNI
lookups, and manually copied DLLs routinely evade source-only discovery.

## Draw Three Graphs

```
  CALL GRAPH                 OWNERSHIP GRAPH             DEPLOYMENT GRAPH

  UI -> API -> engine       host owns request           package A
              -> codec      engine borrows bytes           |
                           codec returns buffer             +-> native DLL
                                                           +-> config/schema

  Shows fan-out             Shows free/retain risk       Shows rollback unit
```

The same code boundary can look attractive in the call graph and impossible in
the ownership graph. For example, extracting a parser is easy if it consumes
bytes and returns a copied value; it is hard if callers retain pointers into an
arena whose lifecycle is controlled by a different runtime.

## Migration Economics

Use a score to expose assumptions, not to manufacture precision:

```
candidate value =
    safety exposure reduction
  + performance or resource opportunity
  + defect/change pain
  + organizational learning value
  + boundary isolation
  + test and observability readiness
  + reversibility
  - fan-out and coordination cost
  - state/data gravity
  - target and packaging multiplicity
  - bridge maintenance cost
  - rewrite uncertainty
```

| Candidate trait | Favor early | Defer |
|-----------------|-------------|-------|
| Inputs/outputs | Bytes, records, messages, opaque handles | Shared mutable object graph |
| State | Stateless or explicitly persisted | Hidden process-global state |
| Consumers | One or few controlled callers | Broad plugin/customer SDK |
| Failure | Explicit error contract | Exceptions, callbacks, and partial side effects interleaved |
| Performance | Measurable hotspot with representative corpus | Anecdotal "Rust will be faster" |
| Deployment | Independently routable or replaceable artifact | Monolithic installer with no feature switch |
| Compatibility | Contract tests exist | Behavior encoded only in production folklore |

Do not count memory safety as an automatic return. Quantify the exposure:
unsafe parsing, untrusted input, concurrency defects, crash frequency, patch
latency, or exploitability. Rust is valuable where its guarantees remove real
risk, not where the migration bridge introduces more operational complexity
than the old code carried.

## Boundary Selection Matrix

| Pressure | Prefer |
|----------|--------|
| Maximum failure and rollback isolation | Process/service boundary |
| Existing durable protocol or event stream | Keep protocol; replace one endpoint |
| Portable sandboxed plugin/component | WIT/component model, if host/target support is proven |
| Microsecond in-process calls and C-shaped data | C ABI with opaque handles |
| Rich C++ ownership types under one toolchain | `cxx` behind a narrow bridge |
| Managed/scripting ergonomics for controlled host versions | Host adapter over a C-shaped or message-shaped core |
| Windows platform identity is contractual | C/system ABI core plus COM/WinRT projection |
| Shared mutable heap across runtimes | Redesign the seam; do not start there |

## A Boundary Contract Before Code

Write a one-page contract and make reviewers sign off:

```text
Capability: parse_document_v1
Inputs: pointer + byte length, immutable for call duration
Outputs: opaque result handle or error code
Allocation: Rust allocates result; caller invokes result_free
Failure: no exception/panic crosses; error text copied via two-call buffer API
Threading: handles are thread-confined; distinct handles may run concurrently
Target: x86_64 Windows MSVC and x86_64 Linux glibc
Package: versioned native library beside host package
Compatibility: host N works with native N and N-1
Rollback: feature switch reloads old process implementation
Exit: 30 days at 100 percent, rollback drill passed, old caller count zero
```

This is scoped evidence, not prose aspiration. It gives tests and packaging
work an executable target.

## Boundary Hazard Register

| Hazard | Discovery question | Selection rule |
|--------|--------------------|----------------|
| ABI | Which calling conventions, layouts, symbol names, and compiler/runtime versions exist? | Select an explicit protocol, WIT, or C/system ABI; reject Rust ABI and trait objects as durable contracts. |
| Allocator | Which CRT/runtime owns each allocation today? | Prefer copied values or owner-frees APIs; price matched release exports into the bridge. |
| Panic/unwind | Where do exceptions, SEH, longjmp, or panics travel? | Terminate each failure domain at the seam and translate. |
| Lifetime | Which values are retained after calls and by whom? | Favor bounded borrows and opaque owned handles over shared object graphs. |
| Threading | Are callers reentrant, affine, concurrent, or callback-driven? | Choose a seam whose scheduler and callback rules can be written and tested. |
| Target | How many OS/architecture/libc/CRT/runtime combinations ship? | Multiply bridge cost by the actual matrix, not the developer laptop. |
| Packaging | How are artifacts found, updated, signed, and rolled back? | Select a boundary whose deployment unit can coexist with the old path. |

## Old World -> New World Bridge

| Established practice | Rust migration use |
|----------------------|--------------------|
| Domain decomposition | Find a capability seam, not a file-by-file translation plan |
| Make/buy analysis | Include bridge lifetime, dual operations, and support in migration cost |
| Anti-corruption layer | Keep host types out of the Rust core and Rust types out of the durable foreign contract |
| Expand/contract deployment | Permit old/new implementations and schemas to coexist |
| Performance engineering | Benchmark the whole boundary, including copies and serialization |
| VSTS release gates or modern progressive delivery | Treat rollback evidence as a release criterion, not an incident improvisation |

## Common Confusion Points

- **"Start with a utility because it is small."** Small but ubiquitous utilities
  can have enormous fan-out and packaging cost.
- **"Start with the hotspot."** A hotspot with a shared allocator or callback
  graph may be a poor first seam; measure boundary overhead and rollback.
- **"Generated bindings remove coordination."** They reduce syntax work, not
  compatibility ownership.
- **"One target in CI is representative."** ABI and loader failures are often
  target-specific.
- **"The bridge is temporary, so design quality is less important."** Temporary
  bridges often outlive the migration plan. Version and support them.
- **"Rewrite economics end at feature parity."** Dual-run operations, package
  servicing, telemetry, and deletion cost are part of the investment.

## Decision Cheat Sheet

| Situation | Decision |
|-----------|----------|
| Unclear production callers | Stop and instrument discovery before selecting a seam |
| High-value capability with message-shaped input/output | Strong first candidate |
| Shared mutable heap and cross-runtime callbacks | Redesign or choose a process boundary |
| Many uncontrolled consumers | Preserve the existing durable protocol/ABI; replace behind it |
| Native in-process latency is mandatory | C ABI facade, explicit ownership, per-target conformance |
| Rollback cannot coexist with new data writes | Fix schema/state reversibility before implementation |
| Packaging matrix exceeds test capacity | Reduce supported targets or choose a service boundary |

## Primary Sources

- Martin Fowler, Strangler Fig Application: https://martinfowler.com/bliki/StranglerFigApplication.html
- Rust Reference, external blocks: https://doc.rust-lang.org/reference/items/external-blocks.html
- Rust Reference, type layout: https://doc.rust-lang.org/reference/type-layout.html
- Microsoft REST API Guidelines: https://github.com/microsoft/api-guidelines

## Related Guides

- Previous: [00-OVERVIEW.md](00-OVERVIEW.md)
- Next: [02-C-ABI-WIRE-PROTOCOLS-WIT-COMPONENTS-AND-PROCESS-BOUNDARIES.md](02-C-ABI-WIRE-PROTOCOLS-WIT-COMPONENTS-AND-PROCESS-BOUNDARIES.md)
- Rollout and exit: [15-STRANGLER-ROLLOUT-ROLLBACK-AND-EXIT.md](15-STRANGLER-ROLLOUT-ROLLBACK-AND-EXIT.md)
