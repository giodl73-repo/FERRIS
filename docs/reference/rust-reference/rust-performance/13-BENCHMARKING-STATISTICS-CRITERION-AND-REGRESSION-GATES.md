---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-performance:benchmarking-statistics-criterion-and-regression-gates
kind: guide
module: rust-performance
section: rust-performance
title: Benchmarking, Statistics, Criterion, and Regression Gates
status: source-custody
source_custody: partial
current_path: rust-performance/13-BENCHMARKING-STATISTICS-CRITERION-AND-REGRESSION-GATES.md
canonical_path: rust-performance/13-BENCHMARKING-STATISTICS-CRITERION-AND-REGRESSION-GATES.md
backsource_ids: [mdloom-backfill:rust-performance:13-benchmarking-statistics-criterion-and-regression-gates]
concepts: [benchmarking, statistics, criterion, regression gates, microbenchmarks, performance ci]
root_concepts: [performance benchmarking]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Benchmarking, Statistics, Criterion, and Regression Gates

## The Big Picture

A benchmark becomes a gate only after the workload, statistic, noise budget,
effect threshold, and ownership policy are explicit.

```
+=============================================================================+
|                         BENCHMARK TO GATE                                   |
|                                                                             |
| operation/workload -> harness -> repeated samples -> statistical estimate   |
|          |              |              |                  |                 |
|          |              |              +-> variance/outliers                |
|          |              +-> black_box, warm-up, batching                    |
|          +-> input distributions and sizes                                  |
|                                                            |                |
| baseline artifact/result <---------------- comparison ------+               |
|                                                            |                |
|                                           practical threshold + uncertainty |
|                                                            |                |
|                                      pass / warn / investigate / block      |
+=============================================================================+
```

## Benchmark Layers

| Layer | Best for | Main blind spot |
|-------|----------|-----------------|
| Microbenchmark | Tight function/algorithm cost | Whole-system interactions |
| Component benchmark | Parser, database adapter, protocol pipeline | Deployment queues/downstreams |
| End-to-end load test | Capacity and latency SLO | Root-cause attribution |
| Production experiment | Real traffic and cost | Control and repeatability |

Use a portfolio. Microbenchmarks protect important mechanisms; end-to-end tests
protect product outcomes.

## Criterion on Stable Rust

Rust's built-in `#[bench]` test harness remains nightly-only under the `test`
feature in Rust 1.97.1. Criterion is an external stable-compatible crate with
warm-up, repeated measurement, analysis, plots/reports, and baseline comparison.

```toml
[dev-dependencies]
criterion = "0.8" # current compatible line at writing; pin by repository policy

[[bench]]
name = "parse"
harness = false
```

```rust
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;

fn bench_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse");
    for size in [128usize, 4096, 1_048_576] {
        let input = make_input(size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, data| {
            b.iter(|| parse(black_box(data)))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_parse);
criterion_main!(benches);
```

The dependency version above illustrates manifest shape, not a claim that it is
permanently newest; crates.io lists 0.8.2 at this review. Use the workspace's
dependency/MSRV policy and commit the lockfile for binary applications/tools as
appropriate.

```
cargo bench --bench parse
```

Benchmark release-like code. Criterion manages its benchmark profile through
Cargo conventions, but inspect profile overrides and features in the repository.

## Protect the Benchmark from the Optimizer

`std::hint::black_box` discourages constant folding/dead-code removal in
benchmarks. Also:

- consume outputs;
- vary inputs when specialization on constants is not intended;
- exclude setup only if production amortizes it similarly;
- include allocation when production allocates;
- batch tiny operations to exceed timer resolution;
- verify the operation still exists with code inspection.

Do not move expensive setup outside `b.iter` merely to improve the number if
real requests pay that setup. When setup is intentionally excluded but each
iteration needs a fresh input, use Criterion batching APIs such as
`iter_batched` so cloning/allocation policy is explicit instead of silently
reusing mutated state.

## Statistics and Practical Significance

| Concept | Use |
|---------|-----|
| Estimate | Central result for the measured quantity |
| Confidence interval | Uncertainty from samples/model |
| Effect size | Absolute and relative difference |
| Noise floor | Normal host/harness variation |
| Practical threshold | Smallest change worth engineering action |
| Multiple comparisons | Many benchmarks increase false alarms |

Criterion reports statistical comparisons, but the repository must define what
to do with them. A statistically detectable 0.5% regression may be irrelevant;
a noisy 8% p99 regression may be operationally critical.

Iterations inside one benchmark process are not automatically independent
samples. Frequency drift, thermal state, allocator state, batching, and
autocorrelation can make nominal sample counts overstate information. Preserve
raw estimates, inspect distributions, and use paired baseline/candidate runs
when the experiment design supports pairing.

Define two thresholds:

```
warning threshold: investigate/repeat
blocking threshold: fail release/merge unless waived by owner
```

Use absolute budgets where possible: nanoseconds for a primitive, milliseconds
for an endpoint, MiB for memory, seconds for build time.

