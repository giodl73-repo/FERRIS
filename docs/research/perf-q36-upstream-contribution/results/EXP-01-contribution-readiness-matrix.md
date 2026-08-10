# EXP-01: Upstream Contribution Readiness Matrix

Date: 2026-08-09
Question: PERF-Q36
Result: select the PERF-Q20 Relink-Don't-Rebuild edit matrix as FERRIUM's first
upstream contribution packet.

## Method

Candidate cases were scored against six gates:

1. a named upstream owner and active problem;
2. a minimized or readily minimizable fixture;
3. deterministic positive and negative controls;
4. reproducible local commands and exact environment identity;
5. a documented upstream artifact form; and
6. a bounded remaining effort before owner review.

This is a readiness comparison, not an upstream acceptance claim.

## Upstream artifact forms

| Upstream home | Contribution form | Required alignment |
|---|---|---|
| rustc-perf | Primary or Secondary compile benchmark, local timing comparison, registration, configuration, license, lockfile, README | rustc-perf owner and authorized perf-run support |
| rustc | Issue, compiler test, profile, PR, or MCP for larger changes | compiler owner or reviewer |
| Cargo | Accepted issue, Criterion benchmark, test, documentation, or PR | `S-accepted` issue and Cargo reviewer |
| rustc_codegen_cranelift | Integrated correctness/performance fixture or implementation experiment | backend and compiler owner |
| Linker project | Reproducer, object-identity evidence, configuration result, or patch | target linker owner |

Sources:

- <https://rustc-dev-guide.rust-lang.org/tests/perf.html>
- <https://rustc-dev-guide.rust-lang.org/contributing.html>
- <https://github.com/rust-lang/rustc-perf/blob/main/collector/compile-benchmarks/README.md>
- <https://github.com/rust-lang/cargo/blob/master/CONTRIBUTING.md>
- <https://github.com/rust-lang/cargo/blob/master/benches/README.md>

## Candidate matrix

| Candidate | Owner/problem | Fixture and controls | Upstream form | Remaining work | Readiness |
|---|---|---|---|---|---|
| PERF-Q20 RDR body-versus-interface edits | Rust Fast Builds/RDR, rustc-perf, Cargo RDR issue | Three-crate chain; 13 cases; incremental on/off; positive body candidates; inline, generic, const, macro, layout, identity, and interface controls | Secondary compile benchmark with incremental patches | Reproduce on Linux, reduce patch set with owners, run `bench_local`, prepare license/config registration | **Selected first** |
| PERF-Q32 `hint-mostly-unused` matrix | Cargo issue `#15644` | Sparse, dense, generic, private, multi-consumer, whole-crate-error, and public METIS controls | Concise issue evidence and possible Cargo/rustc test | Owner approval, issue-format reduction, current-nightly rerun | Ready second |
| PERF-Q31 function cache | Rust 2026 Cranelift goal | Direct cache timing, key perturbation, corruption, stable-function, CGU, and public METIS controls | Upstream-owned integration fixture | Sponsor, integrated rustc branch, capability equality, Linux and multi-platform evidence | Research ready |
| PERF-Q29 linker input stability | rustc plus lld/MSVC/linker owners | Complete versus incremental link, unchanged control, body edit, PDB/ILK and public repo evidence | Reproducer or linker/rustc issue | Stable object identity investigation and Unix controls | Partial |
| PERF-Q23 build-script precision | Cargo accepted issue paths | Hidden input, precise declaration, checksum, unchanged output, persistent output, native metadata | Cargo test/benchmark or accepted issue evidence | Public representative script and owner question | Partial |
| PERF-Q17/Q19 query and early-phase precision | rustc compiler queries | Synthetic topology and held-out edit controls | Secondary benchmark or compiler test | Choose one current issue and re-minimize around it | Queue |
| PERF-Q35 validation selection | External FERRIUM capability | Eight failure classes and public PARLOR control | External planner evidence, not rustc-perf | Held-out repository and maintainer usability study | Not an upstream compiler target |

## Why RDR is first

The RDR matrix has the strongest combination of:

- a named high-value upstream direction;
- one small dependency chain;
- reproducible current behavior;
- edit scenarios that map to incremental patched benchmarks;
- positive optimization candidates;
- mandatory miscompilation-sensitive controls;
- a clear non-goal: no downstream FERRIUM implementation; and
- a benchmark-shaped contribution that can be useful before the mechanism is
  complete.

The current packet is not yet submission-ready because the evidence was
collected on Windows MSVC and rustc-perf officially tracks Linux GNU compiler
performance. The required next sequence is:

1. ask RDR and rustc-perf owners whether the benchmark shape is useful;
2. reproduce the minimized chain on `x86_64-unknown-linux-gnu`;
3. translate the preferred edits into `N-*.patch` scenarios;
4. run the documented local `bench_local` comparison;
5. prepare the two-commit benchmark change, configuration, license, lockfile,
   and README update; and
6. request authorized upstream performance evaluation.

## Estimated conversion effort

The local RDR evidence already contains:

- one 143-line public experiment record;
- one three-package dependency chain;
- 13 edit cases;
- three exploratory repetitions per case;
- incremental-enabled and incremental-disabled controls;
- before/after metadata, library, executable, and runtime observations; and
- a 397-line decision and role review.

Remaining effort is bounded to adaptation rather than rediscovery:

| Work item | Output |
|---|---|
| Owner alignment | Agreed benchmark purpose and preferred patch set |
| Linux reproduction | Behavior and local stable-metric baseline on upstream target |
| Fixture reduction | Smallest case retaining body/interface distinction |
| rustc-perf adaptation | Benchmark directory, `perf-config.json`, patches, registration |
| Local comparison | Required check/debug/opt timing and `bench_local` results |
| Review packaging | Two commits, README, `REUSE.toml`, lockfile, PR description |
| Maintenance | Named FERRIUM owner and response/update commitment |

Human-hours are not claimed because the research sessions did not record
active effort consistently. Artifact count and remaining deliverables are the
auditable effort measure.

## Stop conditions

The first packet is deferred or redirected if:

- upstream owners do not find the benchmark useful;
- Linux reproduction does not preserve the distinguishing behavior;
- minimization removes the body-versus-interface distinction;
- the case duplicates an existing benchmark without adding scenario coverage;
- licensing or redistribution cannot be made explicit;
- local stable metrics show no interesting compiler work; or
- FERRIUM cannot commit to maintaining the accepted artifact.
