---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-performance:overview
kind: guide
module: rust-performance
section: rust-performance
title: Rust Performance - Landscape and Reading Paths
status: source-custody
source_custody: partial
current_path: rust-performance/00-OVERVIEW.md
canonical_path: rust-performance/00-OVERVIEW.md
backsource_ids: [proof-backfill:rust-performance:00-overview]
concepts: [rust performance, measurement, profiling, optimization, benchmarking, production telemetry]
root_concepts: [rust performance]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Rust Performance - Landscape and Reading Paths

## The Big Picture

Rust removes a managed runtime and garbage collector from the default execution
model; it does not remove queues, cache misses, allocation, syscalls, contention,
or bad algorithms. Performance work is therefore an evidence pipeline, not a
bag of "zero-cost" slogans.

```
+=============================================================================+
|                         PERFORMANCE EVIDENCE FLOW                           |
|                                                                             |
|  QUESTION [01] -> WORKLOAD + BASELINE -> MEASURE -> PROFILE -> CHANGE       |
|                         |              |            |                       |
|                         |              |            +-> CPU [03]            |
|                         |              |            +-> memory [04]         |
|                         |              |            +-> async [07]          |
|                         |              |            +-> contention [08]     |
|                         |              |                                    |
|                         |              +-> runtime latency/throughput       |
|                         |              +-> compile time [12]                |
|                         |                                                   |
|                         +-> build/profile contract [02]                     |
|                                                                             |
|  CHANGE SURFACES                                                            |
|  layout/SIMD [05]  codegen/dispatch [06]  I/O [09]  parsing/data [10]       |
|  link/PGO/size [11]  benchmark gates [13]  production/cost [14]             |
|                                                                             |
|  RELEASE DECISION [15]: evidence, risk, rollback, and workload coverage     |
+=============================================================================+
```

The ordering is deliberate. Guides 01-03 establish measurement and attribution.
Guides 04-11 cover runtime mechanisms. Guide 12 is a separate compile-time
track. Guides 13-15 turn observations into durable gates and production
decisions.

## Four Questions Before Any Optimization

| Question | Required answer |
|----------|-----------------|
| What outcome matters? | p99 latency, sustained throughput, RSS, binary size, build time, cloud cost, or another explicit metric |
| Under what workload? | Input distribution, concurrency, data size, warm/cold state, target, and dependency versions |
| Compared with what? | A preserved baseline built and run under the same contract |
| How will causality be checked? | Profile, counter, trace, allocation record, or controlled experiment |

A result without those boundaries is an anecdote. A benchmark that says
"iterators are 12% faster" is incomplete until it names the iterator pipeline,
input, compiler, target CPU, profile, sample method, and uncertainty. The same
code may be faster on one target and slower on another because vectorization,
branch prediction, cache hierarchy, and linker decisions differ.

## The Three Performance Planes

```
PRODUCT OUTCOME: latency / capacity / cost / user deadline
    |
    +--> RUNTIME PLANE
    |      CPU, memory, I/O, scheduling, locks
    |      guides 03-11 and 14
    |
    +--> DELIVERY PLANE
           compile, link, test, cache, CI critical path
           guide 12
```

Runtime and compile-time performance share causes such as monomorphization and
codegen-unit partitioning, but they are different objectives. A release build
with one codegen unit and fat LTO may improve runtime while making developer
builds much slower. Do not report one plane as if it proves the other.

## Reading Paths