Do not block solely on a p-value. A robust gate asks whether the estimated
candidate/baseline effect and its uncertainty cross a predeclared practical
threshold. When many benchmarks are tested, control the alert burden with
benchmark families, confirmation runs, or an explicit multiple-comparison
policy rather than pretending every isolated 5% test has the same false-alarm
rate.

## Baseline Custody

Baselines can be:

| Baseline | Strength | Risk |
|----------|----------|------|
| Previous local run | Fast iteration | Host drift and accidental deletion |
| Main-branch artifact on same host | Strong A/B comparison | Requires artifact/build orchestration |
| Historical database | Trends and seasonality | Hardware/toolchain migrations need segmentation |
| Fixed golden number | Simple gate | Becomes stale and hardware-specific |

Criterion supports saving/comparing named baselines, but CLI details can change
across Criterion versions. Pin the crate/tool version and invoke `--help` in CI
instead of assuming an unversioned syntax forever.

For high-confidence gates, build baseline and candidate commits with the same
toolchain, run them interleaved on the same controlled host, and store raw
results plus environment metadata.

## Deterministic Cost Models

`iai-callgrind` and Callgrind-based approaches can compare instruction/cache
models with less wall-clock noise:

| Strength | Limitation |
|----------|------------|
| Stable instruction-like counts | Very slow |
| Good for small CPU kernels | Valgrind support is primarily Linux |
| Less affected by frequency scaling | Model is not native elapsed time |

Use them as complementary regression evidence, not a substitute for native
latency and throughput.

## CI Gate Design

```
fast PR tier:
  correctness + a few low-noise mechanism benchmarks -> warn/block large deltas

controlled perf tier:
  dedicated host + baseline/candidate A/B + full benchmark matrix

production tier:
  canary + SLO/cost guardrails
```

Shared hosted runners are suitable for smoke tests but often too noisy for small
blocking thresholds. On Azure Pipelines or GitHub Actions, use dedicated agents
for tight gates, record CPU/OS/toolchain identity, avoid concurrent jobs, and
separate toolchain migrations from code comparisons.

## Regression Triage

When a gate fires:

1. repeat the paired comparison;
2. check environment/toolchain/dependency drift;
3. inspect neighboring benchmarks and raw distributions;
4. profile the regressed case;
5. reduce to the responsible change;
6. decide fix, accepted trade, or threshold/workload correction;
7. record the disposition.

Never automatically "fix" a red benchmark by widening its threshold without an
owner and evidence.

## Old World -> New World Bridge

| Prior art | Rust |
|-----------|------|
| BenchmarkDotNet | Criterion |
| PerfView trace after regression | `perf`/ETW/Instruments profile of benchmark |
| VSTS performance gate | Dedicated Rust benchmark lane with A/B artifacts |
| Instruction-count simulator | Callgrind/iai-callgrind |
| Load-test SLO gate | Same model for Rust service |
| Golden performance dashboard | Versioned benchmark history segmented by environment |

The universal bridge is that a benchmark suite is executable performance policy,
not merely a collection of timing loops.

## Common Confusion Points

- **Statistical significance is not practical importance.**
- **A microbenchmark cannot prove end-to-end improvement.**
- **`black_box` does not repair an unrealistic workload.**
- **Hosted CI noise can dwarf small effects.**
- **Comparing different toolchains confounds code and compiler changes.**
- **A threshold without an owner decays into noise.**
- **Benchmark setup must match production amortization.**
- **One input size hides algorithmic crossovers.**

## Decision Cheat Sheet

| Need | Benchmark/gate |
|------|----------------|
| Optimize a tight operation | Criterion parameterized microbenchmark |
| Protect a parser/codec | Corpus-based component benchmark with allocations |
| Protect service SLO | Load sweep with latency histogram and throughput-at-SLO |
| Low-noise instruction regression | iai-callgrind plus native validation |
| PR feedback | Large-delta smoke gate or warning on ordinary runner |
| Block small regressions | Dedicated stable hardware with interleaved A/B |
| Toolchain upgrade | New segmented baseline; do not compare as ordinary code change |
| Flaky result | Reduce noise or mark inconclusive, never average it into certainty |

## Primary Sources

- Criterion.rs book: https://criterion-rs.github.io/book/
- `std::hint::black_box`: https://doc.rust-lang.org/std/hint/fn.black_box.html
- Cargo benchmark targets: https://doc.rust-lang.org/cargo/commands/cargo-bench.html
- iai-callgrind: https://github.com/iai-callgrind/iai-callgrind
- HdrHistogram: https://hdrhistogram.github.io/HdrHistogram/

## Related Guides

- Methodology: [01-MEASUREMENT-METHODOLOGY-WORKLOADS-BASELINES-AND-VARIANCE.md](01-MEASUREMENT-METHODOLOGY-WORKLOADS-BASELINES-AND-VARIANCE.md)
- Production validation: [14-PRODUCTION-PROFILING-TELEMETRY-CAPACITY-AND-COST.md](14-PRODUCTION-PROFILING-TELEMETRY-CAPACITY-AND-COST.md)
- Release gate: [15-OPTIMIZATION-DECISION-MAP-AND-RELEASE-GATE.md](15-OPTIMIZATION-DECISION-MAP-AND-RELEASE-GATE.md)
