---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-performance:cpu-profiling-and-flame-graphs
kind: guide
module: rust-performance
section: rust-performance
title: CPU Profiling and Flame Graphs
status: source-custody
source_custody: partial
current_path: rust-performance/03-CPU-PROFILING-AND-FLAME-GRAPHS.md
canonical_path: rust-performance/03-CPU-PROFILING-AND-FLAME-GRAPHS.md
backsource_ids: [mdloom-backfill:rust-performance:03-cpu-profiling-and-flame-graphs]
concepts: [cpu profiling, flame graphs, sampling, hardware counters, symbols, off-cpu time]
root_concepts: [cpu profiling]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# CPU Profiling and Flame Graphs

## The Big Picture

A profiler answers attribution: where observed execution time accumulated.
It does not establish workload validity or prove that a proposed change improves
the product.

```
+=============================================================================+
|                          CPU ATTRIBUTION PIPELINE                           |
|                                                                             |
| representative workload [01] -> optimized symbolized binary [02]            |
|             |                              |                                |
|             v                              v                                |
|       sample/trace stacks ---------> symbolize + unwind                     |
|             |                              |                                |
|             +-> on-CPU samples             +-> DWARF / PDB / dSYM           |
|             +-> counters: cycles, IPC, misses                               |
|             +-> off-CPU waits if tool supports them                         |
|                                             |                               |
|                                             v                               |
|                               call tree / flame graph / timeline            |
|                                             |                               |
|                                             v                               |
|                                  hypothesis -> change -> benchmark          |
+=============================================================================+
```

## Sampling, Instrumentation, and Tracing

| Method | Strength | Cost / blind spot |
|--------|----------|-------------------|
| Statistical sampling | Low overhead, production-compatible, call-stack attribution | Small/rare functions may be undersampled |
| Instrumentation | Exact entries/exits for instrumented scope | Perturbs timing and code layout |
| Hardware counters | Explains cycles, branches, cache/TLB behavior | Platform and privilege dependent |
| Timeline tracing | Shows concurrency, waits, wakeups, I/O | Higher data volume and analysis cost |
| Deterministic simulators such as Callgrind | Reproducible instruction/cache model | Slow; model differs from native timing |

Start with sampling. Escalate to counters or tracing when the profile says
"where" but not "why."

## Build for Profiling

Use optimized code with enough symbol and unwind information:

```toml
[profile.profiling]
inherits = "release"
debug = 1
strip = "none"
```

```
# Optional stable flag when stack unwinding quality requires frame pointers.
RUSTFLAGS="-C force-frame-pointers=yes" cargo build --profile profiling
```

Frame pointers can improve stack reliability but may have a small target-specific
cost. Benchmark with the same setting if the conclusion is release-sensitive.
Inlining means a source function may appear folded into callers or represented
as inline frames. Generics create mangled monomorphized symbols; use a profiler
that demangles Rust names.

## Linux: `perf`

```
# Count broad hardware events for one run.
perf stat -- ./target/profiling/my_app workload.json

# Sample call stacks. Call-graph mode depends on platform and unwind data.
perf record -F 199 -g -- ./target/profiling/my_app workload.json
perf report

# External convenience wrapper:
cargo install flamegraph
cargo flamegraph --profile profiling -- workload.json
```

`perf` may be restricted by `kernel.perf_event_paranoid`, containers, or cloud
hypervisors. `cargo flamegraph` is an external Cargo subcommand and depends on
platform tools; it is not part of Cargo or Rust's stability promise. Do not
change security policy merely to make a benchmark convenient without approval.

## Windows and macOS

| Platform | Common choices | Artifact requirement |
|----------|----------------|----------------------|
| Windows | WPR/WPA, PerfView, Visual Studio CPU Usage, `samply` where supported | PDB retained and matched to binary; ETW stack collection configured |
| macOS | Instruments Time Profiler, `xctrace`, `samply` | dSYM/debug data and permitted sampling |
| Linux | `perf`, `samply`, eBPF-based tools | DWARF/symbols; permissions |

Windows example:

```
# Build first; start/stop WPR from an elevated shell only if policy permits.
cargo build --profile profiling
wpr -profiles
wpr -start GeneralProfile -filemode
.\target\profiling\my_app.exe workload.json
wpr -stop cpu-profile.etl
```

Confirm that `GeneralProfile` is listed by the installed WPR version; profile
names and command availability vary by Windows SDK/version.
PerfView can collect and inspect ETW without requiring a Rust-specific runtime.
On Azure, VM size, nested virtualization, host counter availability, and
diagnostics policy can limit hardware counters; sampling stacks still has value.

## Reading a Flame Graph

```
width  = samples attributed to a frame and descendants
height = stack depth, NOT time
x-axis = grouping/layout, NOT chronological order

worker_loop
  +-- read bytes
  +-- parse_record
  |     +-- UTF-8 validate
  |     +-- allocate/copy payload
  +-- emit result
```

Look for broad plateaus and repeated towers, then check:

