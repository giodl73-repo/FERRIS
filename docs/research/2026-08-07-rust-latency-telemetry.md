# Rust Latency Telemetry

Date: 2026-08-07  
Question: PERF-Q01  
Status: Complete  
Decision: define the minimum evidence stack FERRIUM will require before it
explains, compares, or attempts to improve Rust build latency.

## Executive conclusion

No single Rust telemetry surface is sufficient.

FERRIUM should use a layered evidence stack:

1. repeat uninstrumented wall-clock runs for the primary latency claim;
2. use `cargo metadata` for the declared package, target, feature, and dependency
   graph;
3. use Cargo JSON messages for observed artifacts, freshness, cached or current
   build-script output, diagnostics, and final success;
4. use one or more separately labelled `cargo --timings` diagnostic runs for
   Cargo-unit duration, dependency unblocking, concurrency, and the coarse
   frontend/codegen split;
5. use rustc self-profile only when compiler-query, cache-hit, or compiler
   blocked-time evidence is required;
6. use rustc-perf when evaluating compiler changes or claims intended for
   upstream Rust.

The primary comparison must not silently include instrumentation whose cost is
large relative to the workload. On the small warm no-op fixture used here,
`--timings` added a large fixed cost while JSON message output did not show a
distinguishable median cost. Instrumented diagnostic runs and wall-clock
benchmark runs therefore need to remain separate evidence classes.

## Decision supported

This research freezes the telemetry foundation required by:

- the build-latency measurement contract;
- all later PERF questions that make latency or reuse claims;
- the Observe stage of the build-intelligence research program;
- future product boundaries between stable Cargo integration and optional
  nightly compiler diagnostics.

It does not select a product architecture, propose a rustc fork, or claim that
the exploratory METIS-CORE timings generalize to larger workspaces.

## Evidence reviewed

### Local doctrine

- `docs/specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md`
- `docs/plans/BUILD_INTELLIGENCE_RESEARCH_PROGRAM.md`
- `docs/research/2026-08-07-rust-incremental-reuse-boundaries.md`
- `docs/research/questions/PERF-Q01-measurement-telemetry.md`

### External sources

- Cargo metadata command:
  <https://doc.rust-lang.org/cargo/commands/cargo-metadata.html>
- Cargo external-tools and JSON-message contract:
  <https://doc.rust-lang.org/cargo/reference/external-tools.html>
- Cargo build timings:
  <https://doc.rust-lang.org/cargo/reference/timings.html>
- rustc self-profile:
  <https://rustc-dev-guide.rust-lang.org/profiling.html>
- measureme profiler data and tools:
  <https://github.com/rust-lang/measureme>
- rustc-perf collector:
  <https://github.com/rust-lang/rustc-perf/tree/master/collector>
- rustc performance-triad measurement guidance:
  <https://rustc-dev-guide.rust-lang.org/compiler-performance.html>

## Telemetry coverage matrix

| Evidence need | Wall clock | Metadata | Cargo JSON | Cargo timings | rustc self-profile | rustc-perf |
| --- | --- | --- | --- | --- | --- | --- |
| End-to-end elapsed time | Primary | No | No | Diagnostic total | Compiler only | Yes |
| Package/dependency graph | No | Yes | Partial observed units | Observed units | No | Benchmark config |
| Target/profile/features | Command context | Yes | Yes | Yes | Invocation context | Yes |
| Artifact freshness | No | No | Yes | Dirty units shown | Query cache evidence | Scenario dependent |
| Build-script outputs | No | No | Yes; not proof of execution | Run units and duration | Partial compiler view | Scenario dependent |
| Unit duration | No | No | No | Yes | Compiler events | Yes |
| Dependency unblocking | No | No | No | Yes | No | No |
| Cargo concurrency | No | No | No | Yes | No | No |
| Frontend/codegen split | No | No | No | Coarse | Detailed events | Yes |
| Query/event self time | No | No | No | No | Yes | With self-profile |
| Query cache hits | No | No | No | No | Yes | With self-profile |
| Incremental load time | No | No | No | No | Yes | With self-profile |
| Linking | Included only | No | Artifact identity | Unit duration when visible | Backend events | Yes |
| Peak memory | External sampler | No | No | No | Not the primary surface | Yes |
| Hardware instructions/cycles | External counter | No | No | No | No | Yes |
| Validation behavior | Command context | Graph context | Observed artifacts | Observed units | Compiler only | Configured scenarios |

## Findings

### FERRIUM-35: stable Cargo telemetry is complementary, not cumulative

**Sources**

- Cargo metadata command and versioned output:
  <https://doc.rust-lang.org/cargo/commands/cargo-metadata.html>
- Cargo JSON messages:
  <https://doc.rust-lang.org/cargo/reference/external-tools.html>
- Cargo timings:
  <https://doc.rust-lang.org/cargo/reference/timings.html>

**Observed constraint**

