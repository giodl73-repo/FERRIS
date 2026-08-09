# Rust Incremental Reuse: Scopes, Boundaries, and Contribution Paths

## Decision supported

Identify where Rust build work can be reused, what invalidates each reuse scope,
which evidence surfaces expose that behavior, and where FERRIUM can help
externally versus through upstream Rust contributions.

## Research question

When a Rust developer changes source, which correctness, code-generation, and
linking results must be recomputed; which can be reused; and what bounded
improvements could reduce unnecessary work without weakening correctness?

## Boundary map

| Scope | Reused unit | Common invalidators | Current evidence | Primary owner |
|---|---|---|---|---|
| Workflow | Prior check/build/test outputs | Command, profile, features, target, environment | Cargo command and artifact state | Cargo and repository |
| Build unit | Package, target, profile, feature, compiler-option combination | Source, dependency, flags, target, toolchain | Cargo metadata, JSON messages, fingerprints, timings | Cargo |
| Compiler invocation | Crate compilation result and metadata | Build-unit invalidation | Cargo messages and rustc invocation | Cargo/rustc boundary |
| Compiler query | Persisted result plus dependency edges | Red dependency, changed input fingerprint, unavailable cache | Nightly self-profile and rustc internals | rustc |
| Item/body | Type checking, MIR, borrow checking, optimization results where query granularity permits | Signature, body, trait, type, macro, layout, or dependency changes | Mostly summarized compiler evidence | rustc |
| Cross-crate interface | Metadata, exported types, traits, constants, macros, inline MIR, layouts | Public or optimization-relevant interface change | Metadata hashes and downstream work | rustc |
| Partial dependency codegen | Eligible public non-generic bodies emitted in dependency or consumers | Consumer demand, profile, inline policy, target, crate type, toolchain | Mono-item ownership, self-profile, artifact bytes, complete-build outcomes | Cargo/rustc boundary |
| Generic instance | Monomorphized function/type for concrete arguments | Generic MIR, type arguments, target, codegen options, optimization context | Mono-item and backend profiling | rustc/backend |
| Codegen unit | LLVM IR, object code, debug information | Mono-item assignment, codegen flags, target, optimization | Self-profile, object and timing evidence | rustc/backend |
| Link | Final binary or library image | Object, native library, link flags, debug data | Link timing and linker diagnostics | rustc/linker |
| Workspace cache | Compatible artifact reused by another workspace | Identity mismatch, cleanup, unavailable artifact | Cargo cache experiments | Cargo |
| Remote cache | Artifact produced elsewhere | Provenance, platform, toolchain, environment, trust | Future cache protocol evidence | Cargo/tooling ecosystem |

The scopes are nested but not perfectly aligned. A tiny source edit may leave
most query results reusable while still causing a crate-level Cargo build unit
and downstream link step to run.

## Findings

### FERRIUM-24: Cargo can avoid rustc entirely when the build unit is valid

**Sources**

