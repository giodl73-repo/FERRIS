# EXP-01: Editor, Cargo, and Target-Directory Contention

Date: 2026-08-08
Question: PERF-Q07
Status: Complete

## Purpose

Separate rust-analyzer's internal semantic work, its Cargo build-data work,
check-on-save, Cargo lock waiting, incompatible command artifacts, and
target-directory duplication.

The experiment asks:

1. What work does rust-analyzer perform before flycheck?
2. Do concurrent identical checks duplicate compilation?
3. What changes when editor check overlaps an artifact-producing build?
4. Does a separate target directory eliminate contention, and what does it
   duplicate?

The synthetic timings are exploratory with three repetitions. An injected
rustc delay makes overlap deterministic enough to inspect lock topology; the
absolute durations are not representative compiler benchmarks.

## Environment

- Windows 11 Enterprise Insider Preview, build 26310;
- Intel Core i7-12800HX, 16 cores and 24 logical processors;
- 32 GiB memory;
- Cargo `1.95.0 (f2d3ce0bd 2026-03-21)`;
- rustc `1.95.0 (59807616e 2026-04-14)`;
- rust-analyzer `1.95.0 (59807616 2026-04-14)`;
- host `x86_64-pc-windows-msvc`;
- LLVM `22.1.2`;
- `CARGO_INCREMENTAL=0`;
- isolated disposable fixture and target directories.

## Synthetic fixture

The four-package workspace contains:

- a binary with a build script;
- one ordinary library;
- one leaf helper library;
- one procedural-macro crate.

The build script sleeps for 750 ms and writes one generated constant. A
temporary `RUSTC_WRAPPER` records each rustc process and adds a 750 ms delay to
invocations containing `--crate-name`.

All Cargo commands used:

```powershell
cargo check --workspace --all-targets --locked --offline
cargo build --workspace --locked --offline
```

Each scenario started from empty target directories. The second concurrent
command started 250 ms after the first.

## Concurrent identical checks

| Topology | Makespan samples (ms) | Median | MAD | Successful rustc processes |
| --- | --- | ---: | ---: | --- |
| One cold check | 6,006 / 5,323 / 5,348 | 5,348 | 25 | 11 |
| Two checks, shared target | 5,411 / 5,374 / 5,205 | 5,374 | 37 | winner 11, waiter 1 |
| Two checks, separate targets | 5,641 / 5,844 / 5,469 | 5,641 | 172 | 11 plus 11 |

With a shared target, one command performed the cold check and the other
reported:

```text
Blocking waiting for file lock on build directory
Finished `dev` profile ... in 4.94s
```

The waiter executed only rustc's `___` capability probe. No crate compilation
was repeated in these three runs. Which process became the producer was not
determined by the 250 ms launch order in every sample.

The shared target contained 4,444,339 bytes after the final run.

Separate targets removed the build-directory wait but performed 22 rustc
processes and retained 8,905,310 bytes across the two directories. Both
commands also briefly reported waiting for the global package-cache lock, so
target isolation did not remove every Cargo lock.

## Editor check followed by terminal build

| Topology | Makespan samples (ms) | Median | MAD | Check rustc | Build rustc |
| --- | --- | ---: | ---: | ---: | ---: |
| One cold build | 6,590 / 8,521 / 5,488 | 6,590 | 1,102 | N/A | 6 |
| Shared target | 8,113 / 8,172 / 8,316 | 8,172 | 58 | 11 | 5 |
| Separate targets | 5,641 / 5,629 / 5,696 | 5,641 | 12 | 11 | 6 |

The cold-build series was unstable and does not support a promoted build-time
comparison.

For the shared target, the build reported:

```text
Blocking waiting for file lock on artifact directory
```

It reused one compatible compile-time unit from check, then compiled five
artifact-producing units. The target retained 10,695,527 bytes.

Separate targets let both commands progress concurrently. The build compiled
all six of its units, and the two directories retained 13,674,214 bytes.

This control demonstrates the trade:

- shared target: less work and storage, but delayed terminal build;
- separate target: lower foreground makespan on this high-core fixture, but
  full duplicated check/build state and concurrent resource demand.

No CPU-utilization claim is made because CPU and power counters were not
captured.

## rust-analyzer project loading

The batch command was:

```powershell
rust-analyzer analysis-stats <fixture> --output csv
```

The negative control added:

```text
--disable-build-scripts --disable-proc-macros
```

### Synthetic project

