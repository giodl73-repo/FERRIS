# EXP-01: Owner Granularity, Resources, and Session Contention

Date: 2026-08-08
Question: PERF-Q16
Status: Complete

## Purpose

Measure where nightly rustc frontend jobs reduce metadata-compilation latency,
where serial phases and scheduling overhead prevent gains, how CPU and memory
change, whether incremental and diagnostic correctness hold, and how one Cargo
jobserver differs from multiple independent build sessions.

The primary distribution is direct nightly metadata compilation with one
backend job. No-analysis, default-serial, self-profile, incremental,
diagnostic, direct-process contention, Cargo jobserver, and multi-session runs
are separately labelled controls.

## Environment

- Windows 11 Enterprise Insider Preview, build 26310;
- Intel Core i7-12800HX, 16 cores and 24 logical processors;
- 32 GiB memory;
- local NTFS workspace;
- nightly rustc `1.99.0-nightly (1a98b1e13 2026-08-07)`, LLVM `23.1.0`;
- Cargo `1.95.0 (f2d3ce0bd 2026-03-21)`;
- host `x86_64-pc-windows-msvc`.

Windows process CPU values are quantized at roughly 15.625 ms. They support
aggregate tradeoff comparisons but not fine attribution on tiny fixtures.

## Compiler and orchestration boundaries

The current nightly interface is:

```text
-Z unstable-options --jobs-frontend N
--jobs-backend N
```

`-Zthreads=N` and `-Zno-parallel-backend` are deprecated aliases. With no
frontend setting, the compiler defaults to one frontend job and can use the
serial implementation. Explicit frontend jobs select the dynamically
thread-safe implementation and create a rustc thread pool.

The pinned compiler:

- defaults `jobs.frontend` to one;
- creates a pool with the requested frontend size;
- acquires and releases jobserver tokens around active workers;
- retains one token so a rustc process cannot be permanently starved;
- groups parallel work into at most 128 chunks;
- schedules HIR body owners for type checking, const evaluation, borrow
  checking, liveness, and runtime-ready MIR;
- schedules several module and item checks;
- leaves the current `par_for_each_slice` implementation serial, including
  the import-resolution caller that uses it.

Relevant source:

