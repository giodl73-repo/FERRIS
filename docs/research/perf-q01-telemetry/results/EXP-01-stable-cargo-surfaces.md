# EXP-01: Stable Cargo telemetry surfaces

Date: 2026-08-07
Question: PERF-Q01
Fixture: public METIS-CORE repository
Revision: `78ae34090e043e79a206f2daffaa3889389b4790`
Generated lockfile SHA-256:
`1CAC404E926E148B0471233D14629D31BA66086F10903269E46B48109D7D6CAE`

## Environment

- Windows 11 Enterprise Insider Preview `10.0.26310`
- 12th Gen Intel Core i7-12800HX
- 24 logical processors
- 31.7 GiB memory
- NTFS
- `rustc 1.95.0 (59807616e 2026-04-14)`
- LLVM `22.1.2`
- `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`
- host `x86_64-pc-windows-msvc`

## Acquisition correction

The immutable source revision did not contain a `Cargo.lock`. Initial
`--locked` attempts exited `101` before producing compiler artifacts. The
corrected preparation was:

```powershell
cargo generate-lockfile --manifest-path <fixture>\Cargo.toml
cargo fetch --locked --manifest-path <fixture>\Cargo.toml
```

Measurement commands then used `--locked --offline` and isolated
`CARGO_TARGET_DIR` paths. The generated lockfile hash above is part of the
fixture identity.

## Commands

Representative stable evidence:

```powershell
cargo metadata --locked --offline --format-version 1 `
  --manifest-path <fixture>\Cargo.toml

cargo check --locked --offline `
  --manifest-path <fixture>\Cargo.toml `
  --target-dir <isolated-target> `
  --message-format=json-render-diagnostics

cargo check --locked --offline `
  --manifest-path <fixture>\Cargo.toml `
  --target-dir <isolated-target> `
  --message-format=json-render-diagnostics `
  --timings
```

Wall-clock samples used PowerShell `Diagnostics.Stopwatch` around the complete
Cargo process. Each mode wrote stdout and stderr to separate session artifacts.

## Results

### Cold and warm Cargo observations

- Cold JSON stream: 13 `compiler-artifact` messages and three
  `build-script-executed` messages.
- Warm no-op JSON stream: 13 artifacts with `fresh: true`; no compiler
  artifacts were rebuilt; the three build-script messages were replayed with
  cached output and therefore did not prove execution.
- Representative cold timing report: 16 dirty units because build-script
  compilation and execution are represented separately.
- Maximum observed Cargo concurrency: five, despite 24 available logical
  processors.
- Representative unit durations:
  - `syn`: about 3.9 seconds;
  - `proc-macro2` build-script run: about 2.35 seconds;
  - `thiserror` build-script run: about 2.52 seconds;
  - METIS-CORE: about 0.7 seconds.

### Warm no-op `--timings` calibration

| Mode | Samples | Median | MAD |
| --- | --- | --- | --- |
| Plain check | 3 | 582.67 ms | 17.93 ms |
| Check with `--timings` | 3 | 1,028.73 ms | 77.14 ms |

On this fixture, timing-report generation added about 446 ms. This is a local
calibration result, not a cross-platform Cargo overhead estimate.

### Human versus JSON message calibration

Five alternating samples were collected for each output mode on the warm
target. One large first-sample outlier occurred in the human-output series.

| Mode | Sorted samples | Median | MAD |
| --- | --- | --- | --- |
| Human | 516.98, 577.09, 610.88, 617.91, 4,922.80 ms | 610.88 ms | 33.79 ms |
| JSON | 437.75, 596.37, 611.26, 725.70, 1,026.20 ms | 611.26 ms | 114.44 ms |
| Metadata | 781.85, 794.10, 823.49, 932.33, 5,120.03 ms | 823.49 ms | 41.64 ms |

JSON-message overhead was not distinguishable from run variance in this small
sample. Metadata is a separate graph-discovery operation and should not be
silently charged to every benchmark repetition.

### Cold wall-clock samples

| Mode | Samples |
| --- | --- |
| Plain check | 10.72, 16.99, 18.59 seconds |
| Check with `--timings` | 11.13, 11.44, 14.45 seconds |

The samples were affected by order, machine state, and cache drift. They are
retained as evidence of variance but are not valid evidence that instrumentation
improves or worsens cold compilation.

## Interpretation

- Cargo JSON and Cargo timings answer different questions and must be joined.
- Build-script execution needs timing or other dirty-unit evidence because the
  JSON output message is replayed from cache.
- A small project's visible cold latency can be dominated by dependencies and
  build scripts rather than its own crate.
- Timing-report generation can materially perturb a sub-second no-op workload.
- Repeated uninstrumented runs should establish the primary latency
  distribution; diagnostic telemetry should be collected separately.

## Limitations

- One small public fixture.
- One Windows host.
- Three timing-calibration samples and five message-format samples.
- No randomized cold-run experiment.
- No nightly rustc self-profile measurement.
- No peak-memory or hardware-counter measurement.
