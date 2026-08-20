# rust-interop-migration/ - Status

Canonical, peer-level reference for introducing Rust into existing native,
managed, scripting, data, service, and Windows estates without treating a
rewrite as the migration plan. The module starts with universal boundary choices
and adds Microsoft-specific bridges as supplemental projections.

## Files

| File | Topic | Status | Coverage notes |
|------|-------|--------|----------------|
| 00-OVERVIEW.md | Landscape and reading paths | done | Boundary ladder, five reading paths, recurring seven-hazard register |
| 01-ESTATE-DISCOVERY-MIGRATION-ECONOMICS-AND-BOUNDARY-SELECTION.md | Estate discovery and migration economics | done | Call/ownership/deployment graphs, seam scoring, contract record |
| 02-C-ABI-WIRE-PROTOCOLS-WIT-COMPONENTS-AND-PROCESS-BOUNDARIES.md | Universal boundary choices | done | Process/protocol, WIT/component, C ABI, host bridge matrix |
| 03-C-INTEROP.md | C interop | done | Opaque handles, pointer contracts, bindgen/cbindgen, link/package topology |
| 04-CPP-INTEROP.md | C++ interop | done | C facade, cxx/autocxx/bindgen, exceptions, toolchain coupling |
| 05-DOTNET-CSHARP-INTEROP.md | .NET/C# interop | done | LibraryImport/PInvoke, SafeHandle, callbacks, NuGet RID assets |
| 06-PYTHON-INTEROP.md | Python interop | done | PyO3/maturin, GIL, buffers, abi3, wheel matrix |
| 07-NODEJS-JAVASCRIPT-INTEROP.md | Node.js/JavaScript interop | done | Node-API/napi-rs, WASM option, event loop, npm prebuilds |
| 08-JVM-INTEROP.md | JVM interop | done | JNI, FFM, references/threads/classloaders, JAR native packaging |
| 09-COM-WINRT-AND-WINDOWS-NATIVE-BOUNDARIES.md | Windows-native boundaries | done | Win32/system ABI, COM identity/apartments, WinRT projection, Windows packaging |
| 10-DATABASES-FILES-SCHEMAS-AND-DATA-FORMATS.md | Data interop | done | Expand/migrate/contract, formats, dual-write risk, file atomicity |
| 11-PROCESSES-SERVICES-IPC-AND-NETWORKING.md | Process and service interop | done | IPC choices, protocol contract, backpressure, shared memory, rollout modes |
| 12-OWNERSHIP-ALLOCATION-ERRORS-AND-UNWINDING-ACROSS-BOUNDARIES.md | Cross-boundary correctness | done | Ownership patterns, allocator matching, status/error taxonomy, unwind policy |
| 13-ASYNC-THREADING-CALLBACKS-AND-CANCELLATION.md | Execution interop | done | Operation state machine, callbacks, runtimes, reentrancy, cancellation |
| 14-PACKAGING-DEPLOYMENT-VERSIONING-AND-SUPPORT.md | Delivery and servicing | done | Artifact/target matrices, loader closure, contract versions, support policy |
| 15-STRANGLER-ROLLOUT-ROLLBACK-AND-EXIT.md | Rollout and exit | done | Shadow/canary gates, semantic diff, rollback levels, observability, deletion |

All 16 numbered canonical guides are **done**.

## Validation Shape

```
guides + frontmatter + internal links + examples
                    |
                    v
              proof check
                    |
                    v
       zero errors and zero warnings
```

## Quality and Scope Notes

- All guides use `maxim.frontmatter.v1`, module
  `rust-interop-migration`, `status: source-custody`, and
  `source_custody: partial`, with canonical paths, stable IDs, concepts, and
  `proof-backfill` backsource IDs.
- Every guide includes the seven MAXIM surfaces: an opening Big Picture diagram,
  layered drill-down, additional ASCII structure where useful, comparison or
  decision tables, an old-world/new-world bridge, Common Confusion Points, and
  a Decision Cheat Sheet.
- Every guide explicitly addresses ABI, allocator, panic/unwind, lifetime,
  threading, target, and packaging hazards. The module repeatedly rejects the
  unstable Rust ABI, default Rust layout, and Rust trait objects as durable
  foreign boundaries.
- Universal options - process/protocol, versioned schema, WIT/component, and C
  ABI - precede runtime-specific adapters. P/Invoke, COM, WinRT, NuGet RID, MSIX,
  and Windows service/loader concepts are valuable supplemental bridges rather
  than load-bearing architecture.
- Snippets are executable or deliberately scoped to a pinned host/binding API
  shape. Raw-pointer examples handle the null-plus-zero case without creating
  invalid Rust slices; host examples balance runtime initialization, bound
  framing, and state calling-convention/native-access assumptions. Generated
  bindings are not described as safety proofs.
- Source families are primarily official language/runtime/platform references:
  Rust Reference/Nomicon/std, WebAssembly Component Model/WIT, Microsoft .NET
  and Windows documentation, Python C API/PyO3/maturin, Node-API/napi-rs,
  OpenJDK/JNI, protobuf/Arrow/Parquet, gRPC, and OpenTelemetry.
- The four role lenses were applied: Reader Path Editor (entry paths and sibling
  links), Reference Integrity Auditor (bounded claims and explicit caveats),
  Executable Evidence Auditor (scoped versions/targets and reproducible
  examples), and Learner Advocate (why-first orientation and decision utility).
- No unresolved inline editorial issue tags are present. This is source-custody
  content, not a Gold certification claim.

## Source-Custody Posture

The numbered guides are canonical source. Per task scope, no source-backfill,
generated `.proof`/`.mdcrop`/`.mdport`/`.fletch` artifacts, navigation files,
portfolio trackers, or files outside this directory were created or modified.
The module is ready for a later explicitly scoped backfill/registration pass.

## Decision Cheat Sheet

| Question | Status answer |
|----------|---------------|
| Are all 16 guides present? | Yes |
| Are Rust ABI and trait objects rejected as durable foreign contracts? | Yes, throughout |
| Are all seven MAXIM surfaces present? | Yes |
| Is source custody complete? | No; it remains partial until an explicitly scoped backfill |
| Is Certified Gold claimed? | No; this is an independently corrected, mechanically clean source module |
