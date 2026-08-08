# rustc Startup and Metadata Loading

Date: 2026-08-08
Question: PERF-Q08
Status: Complete
Decision: add rustc invocation-floor and metadata-demand evidence to the
FERRIUM model; retain daemon or batching design as upstream research rather
than a FERRIUM implementation.

## Executive conclusion

Fixed per-invocation rustc cost is material for tiny crates, but “startup” is
not one measurable phase.

On the warmed Windows control:

- direct rustc `--version` had a 54.04 ms median;
- crate-root parsing on nightly had a 57.76 ms median;
- a tiny stable metadata-only std crate had an 84.30 ms median;
- a tiny stable rlib had a 99.84 ms median.

This places executable loading and early command handling near the majority of
a tiny metadata-only invocation's wall time. It does not mean 54 ms is
recoverable: the version path is only a lower boundary, and the compiler
process is also an isolation, parallel scheduling, diagnostics, and failure
boundary.

Metadata cost depends more on demand shape and crate count than file size
alone. Passing an unused 10.95 MB `.rmeta` changed little relative to the
repeat control. Referencing one exported item increased work; glob-reexporting
the dependency increased it further. Moving from one to 32 referenced tiny
dependencies added about 23 ms despite only about 65 KB of generated metadata.

This matches rustc's architecture. Crate metadata is stored through lazy
values, arrays, and random-access tables. The compiler locates the transitive
crate universe, registers the crates needed by the session, and decodes
specific metadata as queries demand it.

The standard library also contributes fixed metadata work. A diagnostic
self-profile registered 13 crates for the tiny std input and two for the
`no_std` input. The repeated stable medians differed by about 4.5 ms.

Current upstream direction does not include a persistent rustc daemon. The
Relink-Don't-Rebuild design explicitly describes rustc's current architecture
as one new process per crate and therefore seeks to skip unnecessary
invocations. rustc-perf already contains `helloworld` as a compile-time lower
bound and `large-workspace` as a metadata-loading stress test, but the
documented suite does not decompose process, session, sysroot registration,
metadata demand, and output mode into one startup-specific fixture.

FERRIUM should add that decomposition to its read-only build intelligence and
produce reusable fixtures. It should not build a compiler daemon now.

No upstream activity was created.

## Decision supported

This research determines:

- whether fixed rustc invocation cost is material enough to preserve as a
  separate component;
- how dependency count, metadata size, and metadata demand differ;
- what self-profile can show and what remains outside its event surface;
- whether Cargo pays the rustup terminal proxy for each rustc process;
- whether FERRIUM should prototype batching or contribute measurement evidence.

It does not authorize a rustc wrapper, daemon, compiler fork, crate merging,
sysroot replacement, rustup bypass, or upstream filing.

## Evidence reviewed

### Local evidence

- [First seven performance questions](2026-08-08-first-seven-performance-questions.md)
- [Rust latency telemetry](2026-08-07-rust-latency-telemetry.md)
- [Cargo graph scheduling](2026-08-08-cargo-graph-scheduling.md)
- [Cargo build-unit multiplication](2026-08-08-cargo-build-unit-multiplication.md)
- [Editor and Cargo contention](2026-08-08-editor-cargo-contention.md)
- [Experiment](perf-q08-rustc-startup-metadata/results/EXP-01-invocation-floor-and-metadata.md)

### Compiler and metadata sources

Nightly source revision:
`1a98b1e135b254f209c67d447b6d8bcd56a859e0`.