| Intent | Read in order |
|--------|---------------|
| Diagnose a slow service endpoint | [01](01-MEASUREMENT-METHODOLOGY-WORKLOADS-BASELINES-AND-VARIANCE.md) -> [03](03-CPU-PROFILING-AND-FLAME-GRAPHS.md) -> [07](07-ASYNC-RUNTIME-SCHEDULING-TASKS-AND-LATENCY.md) -> [09](09-FILES-NETWORKING-BUFFERING-AND-IO.md) -> [14](14-PRODUCTION-PROFILING-TELEMETRY-CAPACITY-AND-COST.md) |
| Reduce memory or allocator pressure | [01](01-MEASUREMENT-METHODOLOGY-WORKLOADS-BASELINES-AND-VARIANCE.md) -> [04](04-ALLOCATION-OWNERSHIP-MEMORY-FOOTPRINT-AND-ALLOCATORS.md) -> [05](05-DATA-LAYOUT-CACHE-LOCALITY-SIMD-AND-VECTORIZATION.md) -> [10](10-PARSING-SERIALIZATION-COMPRESSION-AND-DATA-MOVEMENT.md) |
| Tune a compute kernel | [01](01-MEASUREMENT-METHODOLOGY-WORKLOADS-BASELINES-AND-VARIANCE.md) -> [03](03-CPU-PROFILING-AND-FLAME-GRAPHS.md) -> [05](05-DATA-LAYOUT-CACHE-LOCALITY-SIMD-AND-VECTORIZATION.md) -> [06](06-ITERATORS-GENERICS-DISPATCH-INLINING-AND-CODEGEN.md) |
| Fix slow builds | [02](02-CARGO-PROFILES-RUSTC-OPTIONS-DEBUG-AND-RELEASE-BEHAVIOR.md) -> [12](12-COMPILE-TIME-PERFORMANCE-WORKSPACES-INCREMENTAL-BUILDS-AND-CI-CACHES.md) |
| Establish regression policy | [13](13-BENCHMARKING-STATISTICS-CRITERION-AND-REGRESSION-GATES.md) -> [14](14-PRODUCTION-PROFILING-TELEMETRY-CAPACITY-AND-COST.md) -> [15](15-OPTIMIZATION-DECISION-MAP-AND-RELEASE-GATE.md) |
| Tune final artifacts | [02](02-CARGO-PROFILES-RUSTC-OPTIONS-DEBUG-AND-RELEASE-BEHAVIOR.md) -> [11](11-LINKING-LTO-PGO-BOLT-AND-BINARY-SIZE.md) -> [15](15-OPTIMIZATION-DECISION-MAP-AND-RELEASE-GATE.md) |

## Evidence Strength

Not all observations deserve the same confidence.

| Evidence | Supports | Does not support |
|----------|----------|------------------|
| Compiler Explorer / assembly inspection | What one compiler emitted for one function and configuration | End-to-end latency or production capacity |
| Microbenchmark | Cost of a bounded operation under a synthetic harness | Whole-system impact without an Amdahl analysis |
| CPU profile | Where sampled on-CPU time accumulated | Off-CPU waits unless the tool records them |
| Allocation profile | Allocation sites and volume in observed paths | Peak live memory unless lifetime is also measured |
| Load test | Behavior under a modeled request mix | Real traffic if the model omits skew, retries, or downstream limits |
| Production telemetry | Actual deployed outcomes | Causality without an experiment or trace |

Latency evidence must also name the load-generation model. A closed-loop client
that waits for each response reduces offered load when the system stalls and can
hide coordinated omission. Open-loop arrival tests, corrected histograms, or
both may be needed when the product claim concerns externally arriving work.

## Stable, Nightly, and External Tools

Most application performance work can remain on stable Rust: Cargo profiles,
stable `rustc -C` codegen options, Criterion, OS profilers, and production
telemetry. Nightly is useful for internal compiler diagnostics and experimental
features, but nightly output is version-sensitive and should not become an
unlabeled release dependency.

```
# Record the environment before an experiment.
rustc -Vv
cargo -V
cargo tree -e features

# Stable release build with symbols useful to profilers.
cargo build --release

# External tools; installation and privilege requirements vary.
perf stat -- ./target/release/my_app       # Linux
samply record ./target/release/my_app      # Linux/macOS, external
```

Windows/MSVC artifacts use PDBs and ETW-oriented tools such as Windows
Performance Recorder/Analyzer, PerfView, or Visual Studio Profiler. Linux
examples commonly use `perf` and DWARF. macOS commonly uses Instruments. The
measurement model is universal; commands are platform-specific.

## Old World -> New World Bridge

| Prior practice | Rust performance equivalent |
|----------------|-----------------------------|
| Native C/C++ perf lab | Same OS counters, profilers, linkers, and cache hierarchy; Rust adds ownership and monomorphization-specific questions |
| BenchmarkDotNet | Criterion or iai-callgrind for controlled Rust benchmarks |
| ETW + PerfView | Still valid for Rust processes on Windows when symbols and stacks are available |
| MSBuild configuration matrix | Cargo profiles plus target/toolchain configuration |
| CLR allocation/GC investigation | Native allocation and lifetime investigation; no GC pauses by default, but allocators and refcounts still cost |
| VSTS gated performance runs | Statistical benchmark gates plus production canary evidence |

