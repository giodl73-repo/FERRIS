# EXP-01: Cargo critical-path scheduling

Date: 2026-08-08
Question: PERF-Q03
Status: Complete

## Purpose

Observe how Cargo orders ready units under constrained parallelism and compare:

- dependency readiness;
- ready-queue delay;
- unit overlap;
- job-count sensitivity;
- a deliberately slow root-gating unit;
- a manual prebuild intervention;
- a small public fixture.

## Environment

- Windows 11 Enterprise Insider Preview `10.0.26310`
- 12th Gen Intel Core i7-12800HX
- 24 logical processors
- 31.7 GiB memory
- NTFS
- stable baseline: Cargo `1.95.0`, rustc `1.95.0`
- diagnostic nightly: Cargo and rustc
  `1.99.0-nightly (1a98b1e13 2026-08-07)`
- diagnostic LLVM `23.1.0`
- Cargo source reviewed at
  `21c2a90636b4a1991eacd14eca439e7e308c1af4`

Machine contention and background activity were not controlled beyond using
new target directories and sequential runs. Results are exploratory.

## Fixtures

### Synthetic scheduling control

An eight-member resolver-v2 workspace:

```text
schedule-app -> a1 -> a2 -> a3
             -> b1 -> b2 -> b3
             -> long-gate
```

Each non-root package has a build script. The six chain build scripts sleep for
150 milliseconds. The direct `long-gate` dependency's build script sleeps for
1.6 seconds.

The cold check graph contained 22 Cargo units:

- seven build-script compilations;
- seven build-script executions;
- seven library checks;
- one application check.

The fixture and raw outputs were retained outside the repository. No product
source code was added.

### Public fixture

METIS-CORE revision:
`78ae34090e043e79a206f2daffaa3889389b4790`.

Generated lockfile SHA-256:
`1CAC404E926E148B0471233D14629D31BA66086F10903269E46B48109D7D6CAE`.

The cold `cargo check` graph contained 16 units.

## Commands

Every primary sample used a new empty target directory:

```powershell
cargo check --locked --offline `
  --manifest-path <fixture>\Cargo.toml `
  --target-dir <new-target> `
  --jobs <1|2|4|24> `
  --message-format=json-render-diagnostics
```

PowerShell `Diagnostics.Stopwatch` measured the complete Cargo process. Standard
output and standard error were captured separately.

The manual prebuild comparison timed both commands together against the same
new target directory:

```powershell
cargo check -p long-gate --locked --offline `
  --manifest-path <synthetic>\Cargo.toml `
  --target-dir <new-target> `
  --jobs 2

cargo check --workspace --locked --offline `
  --manifest-path <synthetic>\Cargo.toml `
  --target-dir <new-target> `
  --jobs 2
```

Representative diagnostic collection:

```powershell
cargo +nightly -Zbuild-analysis `
  --config 'build.analysis.enabled=true' `
  check --locked --offline `
  --manifest-path <fixture>\Cargo.toml `
  --target-dir <new-target> `
  --jobs <1|2|4|24> `
  --message-format=json-render-diagnostics
```

Build-analysis JSONL was parsed using registered dependency indexes and unit
start/finish events. For each unit:

- graph-ready time was the maximum finish time of its dependencies, or
  unit-graph completion when it had no dependencies;
- queue delay was start time minus graph-ready time;
- summed unit work was the sum of all unit start-to-finish durations;
- makespan was final unit finish minus unit-graph completion;
- average active jobs was summed unit work divided by makespan.

These diagnostic runs were not included in the primary wall-clock medians.

## Primary wall-clock results

### Synthetic fixture

| Jobs | Samples | Median | MAD | MAD/median |
| ---: | --- | ---: | ---: | ---: |
| 1 | 11,579.57, 11,643.87, 11,643.94 ms | 11,643.87 ms | 0.07 ms | 0.001% |
| 2 | 7,465.64, 8,838.16, 8,947.46 ms | 8,838.16 ms | 109.30 ms | 1.24% |
| 4 | 7,203.49, 8,411.54, 10,071.54 ms | 8,411.54 ms | 1,208.05 ms | 14.36% |
| 24 | 8,318.15, 8,413.52, 8,992.89 ms | 8,413.52 ms | 95.37 ms | 1.13% |

The four-job series is unstable under the measurement contract and cannot
support an optimization claim.

Relative to the one-job median, the exploratory medians improved by:

- 24.1% at two jobs;
- 27.8% at four jobs;
- 27.7% at 24 jobs.

The additional median improvement after two jobs was only about 4.8%.

### METIS-CORE