`cargo metadata --format-version 1` describes packages, targets, resolved
features, and dependencies. Cargo JSON messages describe what the invoked build
observed, including compiler artifacts, `fresh`, build-script output, compiler
messages, and final success. Cargo timings describe compiler invocations as
units, their duration, concurrency, dependency unblocking, and a coarse
frontend/codegen split.

None of these stable Cargo surfaces reports compiler query execution, query
cache hits, or the semantic reason a unit was invalidated.

**Implication**

FERRIUM must join the surfaces using recorded package, target, profile,
features, command, target directory, toolchain, and fixture identity. It must
not present any one Cargo output as a complete causal trace.

**Confidence:** high.

### FERRIUM-36: Cargo JSON is the stable observed-build ledger, not proof that a
build script ran

**Sources**

- Cargo JSON-message schema:
  <https://doc.rust-lang.org/cargo/reference/external-tools.html>
- Experiment:
  `docs/research/perf-q01-telemetry/results/EXP-01-stable-cargo-surfaces.md`

**Observed behavior**

The cold METIS-CORE check emitted 13 `compiler-artifact` messages and three
`build-script-executed` messages. A warm no-op check emitted the same 13
artifacts with `fresh: true` and repeated the three build-script messages.
Cargo's documented contract says `build-script-executed` is emitted even when
the script did not run and contains the previously cached output.

The JSON stream did not supply per-unit duration.

**Implication**

Cargo JSON should be retained as FERRIUM's stable record of observed artifacts,
freshness, build-script outputs, diagnostics, and outcome. A
`build-script-executed` message must not be presented as proof of execution.
Duration and actual dirty run-unit evidence must come from wall-clock or timing
evidence, not be inferred from message order.

**Confidence:** high.

### FERRIUM-37: Cargo timings explain the Cargo schedule but are a diagnostic
instrument

**Sources**

- Cargo timings:
  <https://doc.rust-lang.org/cargo/reference/timings.html>
- Experiment:
  `docs/research/perf-q01-telemetry/results/EXP-01-stable-cargo-surfaces.md`

**Observed behavior**

The representative cold timing report contained 16 dirty units: the 13
compiler artifacts plus three build-script run units. It showed a maximum
concurrency of five on a 24-logical-processor machine because the dependency
graph constrained available work. It also showed dependency/build-script work
dominating this small fixture: `syn` took about 3.9 seconds, while the
METIS-CORE crate itself took about 0.7 seconds in `cargo check`.

Stable Cargo emits an HTML timing report. The useful embedded report data is
not documented as a versioned machine interface equivalent to Cargo metadata
or the JSON-message schema. The report also cannot see compiler-internal
parallelism or query causes.

**Implication**

Use Cargo timings to diagnose unit duration, schedule, and critical-path
opportunity. Do not make unsupported HTML internals the sole durable product
contract, and do not describe the report as a compiler-level causal trace.

**Confidence:** high for the coverage boundary; medium for long-term output
compatibility because the stable contract is the report, not a documented
structured schema.

### FERRIUM-38: instrumentation must be separated from the primary benchmark

**Source**

- Experiment:
  `docs/research/perf-q01-telemetry/results/EXP-01-stable-cargo-surfaces.md`

**Observed behavior**

On the warm no-op fixture:

- plain check median: 582.67 ms, MAD 17.93 ms;
- check with `--timings` median: 1,028.73 ms, MAD 77.14 ms;
- human output median: 610.88 ms, MAD 33.79 ms;
- Cargo JSON output median: 611.26 ms, MAD 114.44 ms.

The timing report added about 446 ms to a sub-second no-op check in this
environment. No JSON-message overhead was distinguishable from run variance.
Cold comparisons were too confounded by ordering, machine state, and cache
effects to support an overhead claim.

**Implication**

The primary latency distribution must come from minimally instrumented runs.
Timing and self-profile runs are separately labelled diagnostics. Every new
telemetry mode must have a calibration run before FERRIUM uses its elapsed time
as product evidence.

**Confidence:** high for this fixture; low for generalizing the numeric overhead
to other projects or operating systems.

### FERRIUM-39: rustc self-profile is the query-granularity escalation path

**Sources**

- rustc self-profile:
  <https://rustc-dev-guide.rust-lang.org/profiling.html>
- measureme:
  <https://github.com/rust-lang/measureme>

**Observed constraint**

`-Zself-profile` records compiler events that can be summarized by tools such
as `summarize`, including event counts, self time, cache hits, blocked time,
incremental loading, and compiler/backend categories. It is a nightly-only
rustc interface and produces measureme profiler data rather than a stable Cargo
message.

Nightly was not installed in the experiment environment, so this pass did not
measure self-profile overhead.

**Implication**

Self-profile is required when a PERF question needs query or compiler-cache
evidence, but it must remain an opt-in compatibility boundary. FERRIUM's stable
baseline and product operation must still work without nightly.

**Confidence:** high for capability and channel status; self-profile overhead
remains unmeasured.

### FERRIUM-40: rustc-perf is the upstream compiler-change evidence layer

**Sources**

