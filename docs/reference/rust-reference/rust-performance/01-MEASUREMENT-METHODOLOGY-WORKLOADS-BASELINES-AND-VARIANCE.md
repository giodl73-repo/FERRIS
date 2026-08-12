---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-performance:measurement-methodology-workloads-baselines-and-variance
kind: guide
module: rust-performance
section: rust-performance
title: Measurement Methodology, Workloads, Baselines, and Variance
status: source-custody
source_custody: partial
current_path: rust-performance/01-MEASUREMENT-METHODOLOGY-WORKLOADS-BASELINES-AND-VARIANCE.md
canonical_path: rust-performance/01-MEASUREMENT-METHODOLOGY-WORKLOADS-BASELINES-AND-VARIANCE.md
backsource_ids: [mdloom-backfill:rust-performance:01-measurement-methodology-workloads-baselines-and-variance]
concepts: [performance measurement, workloads, baselines, variance, latency, throughput, experimental design]
root_concepts: [performance measurement]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Measurement Methodology, Workloads, Baselines, and Variance

## The Big Picture

Optimization begins by making two executions comparable. The benchmark harness
is secondary; the experiment contract is primary.

```
+=============================================================================+
|                         COMPARABLE EXPERIMENTS                              |
|                                                                             |
|  HYPOTHESIS -> METRIC -> WORKLOAD -> BUILD -> ENVIRONMENT -> RUNS -> TEST   |
|                   |          |         |          |          |       |      |
|                   |          |         |          |          |       +-> CI |
|                   |          |         |          |          +-> variance   |
|                   |          |         |          +-> CPU/OS/noise          |
|                   |          |         +-> profile/target/toolchain         |
|                   |          +-> data/concurrency/warmth                    |
|                   +-> latency/throughput/RSS/cost/build time                |
|                                                                             |
|  BASELINE and CANDIDATE differ by one intended cause                        |
+=============================================================================+
```

## Define the Operational Metric

Pick the metric that represents the constraint, not the one easiest to collect.

| Objective | Useful metrics | Typical failure |
|-----------|----------------|-----------------|
| Interactive request | p50, p95, p99, timeout rate | Reporting only mean latency |
| Batch pipeline | records/s, elapsed time, CPU-hours | Ignoring startup or skewed partitions |
| Service capacity | max sustainable throughput at an SLO | Driving overload and calling queue growth "capacity" |
| Memory | peak RSS/working set, steady RSS, allocation rate, live bytes | Treating allocated bytes as resident memory |
| Artifact | compressed/uncompressed binary size, startup page faults | Reporting file size without symbols/strip policy |
| Developer flow | clean build, incremental edit-build, test critical path | Mixing clean and warm-cache builds |
| Cost | requests per core-hour, bytes per request, currency per workload | Comparing different VM or pricing conditions |

Latency and throughput are coupled through queueing. Increase offered load until
the system reaches saturation and latency rises sharply; the throughput just
before SLO failure is often more useful than the absolute maximum.

State whether the generator is **closed loop** (a client waits before issuing
more work) or **open loop** (arrivals follow an external schedule). Closed-loop
tests can reduce offered load during stalls and under-report latency through
coordinated omission. Use a generator/histogram design that records the delay
experienced by the intended arrival process.

## Workload Contract

```
+-------------------------- WORKLOAD MANIFEST ---------------------------------+
| input: corpus version, size distribution, compressibility, hot keys          |
| traffic: arrival process, concurrency, think time, request mix, retries      |
| state: cold/warm caches, connection reuse, filesystem cache, allocator       |
| target: CPU model, core count, RAM, OS/kernel, target triple, power policy   |
| build: rustc -Vv, Cargo.lock, features, profile, RUSTFLAGS, linker           |
| duration: warm-up, measurement window, repetitions, stop conditions          |
+------------------------------------------------------------------------------+
```

Keep workload data immutable or content-addressed when possible. Seed randomized
generators. Record whether inputs fit in cache and whether compression ratio,
Unicode distribution, error rate, or key skew is representative. A parser
benchmark over one tiny ASCII string says little about a production corpus of
large UTF-8 records with malformed tails.

## Baselines and Experimental Design

The safest comparison is an interleaved A/B sequence on the same host:

```
time --------------------------------------------------------------->
      A1  B1  B2  A2  A3  B3  ...   randomized or balanced ordering

A = preserved baseline commit/artifact
B = candidate commit/artifact
```

Interleaving reduces bias from temperature, background maintenance, cloud host
drift, and long-term frequency changes. For microbenchmarks, Criterion automates
sampling and comparison. For services, use separate but equivalent instances or
alternate immutable artifacts on a quiet host. Do not rebuild A with B's
toolchain unless the toolchain change is intentionally part of the experiment.

| Design | Strength | Caveat |
|--------|----------|--------|
| Before/after once | Fast smoke test | Confounds drift with change |
| Repeated A then repeated B | Estimates within-group variance | Time trend can bias groups |
| Interleaved A/B | Controls many time trends | Requires reliable artifact switching |
| Paired input trials | Controls input difficulty | Pairing must remain intact in analysis |
| Production canary | High external validity | More uncontrolled variables; needs guardrails |

## Warm-Up, Steady State, and Cold Paths

Rust AOT binaries have no JIT warm-up, but the system still warms:

- page cache, DNS, TLS sessions, connection pools, and application caches;
- branch predictors and CPU caches;
- lazy initialization and one-time allocation;
- allocator arenas and worker-thread creation;
- cloud CPU boost/frequency and thermal state.

Measure cold startup separately when it matters. For steady-state measurements,
define a warm-up phase and check for a stable measurement window rather than
discarding an arbitrary number of iterations.

## Variance and Statistical Claims