- [thread-pool construction](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_interface/src/util.rs#L176-L305)
- [jobserver proxy](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_data_structures/src/jobserver.rs#L15-L173)
- [parallel grouping and serial slice helper](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_data_structures/src/sync/parallel.rs#L137-L208)
- [HIR owner and module iterators](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_middle/src/hir/map.rs#L374-L519)
- [type-check owner scheduling](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_hir_analysis/src/lib.rs#L155-L185)
- [borrow-check and module scheduling](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_interface/src/passes.rs#L1110-L1274)
- [import-resolution call site](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_resolve/src/imports.rs#L780-L798)

## Fixtures

| Fixture | Shape | Owners |
|---|---|---:|
| Tiny | One constant function | 1 |
| One body | One body with 10,000 binary operations | 1 |
| Generic owners | Independent generic identity functions | 10,000 |
| Coercion owners | Independent function-item coercion functions | 10,000 |
| Borrow owners | Independent borrowed-value functions | 5,000 |
| Const owners | Independent constants requiring const evaluation | 5,000 |
| Modules | 64 outline modules with 200 generic owners each | 12,800 |

The owner-scaling control generated 100, 1,000, 5,000, and 10,000 identical
generic owners. The Cargo control generated four independent crates with 5,000
generic owners each.

## Primary protocol

The direct-rustc matrix used:

```powershell
rustc <fixture> --crate-name consumer --crate-type lib --edition 2024 `
  --emit metadata -o <output>.rmeta `
  -Z unstable-options --jobs-frontend <1|2|4|8|16> --jobs-backend 1
```

Each pair received one warm-up and 15 interleaved repetitions. The harness
recorded wall time, process CPU time, average occupied cores, peak working set,
metadata bytes, and SHA-256. Every fixture produced one identical metadata hash
across all job counts. Relative MAD remained below 10% in the primary matrix.

## Primary wall-time matrix

| Fixture | j1 | j2 | j4 | j8 | j16 | Best |
|---|---:|---:|---:|---:|---:|---:|
| Tiny | 79.64 ms | 94.49 ms | 93.44 ms | 89.86 ms | 90.78 ms | j1 |
| One body | 417.33 ms | 433.85 ms | 424.21 ms | 429.93 ms | 454.12 ms | j1 |
| Generic owners | 708.22 ms | 604.85 ms | 514.76 ms | 492.31 ms | 525.30 ms | j8 |
| Coercion owners | 646.90 ms | 561.07 ms | 479.79 ms | 468.73 ms | 473.47 ms | j8 |
| Borrow owners | 438.57 ms | 388.79 ms | 335.51 ms | 336.90 ms | 323.52 ms | j16 |
| Const owners | 337.35 ms | 299.88 ms | 259.06 ms | 250.90 ms | 250.20 ms | j16 |
| Modules | 852.43 ms | 744.92 ms | 638.29 ms | 613.02 ms | 654.46 ms | j8 |

The independently schedulable owner fixtures improved by 1.30x to 1.44x at
eight jobs. Tiny and one-large-body controls regressed. Sixteen jobs did not
improve the generic, coercion, or module controls and only narrowly improved
the two 5,000-owner controls.

## CPU and memory tradeoff

Eight jobs versus one:

| Fixture | Wall speedup | CPU ratio | Peak-memory ratio |
|---|---:|---:|---:|
| Generic owners | 1.44x | 1.72x | 1.25x |
| Coercion owners | 1.38x | 1.49x | 1.29x |
| Borrow owners | 1.30x | 1.87x | 1.51x |
| Const owners | 1.34x | 1.72x | 1.29x |
| Modules | 1.39x | 1.74x | 1.22x |

For generic owners, 16 jobs consumed 2.77x the one-job CPU while producing a
slower median than eight jobs. For modules, 16 jobs consumed 2.90x CPU and
regressed from 613.02 to 654.46 ms. More frontend jobs therefore cannot be
treated as free latency reduction.

## Serial and no-analysis controls

Default rustc and explicit one-job parallel mode were interleaved for 15
repetitions. Most differences were within a few percent. The clearest bounded
regression was the const-owner control, from 359.46 to 381.06 ms. This is
consistent with a small compatibility cost rather than a universal penalty.

The no-analysis command:

```powershell
rustc <fixture> --emit metadata -Z no-analysis `
  -Z unstable-options --jobs-frontend <1|8> --jobs-backend 1
```

used 30 interleaved repetitions:

| Fixture | j1 | j8 |
|---|---:|---:|
| Tiny | 86.25 ms | 94.41 ms |
| One body | 140.74 ms | 147.71 ms |
| Modules | 273.85 ms | 287.07 ms |

The parse/expansion boundary did not accelerate. This agrees with the earlier
PERF-Q09 through PERF-Q11 controls and with the pinned serial helper used by
import resolution.

## Owner-count break-even

| Owners | j1 | j2 | j4 | j8 | j16 |
|---:|---:|---:|---:|---:|---:|
| 100 | 120.34 ms | 130.98 ms | 134.08 ms | 124.48 ms | 134.01 ms |
| 1,000 | 171.13 ms | 166.88 ms | 165.55 ms | 166.01 ms | 163.51 ms |
| 5,000 | 427.13 ms | 383.20 ms | 343.25 ms | 325.34 ms | 332.29 ms |
| 10,000 | 726.94 ms | 666.97 ms | 572.19 ms | 520.41 ms | 554.64 ms |

For these trivial owners, 100 owners were below break-even, 1,000 were
approximately neutral, and 5,000–10,000 made four to eight jobs useful.
Owner count is not a universal threshold: work per owner, shared dependencies,
contention, serial phases, and metadata work also matter.

## Process, Cargo, and session contention

Two and four directly launched rustc processes used independent local
jobservers. Four concurrent one-job compilers peaked at 592.86 MiB, while four
eight-job compilers peaked at 743.13 MiB and four 16-job compilers at
755.78 MiB. The fixture did not saturate all 24 logical processors, so 16 jobs
did not produce a makespan regression in that direct control. The important
result is that independent top-level processes did not share one budget.

A single Cargo workspace used four independent 5,000-owner crates:

```powershell
$env:RUSTC = "<nightly rustc>"
$env:RUSTFLAGS = "-Zunstable-options --jobs-frontend=<N> --jobs-backend=1"
cargo check --workspace --offline -j8 --quiet
```

| Frontend jobs | Wall | CPU | Peak process-tree memory |
|---:|---:|---:|---:|
| 1 | 1,029.17 ms | 2,585.94 ms | 425.22 MiB |
| 8 | 902.77 ms | 3,117.19 ms | 557.63 MiB |
| 16 | 887.79 ms | 3,242.19 ms | 561.81 MiB |

Cargo ran four rustc processes concurrently. The inherited jobserver bounded
active work across that one process tree, so 16 configured workers per rustc
did not imply 64 simultaneously active workers. Worker-pool and concurrent
phase memory still increased.

The machine-level control launched four independent Cargo sessions, each with
its own isolated target directory and `cargo -j8`:

| Frontend jobs per session | Four-session makespan | CPU | Peak tree memory |
|---:|---:|---:|---:|
| 1 | 1,750.95 ms | 14,593.75 ms | 1,620.09 MiB |
| 8 | 1,940.88 ms | 17,750.00 ms | 2,114.29 MiB |
| 16 | 1,854.58 ms | 17,765.62 ms | 2,119.25 MiB |

All four top-level Cargo processes created their own coordination domains and
ran up to 16 rustc processes in total. Eight frontend jobs made the batch
10.8% slower than one job, used 21.6% more CPU, and used 30.5% more peak
memory. This is the strongest evidence for a session-level budget above one
Cargo jobserver, especially when terminals, worktrees, editors, CI helpers, or
AI agents compile concurrently.

## Incremental control

A 1,000-owner crate was rebuilt through untouched, identical-rewrite,
one-owner-edit, and shared-const-edit scenarios with separate one- and
eight-job caches. Five minimally instrumented repetitions produced:

| Scenario | j1 | j8 |
|---|---:|---:|
| Untouched | 177.73 ms | 174.81 ms |
| Identical rewrite | 180.68 ms | 199.69 ms |
| One owner edit | 224.90 ms | 229.39 ms |
| Shared const value edit | 221.80 ms | 235.31 ms |

Output hashes matched across job counts for each scenario. Representative
self-profiles showed no body-query providers for untouched and identical
rewrites. The owner edit produced one `typeck_root`, `mir_borrowck`,
`mir_built`, and item-WF miss in both modes. The shared const value edit
produced one const/type/MIR frontier rather than invalidating every referring
function body, because their typed body shape and reference identity did not
change.

There was little parallel work left in these warm scenarios. Eight jobs did
not improve them and sometimes added overhead. This control checks a bounded
incremental path; it does not replace the parallel race and query-cycle test
suite requested by the accepted 2026 project goal.

## Diagnostics and correctness

Twenty independent type mismatches were compiled ten times at one and eight
jobs with a 30-second timeout.

- every run exited with status 1;
- every run emitted all 20 `E0308` errors;
- one-job stderr was byte-identical and ordered by source owner 0 through 19;
- eight-job stderr had ten distinct hashes and ten distinct owner orders.

Parallel execution changed diagnostic presentation order without dropping
errors. Consumers and tests must not assume source order unless rustc provides
an explicit ordering contract. Query-cycle consistency remains a separate
upstream concern named by the 2026 project goal.

Successful metadata was byte-identical across all primary job counts. No
hang, ICE, timeout, or output mismatch occurred in the bounded controls.

## Self-profile observer effect

Three self-profile runs at one and eight jobs used
`-Z self-profile-events=all`. The profiler materially changed the workload:

| Fixture | j1 profiled wall | j8 profiled wall |
|---|---:|---:|
| One body | 561.22 ms | 631.51 ms |
| Generic owners | 1,785.06 ms | 2,626.12 ms |
| Borrow owners | 828.36 ms | 1,256.59 ms |
| Const owners | 867.38 ms | 1,255.36 ms |
| Modules | 2,069.95 ms | 3,590.44 ms |

Parallel query events overlap. Summed query and query-self durations can
exceed wall time, and profiling synchronization reversed the minimally
instrumented speedups. Self-profile remains useful for provider counts and
scheduling evidence, but not as the primary parallel wall benchmark.

## rustc-perf and project-goal boundary

Pinned rustc-perf revision:
`58b05b9a296dfb148aa54c9ec61e8890c65a4223`.

The collector has a frontend-thread test axis and passes
`RUSTC_THREAD_COUNT`. It also contains a hotfix for benchmarks that explicitly
carry the older `-Zthreads` option. Preparation shares a jobserver across Cargo
processes. This is meaningful infrastructure, but it does not by itself supply
the owner-granularity, independent-session, incremental-race, diagnostic-order,
or memory-pressure cases measured here.

- [frontend-thread model](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/src/compile/benchmark/parallel_frontend.rs)
- [benchmark frontend axis and preparation jobserver](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/src/compile/benchmark/mod.rs)
- [collector thread-count integration](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/src/compile/execute/mod.rs)

The accepted 2026 goal explicitly requests remaining incremental and
query-cycle correctness work, a robust parallel frontend test suite, Cargo and
rustc-perf support, more benchmarks, reduced data contention, finer-grained
parallelism, and future parallel name resolution and macro expansion:

- [Promoting Parallel Front End](https://github.com/rust-lang/goals/blob/44ee4db7e818e4ffc7b9f7faadb90316fc0a56f2/src/2026/parallel-front-end.md)

The 2023 announcement reported similar broad expectations: small programs can
regress, values above eight have diminishing returns, memory can rise
substantially, and the jobserver coordinates interprocess with intraprocess
parallelism:

- [Faster compilation with the parallel front-end in nightly](https://blog.rust-lang.org/2023/11/09/parallel-rustc/)

## Limitations

- All fixtures are synthetic and metadata-oriented.
- The primary compiler setting is unstable and nightly-only.
- One Windows hybrid CPU cannot rank policies for other hardware.
- Process CPU is timer-quantized; peak process-tree RSS is sampled.
- The Cargo fixture has four independent crates and no external dependencies.
- Independent-session targets intentionally remove Cargo artifact locking and
  coalescing; shared-target sessions have a different wait/reuse trade studied
  in PERF-Q07.
- The four-session batch measures throughput and resource pressure, not editor
  responsiveness or interactive latency directly.
- Self-profile is a high-overhead observer in the parallel mode.
- No upstream issue, branch, comment, pull request, or benchmark was created.