- [`rustc_driver_impl::run_compiler`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_driver_impl/src/lib.rs)
- [`rustc_metadata::rmeta`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_metadata/src/rmeta/mod.rs)
- [metadata decoder](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_metadata/src/rmeta/decoder.rs)
- [crate reader](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_metadata/src/creader.rs)
- [crate locator](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_metadata/src/locator.rs)
- [rustc-dev-guide metadata chapter](https://rustc-dev-guide.rust-lang.org/backend/libs-and-metadata.html)
- [rustc-dev-guide profiling chapter](https://rustc-dev-guide.rust-lang.org/profiling.html)

### Upstream performance direction

- [2026 Fast Builds roadmap](https://github.com/rust-lang/rust-project-goals/blob/44ee4db7e818e4ffc7b9f7faadb90316fc0a56f2/src/2026/roadmap-fast-builds.md)
- [Relink-Don't-Rebuild](https://github.com/rust-lang/rust-project-goals/blob/44ee4db7e818e4ffc7b9f7faadb90316fc0a56f2/src/2025h2/relink-dont-rebuild.md)
- [rustc-perf compile benchmark inventory](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/README.md)
- [measureme `summarize`](https://github.com/rust-lang/measureme/blob/9aeaf4d0cfaafb13379f0d8f3c3b5e8c13071afc/summarize/README.md)
- [2025 compiler performance survey](https://blog.rust-lang.org/2025/09/10/rust-compiler-performance-survey-2025-results/)

## Invocation model

```text
launcher or Cargo
  -> operating-system process and image load
  -> rustc argument and option handling
  -> target, sysroot, backend, diagnostics, and session construction
  -> crate-root parse
  -> crate discovery and metadata registration
  -> expansion and resolution
  -> demand-driven metadata decoding and semantic queries
  -> metadata, object, archive, or link output
  -> profiler flush, diagnostics flush, and process exit
```

The layers overlap. A version command does not exercise the complete compile
session, and self-profile does not make every layer visible.

## Findings

### FERRIUM-98: fixed invocation cost is material for tiny crates

**Source**

- [Experiment](perf-q08-rustc-startup-metadata/results/EXP-01-invocation-floor-and-metadata.md)

**Observed behavior**

The direct compiler version path had a 54.04 ms median. The tiny stable
metadata-only compilation had an 84.30 ms median, and the identical repeat
control had an 85.78 ms median.

Nightly phase boundaries measured 53.70 ms for version, 57.76 ms through
crate-root parsing, 71.10 ms through expansion, and 80.77 ms for metadata
output.

**Implication**

Small-crate and high-crate-count workloads need an explicit per-invocation
component. However, the version path is a lower bound, not a claim that all of
its time is avoidable process startup.

**Confidence:** high for the warmed Windows fixture; low for cross-platform
magnitude.

### FERRIUM-99: the rustup proxy and Cargo's rustc process are different

**Source**

- [Experiment](perf-q08-rustc-startup-metadata/results/EXP-01-invocation-floor-and-metadata.md)

**Observed behavior**

Launching `rustc --version` through the rustup proxy had a 94.44 ms median,
about 40 ms above the direct toolchain binary.

A temporary `RUSTC_WRAPPER` showed that Cargo passed the direct toolchain rustc
path to the wrapper, not the terminal proxy.

**Implication**

FERRIUM must record the actual compiler executable. It must not multiply
terminal-proxy overhead by Cargo's unit count or recommend bypassing rustup as
a Cargo build optimization.

**Confidence:** high for the installed Windows rustup and Cargo versions.

### FERRIUM-100: the sysroot is fixed metadata work, not zero-cost background

**Sources**

- [Experiment](perf-q08-rustc-startup-metadata/results/EXP-01-invocation-floor-and-metadata.md)
- [crate locator](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_metadata/src/locator.rs)

**Observed behavior**

The stable std metadata median was 84.30 ms versus 79.84 ms for `no_std`.

In separate nightly self-profiles, the std input registered 13 crates and spent
6.08 ms of profiled CPU self time in `metadata_register_crate`. The `no_std`
input registered two crates and spent 0.13 ms in that event.

**Implication**

A nominally dependency-free crate still has a sysroot metadata topology.
Reports must distinguish user dependencies from injected and sysroot crates.
`no_std` is a diagnostic control, not a general speed recommendation.

**Confidence:** high for crate counts and local medians; medium for direct
attribution because the profiles were separate one-run diagnostics.

### FERRIUM-101: metadata decoding is demand-shaped rather than file-size
proportional

**Sources**

- [rmeta lazy structures](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_metadata/src/rmeta/mod.rs)
- [metadata decoder](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_metadata/src/rmeta/decoder.rs)
- [Experiment](perf-q08-rustc-startup-metadata/results/EXP-01-invocation-floor-and-metadata.md)

**Observed behavior**

rustc metadata uses lazy values, arrays, and random-access tables.

The generated 10.95 MB dependency had these stable medians:

- passed through `--extern` but unused: 88.32 ms;
- one exported item referenced: 101.05 ms;
- public glob reexport: 128.29 ms.

The unused result remained close to the two std controls. Broader semantic
demand increased metadata decode, expansion, resolution, and output work.

**Implication**

Metadata bytes alone are not a latency estimate. FERRIUM needs crate
registration, accessed metadata families, exported namespace shape, and query
demand.

**Confidence:** high for the fixture and lazy source architecture.

### FERRIUM-102: dependency count creates fixed work even when bytes are tiny

**Sources**

- [crate locator](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_metadata/src/locator.rs)
- [Experiment](perf-q08-rustc-startup-metadata/results/EXP-01-invocation-floor-and-metadata.md)

**Observed behavior**

Consumers referencing 1, 8, and 32 tiny dependencies had medians of 83.92,
93.20, and 107.20 ms. The 32 generated `.rmeta` files totaled only about
65 KB.

The crate locator explains why transitive metadata must be discoverable:
public types from a direct dependency may expose types from its dependencies.

**Implication**

Crate count, dependency depth, and registration are startup dimensions
independent of source lines and metadata bytes. This evidence informs, but
does not decide, PERF-Q34 modularization.

**Confidence:** high for the controlled fixture.

### FERRIUM-103: output mode matters after the fixed floor

**Source**

- [Experiment](perf-q08-rustc-startup-metadata/results/EXP-01-invocation-floor-and-metadata.md)

**Observed behavior**

The tiny std metadata-only median was 84.30 ms. Producing an rlib increased the
median to 99.84 ms.

**Implication**

For tiny crates, even small codegen and archive work sits on top of a larger
fixed invocation floor. The result cannot be generalized to executable
linking, debug information, optimized codegen, or larger crates.

**Confidence:** high for the control; low beyond metadata versus tiny rlib.

### FERRIUM-104: self-profile is rich but not an end-to-end startup clock

**Sources**

- [rustc profiling guide](https://rustc-dev-guide.rust-lang.org/profiling.html)
- [`rustc_driver_impl::run_compiler`](https://github.com/rust-lang/rust/blob/1a98b1e135b254f209c67d447b6d8bcd56a859e0/compiler/rustc_driver_impl/src/lib.rs)
- [Experiment](perf-q08-rustc-startup-metadata/results/EXP-01-invocation-floor-and-metadata.md)

**Observed behavior**

Self-profile exposed `metadata_register_crate`, metadata decode families,
expansion, resolution, analysis, and metadata writing.

The external nightly parse-root median was 57.76 ms. Its separate self-profile
contained 1.14 ms of profiled CPU events. The difference includes process,
early compiler, uninstrumented, profiler, output, and operating-system work and
cannot be assigned to one cause.

Adding query arguments to the profiler also changed metadata demand enough to
invalidate the pilot.

**Implication**

FERRIUM should join external process wall time with self-profile events and
retain an explicit unclassified interval. Profiling detail is not complete
wall-time attribution.

**Confidence:** high that the attribution gap and observer effect exist; low
on the composition of the unclassified interval.

### FERRIUM-105: one rustc process per crate remains an upstream architectural
boundary

**Sources**

- [Relink-Don't-Rebuild](https://github.com/rust-lang/rust-project-goals/blob/44ee4db7e818e4ffc7b9f7faadb90316fc0a56f2/src/2025h2/relink-dont-rebuild.md)
- [2026 Fast Builds roadmap](https://github.com/rust-lang/rust-project-goals/blob/44ee4db7e818e4ffc7b9f7faadb90316fc0a56f2/src/2026/roadmap-fast-builds.md)

**Observed constraint**

The RDR design explicitly describes current rustc as one process invocation per
crate, with a new process for each compile rather than a daemon. RDR therefore
seeks to skip unnecessary downstream invocations.

The 2026 roadmap prioritizes RDR, linking, alternative backends, frontend
parallelism, shared compilation-mode artifacts, and targeted hot-path work. It
does not select a persistent compiler daemon.

**Implication**

Avoided invocations are a real performance mechanism. A daemon remains a
compiler-lifecycle research question, not an approved FERRIUM implementation.

**Confidence:** high for the documented architecture and roadmap.

### FERRIUM-106: tiny-invocation experiments have configuration footguns

**Sources**

- `rustc -C help`
- [Experiment](perf-q08-rustc-startup-metadata/results/EXP-01-invocation-floor-and-metadata.md)

**Observed behavior**

`-C incremental` accepts a directory value. The pilot value `off` created an
incremental directory named `off` and added persistence work. Likewise,
self-profile query arguments caused additional metadata decoding for query-key
formatting.

Both pilots were discarded and rerun.

**Implication**

Tiny compiler benchmarks must record exact flags, created directories, profile
event sets, and output files. “Incremental disabled” and “profiling enabled”
are not adequate descriptions.

**Confidence:** high.

### FERRIUM-107: the defensible opportunity is invocation and metadata
diagnosis

**Sources**

- Findings FERRIUM-98 through FERRIUM-106
- [rustc-perf benchmark inventory](https://github.com/rust-lang/rustc-perf/blob/58b05b9a296dfb148aa54c9ec61e8890c65a4223/collector/compile-benchmarks/README.md)

**Observed constraint**

rustc-perf already includes:

- `helloworld` as a lower bound on compile time;
- `large-workspace` as a stress test for searching and loading hundreds of
  dependency metadata files.

The documented inventory does not decompose launcher, crate-root boundary,
sysroot registration, dependency count, lazy demand shape, metadata output,
and rlib output in one startup-specific control.

**Implication**

FERRIUM can provide a portable fixture and joined report without duplicating
the compiler or performance dashboard. Any upstream contribution should start
with fixture or telemetry evidence, not a daemon proposal.

**Confidence:** high for the missing decomposition; medium for upstream
acceptance until maintainers are consulted with owner approval.

## Recommendations

### Adopt now

- Record actual rustc executable, launcher, process count, target, sysroot,
  crate type, output mode, incremental state, and profiler event set.
- Report an invocation lower bound separately from parse, expansion, analysis,
  metadata registration and decoding, codegen, and linking.
- Count sysroot, injected, direct, and transitive metadata registrations
  separately.
- Preserve metadata demand shape: unused extern, named item, glob/export
  enumeration, trait or implementation lookup, macro, generic, and MIR demand.
- Keep external wall time primary and retain unclassified intervals.

### Prototype behind a compatibility boundary

- A read-only rustc invocation census joined to Cargo units.
- A startup-boundary fixture spanning version, parse-only, expansion-only,
  metadata-only, rlib, dependency count, and metadata demand.
- A self-profile summarizer that labels process-external and unclassified wall
  time rather than forcing totals to add up.
- A rustc-perf-compatible minimized fixture proposal after owner approval.
- Narrow upstream telemetry proposals for early compiler and metadata
  registration or location events.

The implementation gate remains closed.

### Reject or defer

- Reject metadata file size as a standalone startup estimate.
- Reject `no_std`, crate merging, or dependency removal as automatic startup
  optimizations.
- Reject bypassing rustup for Cargo; Cargo already used the direct compiler.
- Reject inferring cold-start or cross-platform behavior from warm Windows
  measurements.
- Reject summing Cargo unit wall times as serialized fixed overhead because
  rustc processes overlap.
- Defer persistent compiler, daemon, batching, shared in-memory metadata, and
  compiler-lifecycle design.
- Defer crate modularization decisions to PERF-Q34.
- Defer upstream activity until explicit owner approval.

## Potential contribution paths

Without creating upstream activity, Q08 identifies:

1. a rustc-perf startup decomposition fixture adjacent to `helloworld` and
   `large-workspace`;
2. explicit early-session, crate-location, metadata-registration, and metadata
   blob-load events;
3. a cross-platform protocol for warm and cold invocation floors;
4. documentation preventing `-C incremental=off` benchmark contamination;
5. minimized regressions where dependency count or metadata demand grows
   disproportionately.

Each path first requires a current-version reproduction, Linux and macOS
coverage where relevant, and project-specific maintainer guidance.

## Role review

| Role | Disposition |
| --- | --- |
| Rust Safety Steward | Accepted: sysroot and dependency metadata remain correctness inputs; no semantic work is removed or bypassed. |
| Compiler Performance Engineer | Accepted: launcher, warm state, boundaries, metadata demand, output mode, wall time, diagnostic CPU events, observer effects, variance, and limitations remain separate. |
| Interop Boundary Auditor | Accepted: target, sysroot, backend, platform loader, native process, proc-macro, and linker boundaries remain explicit. |
| AI Assurance Skeptic | Accepted: the incremental-flag and query-argument pilots were rejected, the unclassified interval remains unknown, and single profiles are not promoted as distributions. |
| Ecosystem Strategist | Accepted: rustc, Cargo, rustup, rustc-perf, and measureme remain owners; FERRIUM supplies a missing decomposition and evidence. |
| Rust Maintainer | Accepted: ordinary Cargo and rustup usage remains valid; no wrapper, daemon, compiler ritual, or source rewrite is prescribed. |
| Native Platform Adopter | Accepted: Windows behavior, warm-cache scope, launcher choice, rollback, and missing Linux/macOS evidence are explicit. |
| Scope Keeper | Accepted: Q08 ends at invocation and metadata boundaries; frontend phases, modularization, incrementality, and daemon design remain later questions. |
| Validation Checker | Accepted: exact toolchains, commands, compiler paths, generated metadata sizes, 30- and 15-sample distributions, medians, MAD, negative pilots, profiles, and source revisions are recorded. |

No role raised a blocking objection. Role approval does not open the
implementation gate.

## Decision

PERF-Q08 is complete.

FERRIUM should preserve fixed rustc invocation and metadata demand as first
class latency components, using portable direct-rustc fixtures and joined
external/profile evidence. The next question is PERF-Q09: determine whether
parsing and tokenization create material reusable or parallel work after the
invocation floor is removed.