- [Cargo build cache](https://doc.rust-lang.org/cargo/guide/build-cache.html)
- [Cargo build scripts: change detection](https://doc.rust-lang.org/cargo/reference/build-scripts.html#change-detection)

**Observation**

Cargo decides whether a package target must run before rustc can reuse compiler
queries. The effective build-unit identity includes the package, target,
profile, enabled features, compiler options, target triple, dependency
artifacts, source inputs, and relevant build-script outputs.

**Implication**

The cheapest correctness analysis is the invocation that Cargo can safely skip.
FERRIUM can help immediately by explaining why two apparently similar commands
or workspaces do not share the same build unit.

**Confidence:** High.

### FERRIUM-25: Incremental rustc revalidates dependencies rather than blindly trusting prior results

**Sources**

- [Incremental compilation in detail](https://rustc-dev-guide.rust-lang.org/queries/incremental-compilation-in-detail.html)
- [Query evaluation model](https://rustc-dev-guide.rust-lang.org/queries/query-evaluation-model-in-detail.html)

**Observation**

rustc persists query fingerprints and dependency edges. On the next compiler
invocation, the red-green algorithm attempts to prove that cached query results
remain valid. Reuse therefore avoids recomputation but still costs dependency
graph loading, fingerprint comparison, stable hashing, and cache management.

**Implication**

Incremental performance has two separate targets: reduce unnecessary
invalidation and reduce the cost of proving reuse. FERRIUM should measure both
rather than treating cache hits as free.

**Confidence:** High.

### FERRIUM-26: Reuse granularity depends on query boundaries, not source-file size

**Sources**

- [rustc query system](https://rustc-dev-guide.rust-lang.org/query.html)
- [Incremental compilation architecture](https://rustc-dev-guide.rust-lang.org/queries/incremental-compilation.html)

**Observation**

Compiler work is memoized through queries with different keys and dependency
shapes. Some operations are naturally item- or body-oriented; others summarize
whole modules, crates, traits, metadata, or global compiler state. A one-line
edit can therefore invalidate either a narrow body query or a broad aggregate.

**Implication**

FERRIUM should compare the semantic class of an edit with the observed
invalidation fan-out. Unexpectedly broad cases are candidates for minimized
rustc-perf fixtures and upstream query-precision work.

**Confidence:** High on the model; medium on the granularity of any unmeasured
compiler query.

### FERRIUM-27: Earlier compiler phases have weaker reuse and parallelism boundaries

**Sources**

- [Parallel frontend project goal](https://github.com/rust-lang/rust-project-goals/blob/main/src/2026/parallel-front-end.md)
- [Fast Builds roadmap](https://github.com/rust-lang/rust-project-goals/blob/main/src/2026/roadmap-fast-builds.md)

**Observation**

Parsing, macro expansion, name resolution, and HIR lowering contain serial,
crate-wide, or incompletely incremental work. Large crates can remain critical
paths even when later type-checking or MIR queries reuse effectively.

**Implication**

FERRIUM can supply large-crate fixtures, distinguish frontend-dominant edits,
and contribute performance and correctness cases. Structural changes belong
upstream because they alter compiler scheduling and internal invariants.

**Confidence:** High that the boundary matters; medium on workload-specific
impact.

### FERRIUM-28: Cross-crate reuse is constrained by the true downstream interface

**Sources**

- [Rust 2026 Fast Builds roadmap](https://github.com/rust-lang/rust-project-goals/blob/main/src/2026/roadmap-fast-builds.md)
- [rustc metadata](https://rustc-dev-guide.rust-lang.org/backend/libs-and-metadata.html)

**Observation**

Downstream compilation depends on more than source-level public signatures.
Exported types, traits, constants, macros, layouts, generic bodies, inline MIR,
and optimization-relevant facts can become part of the effective interface.
Today, body-only upstream edits may still trigger downstream rebuilds.

**Implication**

Relink-Don't-Rebuild is a major opportunity, but the eligibility boundary must
remain compiler-owned. FERRIUM can contribute edit taxonomies, held-out
fixtures, false-reuse tests, and measured downstream fan-out.

**Confidence:** High.

### FERRIUM-29: Generics cross the dependency boundary into downstream code generation

**Sources**

- [Monomorphization guide](https://rustc-dev-guide.rust-lang.org/backend/monomorph.html)
- [Share-generics tracking issue](https://github.com/rust-lang/rust/issues/47317)

**Observation**

Generic definitions in a dependency can be instantiated for concrete types in
consumer crates. The reusable unit is therefore not simply the compiled
dependency. It may be a monomorphized item whose identity includes generic MIR,
type arguments, target, compiler version, codegen flags, and optimization
context.

**Implication**

FERRIUM can expose generic-driven codegen cost and duplicate instantiations.
Shared generic-instance caching is later research because it affects linkage,
inlining, LTO, symbol ownership, and runtime optimization.

**Confidence:** High on the constraint; medium on the best reuse design.

### FERRIUM-30: Procedural macros and build scripts weaken deterministic reuse boundaries

**Sources**

- [Procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html)
- [Cargo build-script change detection](https://doc.rust-lang.org/cargo/reference/build-scripts.html#change-detection)

**Observation**

Procedural macros execute code during compilation. Build scripts can observe
filesystem and environment inputs and must declare rerun conditions precisely.
Hidden or broad inputs make invalidation conservative and cross-workspace reuse
unsafe.

**Implication**

FERRIUM should first inventory execution time, declared inputs, rerun causes,
generated output, and downstream fan-out. It must not cache arbitrary execution
until the input and output model is deterministic and reviewable.

**Confidence:** High.

### FERRIUM-31: Codegen-unit boundaries trade reuse and parallelism against optimization

**Sources**

- [Monomorphization and codegen units](https://rustc-dev-guide.rust-lang.org/backend/monomorph.html)
- [Cargo profiles: codegen units](https://doc.rust-lang.org/cargo/reference/profiles.html#codegen-units)

**Observation**

rustc partitions monomorphized items into codegen units. More units can improve
parallel development builds and incremental reuse, while fewer units can enable
stronger optimization. Changes in partitioning or optimization context can
invalidate object-level reuse.

**Implication**

FERRIUM can measure profile and codegen-unit tradeoffs and explain where time is
spent. It should not recommend faster development settings as release settings
without reporting runtime, size, and optimization consequences.

**Confidence:** High.

### FERRIUM-32: Relinking is a distinct reuse boundary after compilation

**Sources**

- [rustc linker behavior](https://rustc-dev-guide.rust-lang.org/backend/linker.html)
- [Fast Builds roadmap](https://github.com/rust-lang/rust-project-goals/blob/main/src/2026/roadmap-fast-builds.md)

**Observation**

Unchanged crate metadata or object code does not imply that the final binary can
be reused. Object changes, native libraries, link flags, debug data, and linker
capabilities determine whether a full or incremental link is required.

**Implication**

FERRIUM must report compile and link work separately. It can evaluate supported
linkers and produce relink-only fixtures, but should contribute to existing
incremental-linking efforts rather than create a linker.

**Confidence:** High.

### FERRIUM-33: Cross-workspace reuse is primarily an artifact-identity problem

**Sources**

- [Cargo build cache](https://doc.rust-lang.org/cargo/guide/build-cache.html)
- [Fast Builds roadmap](https://github.com/rust-lang/rust-project-goals/blob/main/src/2026/roadmap-fast-builds.md)

**Observation**

Two workspaces may compile semantically identical dependencies but store or
identify them as separate build units. Safe sharing requires stable identity
across compiler, target, profile, features, flags, dependency graph, build
scripts, environment, and artifact format.

**Implication**

FERRIUM can measure duplicate units and explain identity differences now. Cache
distribution must follow provenance and invalidation design, not precede it.

**Confidence:** High.

### FERRIUM-34: Validation reuse is a repository-policy boundary, not only a compiler boundary

**Sources**

- [FERRIUM engineering principles](../governance/ENGINEERING_PRINCIPLES.md)
- [Build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)

**Observation**

A compiler may safely reuse analysis while a repository still requires broader
tests, lints, platform checks, or release gates. Conversely, repeatedly running
unaffected validation can dominate the edit-to-confidence loop.

**Implication**

FERRIUM may recommend targeted validation only when it names coverage,
uncertainty, mandatory gates, and human approval boundaries. Compiler
incrementality cannot be treated as proof that unrelated behavior is safe.

**Confidence:** High.

## Contribution boundary

### Help externally now

- Join Cargo metadata, JSON messages, timings, graph structure, profiles,
  features, targets, and edit classifications.
- Explain package/target rebuilds, build-script reruns, link work, cache
  identity, and validation duplication.
- Produce deterministic fixture edits and minimized reproductions.
- Compare supported profiles, backends, and linkers without changing defaults
  silently.
- Explain already-lazy metadata, generics, and private code versus avoidable
  public dependency codegen.
- Emit evidence packets that preserve unknown causes and failed runs.

### Prototype behind a compatibility boundary

- Optional nightly self-profile summaries.
- Cross-workspace cache experiments using upstream Cargo interfaces.
- Pre-change build-impact prediction validated against held-out edits.
- Build-script and procedural-macro input auditing.
- Relink-Don't-Rebuild eligibility and regression fixtures.
- Disposable baseline-versus-`hint-mostly-unused` comparisons for explicitly
  selected sparse-use candidates.

### Contribute upstream

- Narrower query dependencies and reduced false invalidation.
- Earlier-phase incrementality and frontend parallelism.
- Lower fingerprinting, serialization, and cache-loading overhead.
- More precise cross-crate interface and RDR behavior.
- Compiler diagnostics that expose invalidation paths.
- Monomorphization visibility, polymorphization, and carefully reviewed sharing.
- Partial-dependency eligibility, codegen-ownership, dense-use, duplication,
  and whole-crate correctness cases for Cargo and rustc.
- rustc-perf cases for representative real-world edits.

### Defer

- Direct dependence on `rustc_private`.
- A FERRIUM compiler fork.
- Arbitrary macro or build-script result caching.
- Shared generic machine code without an upstream compatibility model.
- Remote native artifacts without complete provenance and trust rules.
- Full crate slicing, source transformation, or stub rlibs without an accepted
  upstream owner and compiler-owned semantic design.

## Prioritized ways to make Rust faster

| Priority | Improvement | Why this order |
|---:|---|---|
| 1 | Explain rebuild and invalidation causes | Lowest compatibility risk and immediately improves diagnosis |
| 2 | Eliminate duplicate Cargo build units and cache-identity mismatches | Often avoids entire compiler invocations |
| 3 | Advance RDR for eligible body-only edits | Avoids downstream rebuilds while preserving correctness |
| 4 | Improve query precision and incremental-cache overhead | Reuses more correctness work inside rustc |
| 5 | Parallelize and incrementalize earlier frontend phases | Reduces large-crate critical paths |
| 6 | Improve proc-macro and build-script input discipline | Narrows opaque and conservative reruns |
| 7 | Improve development codegen and linking choices | Reduces backend-dominant iteration time |
| 8 | Research generic-instance and function-level reuse | High potential but complex optimization and identity boundaries |
| 9 | Evaluate selective codegen slicing; research full crate slicing | Current nightly hinting has measured sparse-use value, while frontend slicing requires upstream redesign |

## Role review

| Role | Review disposition |
|---|---|
| Rust Safety Steward | Accepted: every reuse level requires a correctness identity and no safety analysis is classified as removable waste. |
| Compiler Performance Engineer | Accepted: compiler, Cargo, codegen, link, and validation boundaries remain separately measurable. |
| Interop Boundary Auditor | Accepted: native libraries, build scripts, targets, and link behavior remain explicit boundaries. |
| AI Assurance Skeptic | Accepted: unknown invalidation causes and validation uncertainty remain visible. |
| Ecosystem Strategist | Accepted: external diagnosis and upstream contribution precede forks or replacement infrastructure. |
| Rust Maintainer | Accepted: the first interventions explain existing behavior and produce minimized cases rather than source churn. |
| Native Platform Adopter | Accepted: recommendations preserve ordinary workflows, reversibility, provenance, and private evidence boundaries. |
| Scope Keeper | Accepted: each proposed intervention is assigned to external, compatibility-boundary, upstream, or deferred scope. |
| Validation Checker | Accepted: every improvement must be tested with controlled edits, negative cases, cache states, and held-out workloads. |

## Recommendation

Adopt the reuse-scope map as the organizing model for the latency census.
Measure from the outside inward:

1. Did Cargo invoke rustc?
2. Which crate targets ran?
3. Which compiler categories recomputed?
4. Which generic/codegen units changed?
5. Did compilation or linking dominate?
6. Which prior artifact identity failed to match?
7. Which validation repeated, and was it required?

Only cross into rustc implementation after a fixture isolates a repeated,
upstream-owned boundary with measurable impact.
