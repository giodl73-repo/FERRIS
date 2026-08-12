# rust-performance/ - Status

**17 files (STATUS.md + 16 canonical guides) | Complete | Source-first, no generated artifacts**

This module is a peer-level Rust performance reference. It treats performance as
an evidence and release discipline: define a workload and baseline, separate
runtime from compile-time behavior, attribute the limiting resource, change the
lowest-risk mechanism, and validate the final artifact in production. Universal
native-systems guidance comes first; Windows, ETW, Azure, MSVC, PDB, and Azure
Pipelines bridges are supplemental.

## Validation Landscape

```
guides + STATUS
      |
      +-> four-role adversarial review
      +-> seven-surface structural review
      +-> mdloom module check
      +-> frontmatter, link, fence, example, and scoped-diff checks
      |
      v
clean canonical source; no generated or external edits
```

## Guides

| File | Topic | Status |
|------|-------|--------|
| `00-OVERVIEW.md` | Performance landscape, evidence hierarchy, three planes, and reading paths | done |
| `01-MEASUREMENT-METHODOLOGY-WORKLOADS-BASELINES-AND-VARIANCE.md` | Operational metrics, workload contracts, baselines, warm/cold state, variance, and reproducible experiments | done |
| `02-CARGO-PROFILES-RUSTC-OPTIONS-DEBUG-AND-RELEASE-BEHAVIOR.md` | Cargo profile policy, stable rustc codegen controls, debug/release differences, targets, and nightly boundaries | done |
| `03-CPU-PROFILING-AND-FLAME-GRAPHS.md` | Sampling, symbols, flame graphs, off-CPU time, hardware counters, and platform profilers | done |
| `04-ALLOCATION-OWNERSHIP-MEMORY-FOOTPRINT-AND-ALLOCATORS.md` | Allocation metrics, ownership/copies, collection capacity, allocators, refcounts, and RSS | done |
| `05-DATA-LAYOUT-CACHE-LOCALITY-SIMD-AND-VECTORIZATION.md` | Layout contracts, padding, AoS/SoA, locality, auto-vectorization, intrinsics, and portable-SIMD caveats | done |
| `06-ITERATORS-GENERICS-DISPATCH-INLINING-AND-CODEGEN.md` | Iterator fusion, bounds checks, dispatch, monomorphization, inlining, assembly, and code size | done |
| `07-ASYNC-RUNTIME-SCHEDULING-TASKS-AND-LATENCY.md` | Executors, polling, blocking isolation, backpressure, task granularity, runtime metrics, and tails | done |
| `08-THREADS-SYNCHRONIZATION-ATOMICS-AND-CONTENTION.md` | Decomposition, locks, channels, atomics, false sharing, oversubscription, and contention traces | done |
| `09-FILES-NETWORKING-BUFFERING-AND-IO.md` | Buffered/vectored I/O, durability, page cache, networking, async engines, and memory mapping | done |
| `10-PARSING-SERIALIZATION-COMPRESSION-AND-DATA-MOVEMENT.md` | Format contracts, borrowed/owned parsing, streaming, Serde, compression, copies, and corpus benchmarks | done |
| `11-LINKING-LTO-PGO-BOLT-AND-BINARY-SIZE.md` | Linkers, Thin/Fat LTO, PGO, target-specific BOLT, size analysis, symbols, and final-artifact verification | done |
| `12-COMPILE-TIME-PERFORMANCE-WORKSPACES-INCREMENTAL-BUILDS-AND-CI-CACHES.md` | Build scenarios, Cargo timings, crate boundaries, incremental layers, proc macros, sccache, and CI caches | done |
| `13-BENCHMARKING-STATISTICS-CRITERION-AND-REGRESSION-GATES.md` | Benchmark layers, Criterion, benchmark integrity, statistics, baselines, deterministic models, and CI gates | done |
| `14-PRODUCTION-PROFILING-TELEMETRY-CAPACITY-AND-COST.md` | RED/USE telemetry, traces, production profiling, symbol custody, capacity curves, cost, and canaries | done |
| `15-OPTIMIZATION-DECISION-MAP-AND-RELEASE-GATE.md` | Evidence packet, optimization order, workload matrix, release gate, stop rules, and rollback | done |

## Quality Notes

- All 16 guides use `maxim.frontmatter.v1`, `module: rust-performance`,
  `status: source-custody`, `source_custody: partial`, canonical/current paths,
  unique `maxim:rust-performance:*` IDs, and matching
  `mdloom-backfill:rust-performance:*` backsource IDs.
- Every guide contains all seven MAXIM style surfaces: Big Picture ASCII map,
  layered drill-down, additional ASCII structure, comparison/decision tables,
  Old World -> New World bridge, Common Confusion Points, and Decision Cheat
  Sheet.
- Commands state stable/nightly/external-tool and platform boundaries. Stable
  Rust is the default; `-Z`, compiler internals, portable SIMD status, allocator
  API status, self-profile, and tool-version-sensitive surfaces are explicitly
  caveated.
- Claims distinguish measured behavior from folklore and workload-specific
  results from general mechanisms. Runtime, compile-time, link-time, binary-size,
  and production-cost objectives remain separate.
- Cross-links connect the sequence and bridge to `rust-language/` and
  `rust-architecture/` without duplicating their language/compiler roles.
- Primary-source blocks favor official Rust/Cargo/rustc/platform/tool
  documentation; external ecosystem tools are labeled as external.
- All diagrams use ASCII-safe `+`, `-`, `|`, `<`, `>`, `=`, `/`, and `\`
  characters rather than box-drawing glyphs.
- Explicit canonical-guide lint result: **16 files checked, 0 errors, 0 warnings**.

## Four-Role Cross-Review

| Role | Result |
|------|--------|
| Reader Path Editor | Overview offers task-based paths; each guide links adjacent decision surfaces and orients before details |
| Reference Integrity Auditor | Strong claims are bounded by workload, target, version, and authority; no Gold claim is made |
| Executable Evidence Auditor | Commands include toolchain/platform caveats; stable and nightly surfaces are separated |
| Learner Advocate | Peer-level explanations introduce Rust-specific tools and vocabulary without re-teaching compiler, OS, or statistics fundamentals |

No inline review tags remain because identified issues were fixed directly. The
module is source-first and intentionally has **not** run source-backfill or
created `.mdloom`, CROP, MDPORT, FLETCH, or other generated artifacts, as
required by this task. It is not Certified Gold; this pass applied Gold-level
skepticism but was not a certification panel or registry update.

## Decision Cheat Sheet

| Question | Source of truth |
|----------|-----------------|
| Which guides are canonical? | The 16 numbered files listed above |
| Is source custody complete? | No; `source_custody: partial` is explicit |
| Were generated stores updated? | No; this module-only review forbids backfill |
| Does structural validation pass? | Use the recorded mdloom result below |
| Is Gold certification claimed? | No; Gold-level skepticism was applied without certification |

## Completion

2026-08-11 - Independently cross-reviewed and corrected all 16 numbered guides
and STATUS. Exactly this directory was changed; no repository navigation,
generated output, backfill, commit, or external file was modified.