- rustc-perf collector:
  <https://github.com/rust-lang/rustc-perf/tree/master/collector>
- compiler performance guidance:
  <https://rustc-dev-guide.rust-lang.org/compiler-performance.html>

**Observed constraint**

rustc-perf supports controlled full and incremental scenarios, multiple
profiles and backends, self-profile collection, wall time, peak resident
memory, instructions, and cycles. The Rust compiler performance guidance
prefers less noisy hardware metrics such as instruction counts when practical
for regression tracking.

It is designed to compare compiler behavior across a benchmark suite. It does
not replace repository-specific Cargo graph and freshness evidence.

**Implication**

Any FERRIUM intervention intended for rustc or Cargo upstream should graduate
from local fixtures to a rustc-perf-compatible experiment. Repository build
explanation should continue to use the Cargo layers.

**Confidence:** high.

### FERRIUM-41: reproducible acquisition is part of telemetry correctness

**Source**

- Experiment:
  `docs/research/perf-q01-telemetry/results/EXP-01-stable-cargo-surfaces.md`

**Observed behavior**

The public fixture revision intentionally had no committed `Cargo.lock`.
Initial `--locked` runs failed before compiler artifacts were produced. The
corrected protocol checked out an immutable revision, generated and hashed the
lockfile, fetched dependencies before measurement, switched to
`--locked --offline`, and used isolated target directories.

**Implication**

A failed acquisition or lock precondition is evidence, not a slow build.
FERRIUM must record the immutable source revision and either the committed
lockfile or the generated lockfile hash before accepting measurements.

**Confidence:** high.

## Minimum evidence contract

Every promoted Rust build-latency claim must include:

1. immutable fixture identity and lockfile identity;
2. toolchain, host, command, features, profile, jobs, and target-directory
   policy;
3. repeated minimally instrumented wall-clock samples with median and MAD;
4. Cargo metadata captured once per fixture configuration;
5. Cargo JSON captured for representative cold, warm, and edited states, with
   build-script messages treated as possibly cached output;
6. Cargo timings captured separately when unit duration or schedule is part of
   the claim;
7. self-profile captured separately when compiler query or cache behavior is
   part of the claim;
8. rustc-perf evidence before claiming an upstream compiler improvement;
9. an explicit statement of missing telemetry and instrumentation overhead.

## Recommendations

### Adopt now

- Make the layered telemetry stack the PERF program baseline.
- Keep wall-clock benchmark and instrumented diagnostic samples separate.
- Treat Cargo JSON as the stable observed-build ledger, while refusing to infer
  that a build script ran from its replayable output message.
- Require immutable fixture and lockfile identity before measurement.
- Carry the telemetry coverage matrix into every later PERF question.

Owner: FERRIUM.  
Validation: PERF-Q02 and PERF-Q03 must be expressible using the frozen baseline
without changing the evidence vocabulary.

### Prototype behind a compatibility boundary

- A timing-report adapter that extracts only explicitly supported Cargo timing
  information and preserves the original report.
- A nightly self-profile adapter using measureme-compatible tools.
- Optional hardware-counter and peak-memory collectors.

Owner: FERRIUM, with upstream compatibility review.  
Validation: schema/version fixtures, instrumentation calibration, stable-only
fallback, and cross-platform trials.

### Reject or defer

- Parsing undocumented Cargo timing HTML internals as FERRIUM's only durable
  interface.
- Requiring nightly rustc for ordinary repository analysis.
- Using a single instrumented run as the latency benchmark.
- Inferring compiler-query causes from Cargo unit duration alone.

## Role review

| Role | Verdict | Required discipline |
| --- | --- | --- |
| Product lead | Approve | Preserve progressive disclosure: stable overview first, deep compiler evidence on demand. |
| Systems architect | Approve | Keep Cargo, compiler, system, and validation evidence as separate joined layers. |
| Rust/compiler engineer | Approve | Do not infer query invalidation from Cargo freshness or unit duration. |
| Measurement scientist | Approve | Separate diagnostic perturbation from the primary distribution and report limitations. |
| Security/privacy reviewer | Approve | Public evidence uses a public fixture and excludes private paths and source content. |
| Compatibility steward | Approve | Stable Cargo baseline is mandatory; nightly integration remains optional and versioned. |
| Upstream liaison | Approve | Compiler-change claims graduate to rustc-perf and upstream measurement conventions. |
| Developer-experience reviewer | Approve | The baseline can run with the installed stable toolchain and ordinary Cargo commands. |
| Skeptic | Approve with limitation | One small Windows fixture establishes the evidence model, not ecosystem-wide overhead numbers. |

No role raised a blocking objection.

## Open follow-ups

- Measure self-profile overhead and data completeness on an installed nightly
  toolchain before depending on it in a later PERF question.
- Repeat telemetry calibration on a larger workspace and at least one
  non-Windows host.
- Determine whether Cargo exposes a supported structured timing interface
  suitable for long-term ingestion before prototyping a timing adapter.
- Add peak-memory and hardware-counter evidence only when a later decision
  requires them.
