# Rust Latency Component Roadmap

## Decision supported

Define how FERRIUM can help improve Rust build latency without conflating
compiler analysis, incremental compilation, precompiled artifacts, backend
optimization, Cargo orchestration, and linking.

## Research question

Which latency components can FERRIUM measure or improve independently, which
experiments require compatibility boundaries, and which changes belong
upstream in rustc, Cargo, rust-analyzer, or linker projects?

## Why this matters to FERRIUM

Faster feedback compounds the value of AI-generated Rust. An agent can attempt,
compile, test, diagnose, and repair more hypotheses when each verification loop
is short. Slow builds reduce the number of evidence-backed iterations and
encourage weaker validation shortcuts.

The portfolio also contains multiple Rust repositories that repeatedly compile
overlapping dependencies and run similar CI workflows. This creates a useful
laboratory for measuring cross-workspace reuse without making those repositories
depend on FERRIUM.

## Upstream direction

The official Rust
[Fast Builds roadmap](https://github.com/rust-lang/rust-project-goals/blob/main/src/2026/roadmap-fast-builds.md)
states three governing principles:

1. There is no single solution; many improvements must accumulate.
2. User-visible latency includes Cargo coordination and linking, not only rustc.
3. Different workflows require different optimizations.

Its named directions include Relink-Don't-Rebuild, Wild incremental linking,
alternative development backends, compiler hot-path optimization, frontend
parallelism, end-to-end incrementality, shared artifacts, and crate slicing.

## Component map

```text
developer edit
   |
   v
editor / rust-analyzer
   |
   v
Cargo graph + feature/profile/target resolution
   |
   +--> build scripts and procedural macros
   |
   v
rustc frontend
  parse -> expand -> resolve -> HIR -> type/trait -> borrow/MIR
   |
   v
incremental query and artifact reuse
   |
   v
monomorphization + codegen-unit partitioning
   |
   v
LLVM / Cranelift / future backend
   |
   v
object and debug-information emission
   |
   v
LLD / Wild / platform linker
   |
   v
test or executable launch
```

Each boundary has different cache keys, correctness risks, owners, and
measurement tools.

## Components

### FERRIUM-12: Build observability and causal explanation

**Problem**

Cargo timings, rustc self-profile data, rustc-perf, linker timings, and IDE
behavior are separate surfaces. The official compiler survey says users cannot
easily determine what recompiled, which macros cost most, or what action to
take.

**Sources**

- [Rust compiler performance survey](https://blog.rust-lang.org/2025/09/10/rust-compiler-performance-survey-2025-results/#understanding-why-builds-are-slow)
- [rustc profiling guide](https://rustc-dev-guide.rust-lang.org/profiling.html)

**Improvement boundary**

External analysis of exported timing and profile artifacts. No compiler fork is
required.

**FERRIUM role**

Own the neutral measurement vocabulary: workflow, crate graph, critical path,
invalidated unit, query family, codegen, link, cache state, and recommendation
evidence.

**Confidence:** Very high.

### FERRIUM-13: Cargo graph, features, profiles, and scheduling

**Problem**

Dependency count, activated features, large crates, deep workspace graphs,
duplicate build modes, and Cargo/rust-analyzer lock competition can dominate
latency.

**Source**

- [Rust compiler performance survey](https://blog.rust-lang.org/2025/09/10/rust-compiler-performance-survey-2025-results/)

**Improvement boundary**

Cargo metadata and timing outputs. Advice can remain external; scheduling and
artifact-layout changes belong upstream in Cargo.

**Possible improvements**

- Explain the build critical path rather than only total crate time.
- Identify unused or unexpectedly activated features.
- Distinguish crates that should be split from splits that only increase graph
  overhead.
- Detect repeated check/build/test work and build-lock contention.
- Recommend profile, debuginfo, linker, backend, and codegen-unit experiments
  with explicit tradeoffs.

**Confidence:** High.

### FERRIUM-14: Cross-workspace artifact cache and practical precompilation

**Problem**

Cargo artifacts normally belong to one workspace, so identical build units can
be recompiled in many repositories.

**Source**

- [Cargo cross-workspace cache goal](https://github.com/rust-lang/rust-project-goals/blob/main/src/2026/cargo-cross-workspace-cache.md)

**What "precompiled" can mean**

Rust already distributes a precompiled standard library for supported targets
and compiles dependencies into reusable metadata and library artifacts inside a
workspace. The next practical step is reusing an identical build unit across
workspaces.

A reusable artifact key must account for at least:

- rustc and Cargo versions,
- target and host triples,
- source identity,
- crate features and `cfg` values,
- profile and optimization settings,
- panic strategy and codegen flags,
- dependency artifact identities,
- environment inputs,
- build-script and procedural-macro effects.

The official Cargo goal begins conservatively with ordinary crates and excludes
build scripts and procedural macros from the initial cache.

**FERRIUM role**

- Measure duplicate build units across selected public portfolio repositories.
- Define provenance and explain why two apparently similar units are or are not
  cache-compatible.
- Exercise Cargo's nightly cache when available and report reproducible results
  upstream.
- Use PERF-Q30's signed-root, trust, label, revocation, and net-benefit model for
  disposable remote-prewarming experiments, while waiting for Cargo's local
  cache contract before artifact-bearing product work.

**Confidence:** Very high.

### FERRIUM-15: Procedural macros and build scripts

**Problem**

Procedural macros and build scripts execute code during a build. They can read
environment variables, files, tools, and platform state that are not obvious
from their nominal Rust inputs. This makes safe cache reuse harder.

**Sources**

- [Cargo cross-workspace cache goal](https://github.com/rust-lang/rust-project-goals/blob/main/src/2026/cargo-cross-workspace-cache.md)
- [parallel frontend goal](https://github.com/rust-lang/rust-project-goals/blob/main/src/2026/parallel-front-end.md)
- [Rust compiler performance survey](https://blog.rust-lang.org/2025/09/10/rust-compiler-performance-survey-2025-results/#understanding-why-builds-are-slow)

**Possible improvements**

- Measure macro duration and generated token volume.
- Make build-script inputs and outputs more explicit.
- Separate deterministic macros from environment-sensitive macros.
- Investigate sandboxing or declared-input contracts before persistent caching.
- Parallelize expansion where compiler invariants permit.

**FERRIUM role**

Begin with observability and classification. Do not cache arbitrary macro or
build-script execution without a sound input model.

**Confidence:** High.

### FERRIUM-16: Frontend parallelism and early-phase incrementality

**Problem**

Large crates can become single-node critical paths. Parts of parsing, name
resolution, HIR lowering, and macro expansion remain serial or insufficiently
incremental.

**Sources**

- [parallel frontend goal](https://github.com/rust-lang/rust-project-goals/blob/main/src/2026/parallel-front-end.md)
- [Fast Builds roadmap](https://github.com/rust-lang/rust-project-goals/blob/main/src/2026/roadmap-fast-builds.md)

**Upstream work**

The Rust project is stabilizing the parallel frontend, expanding tests and
rustc-perf coverage, enabling Cargo support, reducing contention, and exploring
parallel name resolution and macro expansion.

**FERRIUM role**

- Supply representative large-crate and incremental fixtures.
- Compare serial and parallel correctness and performance.
- Report critical-path and contention evidence.
- Contribute targeted tests, benchmark cases, documentation, or reviewed fixes
  upstream rather than maintaining a fork.

**Confidence:** High that the opportunity exists; medium on attainable speedup
for each workload.

### FERRIUM-17: Query precision and incremental compilation

**Problem**

rustc's red-green query system can reuse unchanged work, but persistent
fingerprinting and dependency tracking cost time. Conservative invalidation can
still propagate broadly. Earlier compiler phases are not covered as completely
as later phases.

**Sources**

- [incremental compilation guide](https://rustc-dev-guide.rust-lang.org/queries/incremental-compilation-in-detail.html)
- [Fast Builds roadmap](https://github.com/rust-lang/rust-project-goals/blob/main/src/2026/roadmap-fast-builds.md)

**Componentized improvements**

1. Reduce false invalidation by improving dependency precision.
2. Incrementalize earlier phases such as resolution and expansion.
3. Reduce fingerprinting, serialization, and cache-loading cost.
4. Improve on-disk format locality and selective loading.
5. Preserve useful work across check, build, lint, and test modes.
6. Explain invalidation paths to developers and compiler contributors.

**FERRIUM role**

First expose invalidation effects externally. Later, candidate rustc changes
must be benchmarked through rustc-perf and contributed upstream.

**Confidence:** High.

### FERRIUM-18: Relink-Don't-Rebuild and cross-crate boundaries

**Problem**

Changing an upstream function body can cause dependent crates to rebuild even
when their source-level view of the dependency is unchanged.

**Source**

- [Fast Builds roadmap](https://github.com/rust-lang/rust-project-goals/blob/main/src/2026/roadmap-fast-builds.md)

**Improvement**

Relink-Don't-Rebuild separates implementation changes from interface changes.
For eligible edits, downstream crates can reuse prior compilation artifacts and
only relink. The official roadmap targets 5-10x improvements for common changes.

**Hard questions**

- Which metadata, inline MIR, generics, constants, macros, and layout facts form
  the true downstream interface?
- When does cross-crate optimization require downstream recompilation?
- How are stale artifacts prevented under changed compiler flags or targets?

**FERRIUM role**

Develop edit classifications and fixtures that test body-only, public API,
inline, generic, constant, macro, layout, and feature changes. Feed failures and
measurements into the upstream RDR effort.

**Confidence:** High.

### FERRIUM-19: Generic instantiation and deeper precompilation

**Problem**

Rust monomorphizes generic code for concrete types. Generic definitions from
dependencies can generate machine code in downstream crates, limiting how fully
the dependency can be precompiled in isolation.

**Sources**

- [monomorphization guide](https://rustc-dev-guide.rust-lang.org/backend/monomorph.html)
- [share-generics experiment](https://github.com/rust-lang/rust/issues/47317)

**Possible improvements**

- Share compatible monomorphized instances across crates.
- Cache mono items by compiler, target, MIR, type arguments, and codegen options.
- Reduce unnecessary generic instantiations through polymorphization.
- Improve visibility into which APIs generate the most LLVM IR.
- Use compiler-owned crate slicing to defer frontend and codegen work that is
  not needed by a root build.

**Tradeoffs**

Sharing can reduce compilation and duplication while limiting local
optimization, inlining, or LTO opportunities. Linkage and symbol ownership must
remain correct.

**FERRIUM role**

Measure first. A generic-instantiation cache is a later research prototype, not
the first product. PERF-Q32 confirms that generic definitions are already
instantiated on demand and that the current `hint-mostly-unused` opportunity
is primarily wide public non-generic codegen. Full crate slicing remains a
separate, unaccepted compiler architecture proposal.

**Confidence:** Medium-high.

### FERRIUM-20: Development codegen backends

**Problem**

LLVM is optimized for high-quality generated code, not minimum latency for
unoptimized edit-run-debug loops.

**Sources**

- [Fast Builds roadmap](https://github.com/rust-lang/rust-project-goals/blob/main/src/2026/roadmap-fast-builds.md)
- [Cranelift performance goal](https://github.com/rust-lang/rust-project-goals/blob/main/src/2026/improve-cg_clif-performance.md)

**Upstream directions**

- Improve Cranelift's backend performance, targeting a 2x internal speedup.
- Explore a persistent function-level machine-code cache.
- Explore JIT or interpretation for infrequently executed functions.
- Investigate TPDE as a future fast backend.

**FERRIUM role**

Provide a backend-selection advisor and compatibility matrix only after
measuring real repositories. Contribute benchmark cases upstream. Do not create
another backend. PERF-Q31 confirms that Cranelift's existing function-stencil
cache can recover precision lost at CGU boundaries, but rustc integration,
admission, integrity, debug, unwind, relocation, and daemon lifecycle belong
upstream. FERRIUM supplies fixtures and evaluation rather than a cache service.

**Confidence:** High.

### FERRIUM-21: Debug information and object emission

**Problem**

Full development debug information increases compilation, object size, cache
size, and link time even when developers mainly need backtraces.

**Source**

- [Rust compiler performance survey](https://blog.rust-lang.org/2025/09/10/rust-compiler-performance-survey-2025-results/#debug-information)

**FERRIUM role**

Measure `full`, `line-tables-only`, and `none` against build latency, debugger
requirements, backtraces, artifact size, and cache transfer cost. Recommend a
profile only with an explicit debugging tradeoff.

**Confidence:** High.

### FERRIUM-22: Linking and incremental linking

**Problem**

Linking commonly repeats global work after a small change and can dominate an
otherwise short rebuild.

**Sources**

- [Fast Builds roadmap](https://github.com/rust-lang/rust-project-goals/blob/main/src/2026/roadmap-fast-builds.md)
- [Rust compiler performance survey](https://blog.rust-lang.org/2025/09/10/rust-compiler-performance-survey-2025-results/)

**Improvement layers**

1. Select a faster existing linker where supported.
2. Avoid downstream recompilation through RDR.
3. Reuse linker state or changed-object knowledge through Wild.
4. Preserve platform correctness, debug behavior, and reproducibility.

**FERRIUM role**

Attribute link time separately, maintain representative link-heavy fixtures,
and report platform-specific results. Do not initially build a linker.

**PERF-Q29 disposition**

The Windows public control found a modest 4.4% complete-link improvement from
`rust-lld` and a 75.5% unchanged MSVC incremental-link improvement. The latter
required `/OPT:NOREF`, a larger executable and PDB, and 53.2 MB of ILK state.
One ordinary Rust body edit renamed the complete root-object set and forced a
full link. The immediate opportunity is therefore a read-only linker plan and
state ledger plus an upstream stable-input-identity fixture, not a FERRIUM
linker or automatic configuration.

See [Linking and incremental linking](2026-08-09-linking-incremental-linking.md).

**Confidence:** High for the measured mechanism; medium for cross-platform
prevalence.

### FERRIUM-23: IDE and verification-loop integration

**Problem**

rust-analyzer, `cargo check`, Clippy, tests, and executable builds can duplicate
analysis or compete for locks and disk artifacts.

**Sources**

- [Rust compiler performance survey](https://blog.rust-lang.org/2025/09/10/rust-compiler-performance-survey-2025-results/#type-checking-and-ide-performance)
- [Fast Builds roadmap](https://github.com/rust-lang/rust-project-goals/blob/main/src/2026/roadmap-fast-builds.md)

**FERRIUM role**

Model the entire verification loop, not one command. Measure time-to-first
diagnostic and time-to-tested-result for human and agent workflows.

**Confidence:** High.

### System environment boundary

Filesystem placement, VM and container mounts, memory limits, concurrent
sessions, CPU job policy, antivirus, indexing, power, and thermal state can
change wall time without changing compiler work.

PERF-Q33 measured the same Linux toolchain and METIS source inside one WSL2 VM.
Keeping source and target on ext4 produced a 4.81-second clean median and a
57.8-millisecond warm no-op median. Placing both on the mounted Windows volume
produced 16.52 seconds and 13.68 seconds. Target placement was the largest
boundary. Cargo job scaling reached a plateau from eight through twenty-four
jobs; all logical processors were not automatically best.

The FERRIUM role is a read-only environment fingerprint, comparison guard, and
supported diagnostic path. Security exclusions, service or power changes,
forced memory pressure, repository migration, universal job settings, and host
automation remain closed.

See [System effects on Rust build latency](2026-08-09-system-effects-build-latency.md).

## Precompilation ladder

Precompilation is not one feature. It is a sequence of increasingly difficult
reuse levels:

| Level | Reused unit | Feasibility | Main blocker |
|---:|---|---|---|
| 1 | Standard library shipped with toolchain | Existing | Custom targets/configurations |
| 2 | Same-workspace dependency artifact | Existing | Cache invalidation and profile variants |
| 3 | Identical ordinary crate across workspaces | Active Cargo goal | Stable build-unit identity and cleanup |
| 4 | Remote prewarmed ordinary artifacts | Plausible after Level 3 | Trust, provenance, transport, platform matrix |
| 5 | Build-script/proc-macro artifacts | Research | Hidden environment and filesystem inputs |
| 6 | Shared generic instantiations | Experimental | Optimization, symbol ownership, codegen compatibility |
| 7 | Function-level machine-code cache | Cranelift mechanism demonstrated; rustc integration research | Rust identity, admission, integrity, daemon lifecycle, memory, optimization, debugging |
| 8 | Crate slicing / partial dependency compilation | Selective nightly codegen slicing exists; full slicing is unaccepted | Whole-crate frontend correctness, coherence, macros, generated code, dynamic dispatch, diagnostics, and scheduling |

FERRIUM should climb this ladder in order rather than calling every cache
"precompilation."

## FERRIUM roadmap

### Phase 0: Measurement contract — now

**Output:** research and benchmark specification only.

- Define clean, incremental, check, build, test, link, and IDE scenarios.
- Select public fixtures representing graph-heavy, generic-heavy,
  macro-heavy, large-crate, and link-heavy builds.
- Define machine-readable evidence fields and privacy boundaries.
- Establish stable-only and optional-nightly measurement tiers.

**Exit gate:** independent role review confirms that commands, environments,
cache states, and tradeoffs are reproducible.

### Phase 1: Portfolio latency census

**Output:** measured reports, still no integration dependency.

- Measure duplicate build units across selected public portfolio repos.
- Identify critical-path crates, repeated dependencies, feature divergence,
  codegen/link dominance, and CI cache duplication.
- Compare controlled edit intent with observed package, target, test, codegen,
  and link work.
- Determine where stable Cargo evidence stops and optional compiler-detail
  evidence materially improves causality.
- Compare current defaults with supported configuration changes.
- Produce upstream-quality minimal reproductions for surprising behavior.

**Exit gate:** at least three distinct repositories demonstrate a repeated,
product-neutral problem.

### Phase 2: Build-causality prototype

**Output:** optional external analyzer behind exported-file boundaries.

- Ingest Cargo timings and metadata.
- Optionally ingest nightly self-profile summaries.
- Explain what rebuilt, why it was on the critical path, and which experiments
  are relevant.
- Forecast the package, target, test, and link blast radius of held-out edits.
- Recommend an evidence-backed validation plan with explicit coverage,
  uncertainty, mandatory gates, and human approval boundaries.
- Emit a reviewable Ferris build evidence packet.
- Never silently apply source or configuration changes.

**Exit gate:** explanations and forecasts match held-out observed work closely
enough to reduce maintainer investigation time without misclassifying compiler,
backend, linker, or validation costs.

### Phase 3: Precompilation and cache experiments

**Output:** compatibility and measurement adapters around upstream Cargo work.

- Test Cargo's cross-workspace cache when available.
- Measure hit rate, disk reduction, correctness, and invalidation causes.
- Apply the PERF-Q30 provenance and forest-root contract to locally and remotely
  produced artifacts.
- Keep proc macros, build scripts, and generic-instance caching separate.

**Exit gate:** demonstrated reuse across workspaces with no stale-artifact or
configuration-identity failures.

### Phase 4: Incremental and RDR contribution program

**Output:** fixtures, rustc-perf cases, issue reports, documentation, and
targeted upstream PRs.

- Maintain body-only versus interface-changing edit suites.
- Contribute RDR, frontend-parallel, and cache correctness cases.
- Investigate one narrowly measured invalidation or profiler gap at a time.
- Fund or collaborate with existing owners when changes require sustained
  compiler expertise.

**Exit gate:** upstream acceptance or a documented reason to keep an experiment
external.

### Phase 5: Advanced reuse research

**Output:** research prototypes only after upstream alignment.

- Shared generic-instance cache.
- Deterministic proc-macro/build-script contracts.
- Upstream-owned function-level Cranelift integration and daemon experiments.
- Full crate slicing and frontend partial compilation after upstream
  acceptance; current codegen-only hinting remains a measured Cargo experiment.

**Exit gate:** an upstream sponsor, precise compatibility model, benchmark suite,
and clear maintenance owner exist.

## Adopt now

- Make latency a FERRIUM research program with component-level measurements.
- Align terminology and fixtures with the official Fast Builds roadmap.
- Start with build causality and cross-workspace duplicate-work measurement.
- Treat Cargo cross-workspace caching and RDR as contribution opportunities.
- Maintain PERF-Q31 function-cache precision, corruption, admission, and public
  repository fixtures for upstream Cranelift work.
- Maintain PERF-Q32 sparse, dense, generic, private, multi-consumer,
  whole-crate-error, and public-repository fixtures for Cargo evaluation.
- Require PERF-Q33 environment equivalence and attribution confidence before
  promoting compiler, Cargo, backend, linker, or cache comparisons.

## Prototype behind a compatibility boundary

- An external analyzer reading Cargo metadata/timings and optional summarized
  rustc self-profile output.
- Cache experiments that use upstream Cargo nightly interfaces.
- Backend and linker experiments selected through configuration, not forks.
- Read-only baseline-versus-`hint-mostly-unused` comparisons for explicitly
  selected dependencies in disposable target directories.
- Read-only source/target placement, job-response, memory/session, security,
  indexing, power, and VM diagnostics through supported interfaces.

## Defer or reject

- Defer direct rustc modification until a fixture demonstrates a specific
  upstream defect or hot path.
- Defer generic-instance and proc-macro caching until their input and
  correctness models are explicit.
- Defer a FERRIUM function-cache daemon, machine-code store, LLVM or LTO cache,
  persistence, and restoration; support only upstream-owned,
  development-Cranelift experiments under the PERF-Q31 boundary.
- Defer full crate slicing, stub rlibs, source-level slicing, automatic hint
  adoption, manifest rewrites, and compiler forks under the PERF-Q32 boundary.
- Reject automatic security exclusions, protection or indexing disablement,
  power-plan, affinity, priority, VM, swap, memory-pressure, repository
  placement, and universal job changes under the PERF-Q33 boundary.
- Defer production remote binary distribution and automatic restoration until
  Cargo identity, path portability, platform coverage, and real-service
  economics satisfy the PERF-Q30 prototype gate.
- Reject a FERRIUM compiler fork, backend, or linker as the opening move.
- Reject aggregate "Rust is slow" benchmarks that do not identify the component
  and workflow.

## Non-goals

- Promising one universal speedup.
- Treating cached native artifacts as portable across incompatible toolchains,
  targets, flags, or environments.
- Trading away safety analysis or release optimization without naming the cost.
- Making portfolio repositories depend on experimental FERRIUM tooling.
- Competing with active upstream owners where fixtures, funding, review, or
  contributions would help more.