1. Is the frame inclusive or self time?
2. Is it application work, dependency work, allocator work, syscall, or kernel?
3. Was the process CPU-bound during the window?
4. Are samples missing because stacks failed to unwind?
5. Does the candidate hot path account for enough total time to matter?

Amdahl's law is the first sanity check. If a function is 4% of total sampled
time, eliminating it entirely cannot deliver a 20% end-to-end speedup.

## On-CPU vs Off-CPU

An on-CPU flame graph can make a blocked service look efficient because waiting
threads collect no CPU samples.

```
request latency
   |
   +-> on CPU: parsing, hashing, copying, user code
   |
   +-> runnable but not scheduled: CPU saturation / priority
   |
   +-> blocked: mutex, channel, socket, disk, timer
```

Use scheduler/I/O traces and off-CPU profiling for wall-time questions. On
Linux, `perf sched`, eBPF tools, or `samply` timelines may help. On Windows, ETW
context-switch, disk, TCP, and wait events provide the analogous view.

## Hardware Counters

| Counter pattern | Possible interpretation | Next check |
|-----------------|-------------------------|------------|
| High cycles, low instructions per cycle | stalls, dependency chains, memory latency | cache misses, branch misses, frontend stalls |
| High branch misses | unpredictable control flow | input distribution, branchless/vector alternatives |
| High last-level-cache misses | poor locality or working set too large | data layout, access order, NUMA |
| High instructions but good IPC | excess work, copies, generic expansion | algorithm, data movement, codegen |

Counter names and multiplexing differ by CPU. Virtualized counts may be missing
or approximate. Normalize counts into rates such as misses per access,
instructions per cycle, or events per unit of work, and check whether the tool
multiplexed events. Treat counters as mechanism evidence tied to the observed
machine, not portable laws.

## Old World -> New World Bridge

| Familiar tool/model | Rust-specific addition |
|---------------------|------------------------|
| ETW/PerfView CPU stacks | Native Rust frames and PDB symbol retention |
| `perf` for C/C++ | Same tool; demangle monomorphized Rust symbols |
| Visual Studio native profiler | Cargo profile replaces project configuration |
| Call-tree hot path analysis | Account for inlining, iterator fusion, drop glue, allocator frames |
| CLR wall-clock trace | No GC/JIT events by default; focus on native allocation, scheduler, syscalls |

The profiler remains an OS and machine-code tool. Rust changes symbol shapes and
runtime mechanisms, not the fundamentals of stack sampling.

## Common Confusion Points

- **Flame graph width is not elapsed chronology.**
- **A hot function may be innocent.** It may simply execute the most necessary
  work; optimize causes, not names.
- **Missing symbols can create `[unknown]` plateaus.**
- **Inlining changes attribution.** Inspect caller context and assembly when
  source-level frames are ambiguous.
- **On-CPU profiles miss waits.**
- **Sampling frequency is not automatically "higher is better."** Excessive
  frequency perturbs the workload and creates large traces.
- **One profile is workload-specific.** Change the request mix or target and
  sample again.

## Decision Cheat Sheet

| Question | Tool/approach |
|----------|---------------|
| Where is CPU time? | Sampling call stacks on an optimized symbolized build |
| Is the process CPU-bound? | Wall time vs CPU time plus system utilization |
| Why is a hot loop slow? | Hardware counters, assembly, and [05](05-DATA-LAYOUT-CACHE-LOCALITY-SIMD-AND-VECTORIZATION.md) |
| Why is latency high with low CPU? | Off-CPU/scheduler/I/O trace |
| Are generic abstractions bloating work? | Profile plus [06](06-ITERATORS-GENERICS-DISPATCH-INLINING-AND-CODEGEN.md) |
| Need production-safe attribution? | Low-frequency sampling with symbol custody and overhead limits |
| Need a release claim? | Re-run the benchmark from [01](01-MEASUREMENT-METHODOLOGY-WORKLOADS-BASELINES-AND-VARIANCE.md) after the change |

## Primary Sources

- Linux perf documentation: https://perf.wiki.kernel.org/
- Flame Graphs: https://www.brendangregg.com/flamegraphs.html
- samply: https://github.com/mstange/samply
- Windows Performance Toolkit: https://learn.microsoft.com/windows-hardware/test/wpt/
- Cargo Flamegraph: https://github.com/flamegraph-rs/flamegraph

## Related Guides

- Build symbols correctly: [02-CARGO-PROFILES-RUSTC-OPTIONS-DEBUG-AND-RELEASE-BEHAVIOR.md](02-CARGO-PROFILES-RUSTC-OPTIONS-DEBUG-AND-RELEASE-BEHAVIOR.md)
- Memory attribution: [04-ALLOCATION-OWNERSHIP-MEMORY-FOOTPRINT-AND-ALLOCATORS.md](04-ALLOCATION-OWNERSHIP-MEMORY-FOOTPRINT-AND-ALLOCATORS.md)
- Production use: [14-PRODUCTION-PROFILING-TELEMETRY-CAPACITY-AND-COST.md](14-PRODUCTION-PROFILING-TELEMETRY-CAPACITY-AND-COST.md)