| Mode | Wall samples (ms) | Median | MAD | Cargo target bytes |
| --- | --- | ---: | ---: | ---: |
| Build data enabled | 60,248 / 11,157 / 10,901 | 11,157 | 256 | 5,745,356 |
| Build data disabled | 8,216 / 8,055 / 8,654 | 8,216 | 161 | 0 |

The first build-data run was a toolchain and process warm-up outlier.

rust-analyzer logged:

```text
cargo check --quiet --workspace --message-format=json \
  --keep-going --compile-time-deps --all-targets
```

The command produced the build-script `OUT_DIR` and the procedural-macro
dynamic library. The rust-analyzer source documents that it acts as
`RUSTC_WRAPPER` for this command so only build scripts and procedural macros
are compiled while ordinary crate checking is skipped.

After project loading, rust-analyzer separately analyzed approximately 986,000
dependency lines and reported about 561 MB at completion. Disabling build data
did not remove that semantic database work. It instead produced unknown types
for generated input.

### Public fixtures

| Fixture | Revision | Build-data wall | Cargo build-data time | RA analysis total | Target bytes |
| --- | --- | ---: | ---: | ---: | ---: |
| METIS-CORE | `78ae34090e043e79a206f2daffaa3889389b4790` | 80.620 s | 21.38 s | 13.92 s | 90,927,709 |
| RUNE | `194449444624fb10add4137cb0da8d0327164fa7` | 28.066 s | 13.92 s | 10.23 s | 52,548,177 |

Lockfile SHA-256:

- generated METIS-CORE lock:
  `1f4fd21fe5fb1ffd141cc6fe5956e71837159c5b5b4309b30c34c0d9f3577cdd`;
- committed RUNE lock:
  `802bc3dfcf7099e93fa2b65ffbb686a2e3a7136a340a5ae9b2eb41a0670e7a56`.

Single negative-control runs with build data disabled took 16.671 seconds for
METIS-CORE and 11.798 seconds for RUNE and wrote no Cargo target artifacts.
These one-run public values are diagnostics, not promoted comparative claims.

METIS-CORE's rust-analyzer summary attributed 1.99 seconds to metadata, 21.38
seconds to Cargo build data, and 13.92 seconds to later analysis. Its 80.620
second wall time therefore retained a large unclassified project-loading
interval. The experiment does not infer whether proc-macro loading, file
loading, operating-system interference, or another internal phase caused it.

The public analysis reported:

- METIS-CORE: about 1.95 million dependency lines and 1.3 GB at completion;
- RUNE: about 1.56 million dependency lines and 933 MB at completion.

Target-directory tuning cannot remove this independent in-memory semantic
work.

## LSP save trace

A minimal LSP client initialized rust-analyzer with:

- check-on-save enabled;
- workspace and all-target checking;
- cache priming disabled;
- build-script and proc-macro loading disabled for isolation;
- either the workspace target or `target-ra`.

Startup and the saved edit each caused flycheck to be spawned. rust-analyzer
source gives restart messages priority, cancels the previous process, and
debounces new restart requests for 50 ms.

Both shared and isolated traces completed eleven rustc processes with no
incomplete wrapper record. The isolated check retained 4,473,617 bytes. The
trace confirms restart and cancellation behavior, but does not demonstrate
wasted completed compilation in this small fixture.

Disabling build data caused rust-analyzer diagnostics for:

```text
`OUT_DIR` not set, build scripts may have failed to run
proc-macro expansion is disabled
```

That is the correctness cost of the negative control, not an acceptable
general optimization.

## Limitations

- The rustc delays are synthetic and favor visible overlap.
- CPU, memory bandwidth, disk queue, and power counters were not sampled.
- Concurrent commands had no registry dependencies, but still touched Cargo's
  global package-cache lock.
- The shared-target tests did not reproduce Cargo's documented unresolved case
  where two dirty checks can rebuild a shared unit after waiting.
- Public rust-analyzer measurements used one diagnostic run per mode.
- `analysis-stats` is an unstable rust-analyzer subcommand and is retained as
  diagnostic evidence, not a product interface.
- The open rust-analyzer issue about a large-workspace stall with a custom
  target directory has no confirmed root cause or resolution.
- Linux and macOS behavior was not measured.

## Retained evidence

The private experiment record retains:

- fixture source and generated lockfiles;
- exact rustc-wrapper start and finish records;
- Cargo stdout, stderr, durations, exit status, target sizes, medians, and MAD;
- rust-analyzer project-loading logs and summaries;
- LSP initialization, flycheck lifecycle, diagnostics, and wrapper records.

No upstream issue, comment, branch, or pull request was created.