The universal bridge is the scientific method: define the operational question,
control the build and workload, observe the limiting resource, change one
mechanism, and re-measure.

## Common Confusion Points

- **"Rust is fast" is not a measurement.** Rust gives strong control over
  representation and runtime mechanisms; an application can still make poor
  choices.
- **Debug behavior is not release behavior.** Optimizer, debug assertions,
  overflow checks, incremental compilation, and codegen-unit defaults differ.
- **A flame graph is not a benchmark.** It attributes sampled time; it does not
  establish whether the run is representative or statistically stable.
- **Lower mean latency can hide worse tails.** Queueing and contention often
  move p95/p99 independently of the mean.
- **Fewer allocations is not automatically faster.** Reuse can retain excess
  capacity, harm locality, or add synchronization.
- **Nightly compiler diagnostics are observations, not stable contracts.**
- **A Windows or Azure result is not universal.** Preserve target, VM shape,
  power policy, filesystem, and network assumptions.

## Decision Cheat Sheet

| If the question is... | Start with |
|-----------------------|------------|
| "Did this change improve the product?" | [01](01-MEASUREMENT-METHODOLOGY-WORKLOADS-BASELINES-AND-VARIANCE.md) and [14](14-PRODUCTION-PROFILING-TELEMETRY-CAPACITY-AND-COST.md) |
| "Where is CPU time going?" | [03](03-CPU-PROFILING-AND-FLAME-GRAPHS.md) |
| "Why is memory high?" | [04](04-ALLOCATION-OWNERSHIP-MEMORY-FOOTPRINT-AND-ALLOCATORS.md) |
| "Why did a rewrite fail to vectorize?" | [05](05-DATA-LAYOUT-CACHE-LOCALITY-SIMD-AND-VECTORIZATION.md) |
| "Are abstractions producing bad code?" | [06](06-ITERATORS-GENERICS-DISPATCH-INLINING-AND-CODEGEN.md) |
| "Why are async tails bad?" | [07](07-ASYNC-RUNTIME-SCHEDULING-TASKS-AND-LATENCY.md) |
| "Why does scaling flatten?" | [08](08-THREADS-SYNCHRONIZATION-ATOMICS-AND-CONTENTION.md) |
| "Is the bottleneck I/O or data movement?" | [09](09-FILES-NETWORKING-BUFFERING-AND-IO.md) and [10](10-PARSING-SERIALIZATION-COMPRESSION-AND-DATA-MOVEMENT.md) |
| "Should we enable LTO/PGO/BOLT?" | [11](11-LINKING-LTO-PGO-BOLT-AND-BINARY-SIZE.md) |
| "Why is CI slow?" | [12](12-COMPILE-TIME-PERFORMANCE-WORKSPACES-INCREMENTAL-BUILDS-AND-CI-CACHES.md) |
| "Can this ship?" | [15](15-OPTIMIZATION-DECISION-MAP-AND-RELEASE-GATE.md) |

## Primary Sources

- The Rust Performance Book: https://nnethercote.github.io/perf-book/
- Cargo profiles: https://doc.rust-lang.org/cargo/reference/profiles.html
- rustc code generation options: https://doc.rust-lang.org/rustc/codegen-options/
- rustc-perf measurement suite and dashboard: https://github.com/rust-lang/rustc-perf
- Criterion.rs book: https://criterion-rs.github.io/book/

## Related Guides

- Next: [01-MEASUREMENT-METHODOLOGY-WORKLOADS-BASELINES-AND-VARIANCE.md](01-MEASUREMENT-METHODOLOGY-WORKLOADS-BASELINES-AND-VARIANCE.md)
- Language foundation: [../rust-language/00-OVERVIEW.md](../rust-language/00-OVERVIEW.md)
- Compiler and Cargo internals: [../rust-architecture/00-OVERVIEW.md](../rust-architecture/00-OVERVIEW.md)
- Module status: [STATUS.md](STATUS.md)