| Jobs | Samples | Median | MAD | MAD/median |
| ---: | --- | ---: | ---: | ---: |
| 1 | 10,260.80, 11,098.37, 15,646.80 ms | 11,098.37 ms | 837.57 ms | 7.55% |
| 2 | 7,565.18, 8,078.89, 8,422.07 ms | 8,078.89 ms | 343.18 ms | 4.25% |
| 4 | 7,932.05, 8,025.21, 8,614.46 ms | 8,025.21 ms | 93.16 ms | 1.16% |
| 24 | 7,332.49, 7,716.36, 8,497.54 ms | 7,716.36 ms | 383.87 ms | 4.97% |

Relative to the one-job median, the exploratory medians improved by:

- 27.2% at two jobs;
- 27.7% at four jobs;
- 30.5% at 24 jobs.

The 24-job median was about 4.5% faster than the two-job median.

All series contain only three samples. They remain exploratory even when their
MAD ratio is below the instability threshold.

## Diagnostic schedule results

### Synthetic fixture

| Jobs | Makespan | Summed unit work | Average active jobs | Peak | Long-gate compile start | Start rank | Queue delay |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 10.9063 s | 10.8991 s | 0.999 | 1 | 5.3564 s | 11 | 5.3564 s |
| 2 | 8.1545 s | 15.9163 s | 1.952 | 2 | 4.6270 s | 13 | 4.6270 s |
| 4 | 8.0843 s | 25.7672 s | 3.187 | 4 | 3.7010 s | 12 | 3.7010 s |
| 24 | 6.4846 s | 39.2653 s | 6.055 | 7 | 0.0474 s | 5 | 0.0474 s |

At two jobs, Cargo began by advancing the two three-crate chains. The
`long-gate` build-script compilation was ready immediately but started
thirteenth. Its build-script execution, library check, and final application
check then formed the observed completion-gating chain.

Configured capacity and observed Cargo concurrency differed sharply at 24
jobs: only seven units overlapped at peak.

Summed unit work increased with configured parallelism. Because each job-count
trace came from a separate diagnostic run, unit durations are not fixed costs.
Instrumentation, process startup, CPU scheduling, storage, antivirus, memory,
and thermal or power behavior may contribute.

### METIS-CORE diagnostic comparison

| Jobs | Makespan | Summed unit work | Average active jobs | Peak | Maximum ready-queue delay |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 2 | 7.1857 s | 11.7335 s | 1.633 | 2 | 2.0314 s |
| 24 | 7.9604 s | 16.4294 s | 2.064 | 6 | 0.0011 s |

The 24-job diagnostic trace nearly eliminated ready-queue delay but was slower
than the two-job diagnostic trace. The primary uninstrumented medians showed a
different ordering. This is direct evidence that a diagnostic schedule trace
must explain execution rather than serve as the benchmark result.

## Manual prebuild result

At two jobs, prebuilding `long-gate` and then checking the workspace produced:

| Samples | Median | MAD |
| --- | ---: | ---: |
| 9,628.22, 10,214.22, 10,982.80 ms | 10,214.22 ms | 586.00 ms |

The ordinary two-job workspace median was 8,838.16 milliseconds. The manual
prebuild was about 15.6% slower because it serialized work that Cargo could
otherwise overlap.

## Failed and negative evidence

- An initial METIS diagnostic attempt used an isolated empty `CARGO_HOME` with
  `--offline` and exited `101` because the registry cache did not contain
  `rand`. It was not counted as a build sample.
- The synthetic four-job wall-clock series exceeded the contract's 10%
  instability threshold.
- The manual prebuild intervention was slower.
- The 24-job METIS diagnostic trace was slower than its two-job diagnostic
  trace even though ready-queue delay was lower.

## Interpretation

1. Cargo's fixed-cost transitive-fan-out heuristic can delay a slow direct
   dependency that gates the requested root.
2. More configured jobs reduce queueing only when ready graph work exists.
3. Lower queue delay does not guarantee lower wall time because overlapping
   work changes unit duration and resource contention.
4. A slow crate should not be manually prebuilt merely because it appears late
   in one schedule.
5. The useful product surface is explanation and counterfactual analysis, not
   automatic reordering.

## Limitations

- One synthetic workspace and one small public fixture.
- One Windows machine and filesystem.
- Three wall-clock samples per job count.
- Cold `cargo check` only; no build, test, release, warm edit, or link workload.
- Delays were implemented with build-script sleeps rather than CPU-heavy rustc
  work.
- No CPU, memory, I/O, frequency, thermal, or process-resource trace.
- Nightly build-analysis schema is unstable.
- No larger workspace and no held-out historical-duration prediction.
- Diagnostic runs were collected sequentially and observed different machine
  states.