Distributions matter. Report sample count, center, spread, and interval. For
latency, preserve percentiles or the raw histogram; do not average percentiles
from independent shards.

The effective sample count is not automatically the number of loop iterations.
Autocorrelation, batching, repeated requests within one process, and shared host
events reduce independence. High percentiles also need enough observations:
report histogram resolution, count, and confidence appropriate to the tail being
claimed. For paired A/B trials, analyze paired differences or ratios rather than
treating the groups as unrelated.

| Symptom | Likely source | Response |
|---------|---------------|----------|
| Bimodal latency | CPU migration, cache state, periodic work, two request classes | Segment by class; inspect traces |
| Long right tail | queueing, lock convoy, I/O retries, allocator growth | Measure off-CPU time and concurrency |
| Gradual drift | thermal/frequency changes, background jobs, leak/cache growth | Interleave runs; extend observation |
| High run-to-run spread | noisy host, small effect, insufficient duration | Isolate host; increase work/sample |
| Stable microbench, unstable service | omitted subsystem or traffic interaction | Add end-to-end workload |

An effect smaller than normal variance is not "probably real." Increase sample
size, reduce noise, or state that the experiment is inconclusive. Confidence-
interval overlap by itself is not a valid universal decision rule; estimate the
candidate-to-baseline effect under the chosen paired or independent design.
Statistical significance is also not practical significance: a precisely
measured 0.2% improvement may not justify complexity.

## Reproducible Run Record

```
# Stable toolchain and dependency record.
rustc -Vv
cargo -V
cargo metadata --format-version 1 --locked > measurement-metadata.json
cargo build --release --locked

# Linux example: pin one process to CPUs 2-3.
taskset -c 2-3 ./target/release/my_bench

# Windows example: record the process and hardware context separately.
Get-ComputerInfo | Select-Object WindowsVersion, OsBuildNumber, CsProcessors
```

The JSON file is an experiment artifact, not a canonical source guide artifact;
store it with benchmark results if your workflow permits. `taskset` is Linux
specific. Windows processor affinity can be set through PowerShell/.NET APIs or
the test harness, but processor groups on large machines require care. On cloud
VMs, pin the SKU and region, record whether the host is dedicated, and repeat
across instances if the claim must generalize.

## Runtime vs Compile-Time Matrices

Do not combine them into one "performance" score.

| Matrix | Baseline state | Candidate state |
|--------|----------------|-----------------|
| Runtime | Same binary profile, workload, host contract | One code or build change |
| Clean build | Empty `target`, dependencies fetched | Same cache/fetch state |
| Incremental build | Same prior build plus scripted edit | Same prior build plus same edit |
| CI duration | Same runner class and cache-key policy | One graph/cache/tool change |

## Old World -> New World Bridge

This is ordinary experimental design applied to native Rust. The mappings are
direct:

| Prior art | Rust workflow |
|-----------|---------------|
| BenchmarkDotNet job/config | Criterion benchmark group and Cargo profile |
| Perf lab immutable image | Pinned toolchain, lockfile, target, and host image |
| ETW experiment manifest | Workload/build/environment record plus trace |
| A/B service flight | Rust artifact canary under the same routing policy |
| MSBuild clean/incremental timings | Cargo clean build vs scripted incremental edit |

Rust-specific additions are monomorphization, Cargo feature resolution, target
CPU flags, panic strategy, and linker/LTO state. They belong in the build
contract because they can materially change the executable.

## Common Confusion Points

- **Throughput at unconstrained latency is not capacity.** State the SLO.
- **`--release` is not a complete build identity.** Profiles can be overridden;
  targets, features, lockfiles, flags, and linker matter.
- **A warm run does not test startup.** Measure cold and steady-state separately.
- **More iterations do not remove systematic bias.**
- **CPU pinning can distort a multithreaded service.** Use it for controlled
  questions, not automatically.
- **Cloud VM names do not guarantee identical hosts.** Record observed CPU model
  and repeat when portability matters.
- **Percent change without absolute values can mislead.**

## Decision Cheat Sheet

| Situation | Measurement design |
|-----------|--------------------|
| Small local operation | Criterion microbenchmark with `black_box` and preserved baseline |
| CPU-bound pipeline | Fixed corpus, hardware counters, elapsed time, CPU time |
| Concurrent service | Load sweep, latency histogram, throughput at SLO, resource saturation |
| Memory reduction | Allocation profile plus peak/steady RSS under a lifetime-representative workload |
| Startup optimization | Cold process and cold/warm cache cases reported separately |
| Build optimization | Clean and scripted incremental scenarios, each with cache state declared |
| Noisy or small effect | Interleaved paired A/B runs; gate on the effect estimate, uncertainty, and practical threshold |
| Release decision | Feed workload coverage and uncertainty into [15](15-OPTIMIZATION-DECISION-MAP-AND-RELEASE-GATE.md) |

## Primary Sources

- Criterion.rs analysis: https://criterion-rs.github.io/book/analysis.html
- Rust Performance Book, benchmarking: https://nnethercote.github.io/perf-book/benchmarking.html
- Cargo reproducible builds and lockfiles: https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
- HdrHistogram: https://hdrhistogram.github.io/HdrHistogram/

## Related Guides

- Previous: [00-OVERVIEW.md](00-OVERVIEW.md)
- Build identity: [02-CARGO-PROFILES-RUSTC-OPTIONS-DEBUG-AND-RELEASE-BEHAVIOR.md](02-CARGO-PROFILES-RUSTC-OPTIONS-DEBUG-AND-RELEASE-BEHAVIOR.md)
- Statistical gates: [13-BENCHMARKING-STATISTICS-CRITERION-AND-REGRESSION-GATES.md](13-BENCHMARKING-STATISTICS-CRITERION-AND-REGRESSION-GATES.md)
