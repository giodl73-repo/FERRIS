# Rust Compiler Performance: Architecture, Bottlenecks, and FERRIUM Opportunities

## Decision supported

Decide whether FERRIUM should begin by modifying the Rust compiler, building a
new compiler backend, or creating an external build-intelligence layer that
explains and improves existing Cargo and rustc workflows.

## Research question

Why can Rust compilation be slow, which parts of the cost belong to rustc
itself versus Cargo, linking, project structure, and development configuration,
and where can FERRIUM add value without prematurely forking the compiler?

## Local evidence

- `README.md:5-32` defines Hammer as the build-causality lane and keeps FERRIUM
  research-only until a cited note, baseline, and bounded validation contract
  exist.
- `PRODUCT_PLAN.md:33-37` proposes build intelligence that attributes rebuild
  cost to crate graphs, features, linking, and caching.
- `context/waves/2026-08-07-lab-foundation/WAVE.md:10-38` prohibits product
  selection and implementation before the opportunity benchmark.
- `.roles/parliament/compiler-performance-engineer.md` requires cold,
  incremental, check, build, test, and link workflows to be measured
  separately.

## Source access

Yes: the complete compiler source is public at
[`rust-lang/rust`](https://github.com/rust-lang/rust). The repository contains
rustc, the standard library, rustdoc, bootstrap infrastructure, and associated
tools. Rust is dual-licensed under Apache-2.0 and MIT terms
([project copyright statement](https://github.com/rust-lang/rust/blob/master/COPYRIGHT)).

As inspected on 2026-08-07, the repository's compiler tree includes dedicated
crates for parsing, expansion, resolution, HIR, type checking, trait solving,
borrow checking, MIR, monomorphization, incremental compilation, LLVM, Cranelift,
GCC code generation, the query implementation, and the compiler driver.

Important source locations include:

| Area | Source |
|---|---|
| Driver and phase orchestration | [`compiler/rustc_driver_impl`](https://github.com/rust-lang/rust/tree/master/compiler/rustc_driver_impl) |
| Parsing and macro expansion | [`compiler/rustc_parse`](https://github.com/rust-lang/rust/tree/master/compiler/rustc_parse), [`compiler/rustc_expand`](https://github.com/rust-lang/rust/tree/master/compiler/rustc_expand) |
| Type checking and trait solving | [`compiler/rustc_hir_typeck`](https://github.com/rust-lang/rust/tree/master/compiler/rustc_hir_typeck), [`compiler/rustc_trait_selection`](https://github.com/rust-lang/rust/tree/master/compiler/rustc_trait_selection), [`compiler/rustc_next_trait_solver`](https://github.com/rust-lang/rust/tree/master/compiler/rustc_next_trait_solver) |
| Borrow checking and MIR | [`compiler/rustc_borrowck`](https://github.com/rust-lang/rust/tree/master/compiler/rustc_borrowck), [`compiler/rustc_mir_transform`](https://github.com/rust-lang/rust/tree/master/compiler/rustc_mir_transform) |
| Query and incremental engine | [`compiler/rustc_query_impl`](https://github.com/rust-lang/rust/tree/master/compiler/rustc_query_impl), [`compiler/rustc_middle/src/dep_graph`](https://github.com/rust-lang/rust/tree/master/compiler/rustc_middle/src/dep_graph) |
| Monomorphization | [`compiler/rustc_monomorphize`](https://github.com/rust-lang/rust/tree/master/compiler/rustc_monomorphize) |
| Code generation | [`compiler/rustc_codegen_ssa`](https://github.com/rust-lang/rust/tree/master/compiler/rustc_codegen_ssa), [`compiler/rustc_codegen_llvm`](https://github.com/rust-lang/rust/tree/master/compiler/rustc_codegen_llvm), [`compiler/rustc_codegen_cranelift`](https://github.com/rust-lang/rust/tree/master/compiler/rustc_codegen_cranelift) |

The official development guide recommends a full or partial clone and uses
`x.py`/`x` for bootstrapping. A compiler build can require roughly 10-15 GB of
free storage. On Windows, the supported entry point is `x.ps1`
([build guide](https://rustc-dev-guide.rust-lang.org/building/how-to-build-and-run.html)).

```powershell
git clone --filter=blob:none https://github.com/rust-lang/rust.git C:\src\rust
Set-Location C:\src\rust
.\x.ps1 setup
.\x.ps1 check
```

`rustup component add rust-src` is useful for standard-library source, but it is
not a substitute for cloning the compiler repository.

## Compiler pipeline

The official
[compiler overview](https://rustc-dev-guide.rust-lang.org/overview.html)
describes this broad path:

```text
source
  -> lexing and parsing
  -> macro expansion and name resolution
  -> AST lowering to HIR
  -> type inference, trait solving, and type checking
  -> THIR and MIR construction
  -> borrow checking and MIR optimization
  -> monomorphization collection
  -> codegen-unit partitioning
  -> LLVM/Cranelift/GCC backend
  -> object generation
  -> linking
```

`cargo check` normally stops before machine-code generation and final linking.
That makes it useful for separating front-end semantic cost from backend and
linker cost, but `cargo check` and `cargo build` do not currently share all
artifacts, so alternating between them can repeat work
([2025 compiler performance survey](https://blog.rust-lang.org/2025/09/10/rust-compiler-performance-survey-2025-results/)).

## Findings

### FERRIUM-01: "Rust compilation" is several different workloads

**Sources**

- [rustc compiler overview](https://rustc-dev-guide.rust-lang.org/overview.html)
- [rustc code generation guide](https://rustc-dev-guide.rust-lang.org/backend/codegen.html)
- [2025 compiler performance survey](https://blog.rust-lang.org/2025/09/10/rust-compiler-performance-survey-2025-results/)

**Observation**

Front-end checking, incremental invalidation, generic instantiation, LLVM
optimization, debug-information generation, and linking are separate costs.
Their proportions change by command, profile, crate graph, platform, linker,
cache state, and source edit.

**Implication**

FERRIUM must not report one undifferentiated "compile time." Any benchmark and
diagnostic contract must distinguish at least clean check, clean build,
incremental check, incremental build, test build, code generation, and link
time.

**Confidence:** High.

### FERRIUM-02: Rust deliberately performs more semantic work than simpler native languages

**Sources**

- [rustc compiler overview](https://rustc-dev-guide.rust-lang.org/overview.html)
- `compiler/rustc_hir_typeck`
- `compiler/rustc_trait_selection`
- `compiler/rustc_borrowck`

**Observation**

rustc performs type inference, trait solving, coherence checking, borrow
checking, pattern analysis, MIR validation, linting, and extensive diagnostics.
These checks are central to Rust's value rather than accidental overhead.

**Implication**

A credible speed strategy should avoid framing safety analysis as removable
waste. Better opportunities are incremental reuse, parallel scheduling,
diagnostic reuse, project-structure guidance, and faster development backends.

**Confidence:** High for the work performed; medium for its share of any
specific build until measured.

### FERRIUM-03: The query system already provides a causal model

**Sources**

- [query evaluation model](https://rustc-dev-guide.rust-lang.org/queries/query-evaluation-model-in-detail.html)
- [`compiler/rustc_middle/src/dep_graph/graph.rs`](https://github.com/rust-lang/rust/blob/master/compiler/rustc_middle/src/dep_graph/graph.rs)

**Observation**

rustc models compiler knowledge as lazily evaluated, memoized queries. Query
invocations form a dependency graph, and results are cached. This architecture
is designed to expose dependencies between computations and support reuse.

**Implication**

The query/dependency model is the strongest technical foundation for explaining
why a source edit triggered work. FERRIUM should consume supported profiler and
timing outputs first rather than link directly to rustc's unstable internal API.

**Confidence:** High.

### FERRIUM-04: Incremental compilation trades recomputation for tracking, hashing, persistence, and conservative invalidation

**Sources**

- [incremental compilation in detail](https://rustc-dev-guide.rust-lang.org/queries/incremental-compilation-in-detail.html)
- [`compiler/rustc_middle/src/dep_graph/graph.rs`](https://github.com/rust-lang/rust/blob/master/compiler/rustc_middle/src/dep_graph/graph.rs)

**Observation**

Incremental rustc persists a dependency graph and fingerprints across compiler
processes. The red-green algorithm tries to prove cached results remain valid.
Stable hashing, serialization, disk loading, identity mapping, and dependency
checking all have costs. The guide explicitly notes that fingerprint
computation is a major reason incremental compilation can be slower than
non-incremental compilation in some cases.

Small changes can also affect broad outputs. At the workspace level, changing a
crate can force dependent crates to rebuild even when relinking might be
sufficient.

**Implication**

There are two distinct opportunities: improve compiler invalidation itself, or
make invalidation visible and actionable to developers. The latter has a much
smaller compatibility and maintenance burden for FERRIUM.

**Confidence:** High.

### FERRIUM-05: Monomorphization exchanges compile time and binary size for runtime performance

**Sources**

- [monomorphization guide](https://rustc-dev-guide.rust-lang.org/backend/monomorph.html)
- [`compiler/rustc_monomorphize/src/collector.rs`](https://github.com/rust-lang/rust/blob/master/compiler/rustc_monomorphize/src/collector.rs)

**Observation**

Rust emits specialized machine-code instances for concrete uses of generic
functions and types. Generic definitions from dependency crates can be
instantiated in downstream crates. The compiler constructs a graph of required
mono items, including functions, closures, statics, drop glue, vtables, and
shims.

This produces fast runtime code but increases collection work, generated IR,
backend optimization work, and binary size.

**Implication**

A useful build advisor can identify crates, generic APIs, feature selections,
and instantiation patterns producing disproportionate codegen work. It must not
recommend dynamic dispatch or abstraction removal without reporting runtime and
maintainability tradeoffs.

**Confidence:** High.

### FERRIUM-06: LLVM is both a major strength and a major development-build cost

**Sources**

- [rustc code generation guide](https://rustc-dev-guide.rust-lang.org/backend/codegen.html)
- [rustc profiling guide](https://rustc-dev-guide.rust-lang.org/profiling.html)
- [Rust compiler performance survey, "What's next"](https://blog.rust-lang.org/2025/09/10/rust-compiler-performance-survey-2025-results/#whats-next)

**Observation**

rustc normally lowers MIR to LLVM IR, partitions work into codegen units, and
runs LLVM optimization and object generation in parallel. LLVM supplies mature
optimizations and broad target support, but optimized backend work is expensive.
The profiling guide states that much of the cost of compiling rustc itself is
spent in LLVM and recommends measuring generated LLVM IR volume.

Cranelift is already an official alternative-backend initiative intended to
improve development-build speed. A GCC backend also exists.

**Implication**

FERRIUM should not begin by creating another backend. It can instead explain
when an existing backend is suitable, quantify compatibility gaps, and preserve
release-build validation under LLVM.

**Confidence:** High.

### FERRIUM-07: Parallelism exists, but important serial regions and scaling limits remain

**Sources**

- [parallel compilation guide](https://rustc-dev-guide.rust-lang.org/parallel-rustc.html)
- [Rust compiler performance survey, "What's next"](https://blog.rust-lang.org/2025/09/10/rust-compiler-performance-survey-2025-results/#whats-next)

**Observation**

Code generation is parallel through codegen units. Type checking, borrow
checking, and MIR optimization have parallel implementations, but the guide
notes that frontend parallelism has been under active change and has not always
been enabled by default. Lexing, parsing, HIR lowering, and macro expansion have
remained serial regions. Shared data structures and query synchronization can
also create contention and reduced scaling beyond a small number of threads.

**Implication**

More cores cannot automatically eliminate build latency. FERRIUM should report
parallel occupancy, critical paths, serial regions, and lock or dependency
bottlenecks rather than only aggregate CPU use.

**Confidence:** Medium-high because the implementation is actively changing.

### FERRIUM-08: Linking and debug information are often outside the core semantic compiler bottleneck

**Source**

- [2025 compiler performance survey](https://blog.rust-lang.org/2025/09/10/rust-compiler-performance-survey-2025-results/)

**Observation**

The survey identifies linking as a frequent incremental-build complaint because
it is generally repeated from scratch. Faster linkers materially improve this
stage. Full development-profile debug information also increases compile time,
link time, and target-directory size; Rust's measurements cited improvements of
roughly 2-30% in cycle counts from reducing debug information in tested cases.

**Implication**

A diagnostic tool must distinguish rustc front-end work from code generation,
debug information, and external linker work. Otherwise it will recommend
compiler changes for configuration or linker problems.

**Confidence:** High.

### FERRIUM-09: Cargo workflow and cache topology can repeat otherwise avoidable work

**Source**

- [2025 compiler performance survey](https://blog.rust-lang.org/2025/09/10/rust-compiler-performance-survey-2025-results/)

**Observation**

Reported problems include `cargo check`/`cargo build` cache separation,
rust-analyzer and Cargo competing for the build lock, large CI caches, activated
dependency features, deep workspace graphs, and unnecessary downstream rebuilds.
These are orchestration and artifact-topology issues as much as compiler issues.

**Implication**

FERRIUM's first measurement model should join Cargo graph/timing data with rustc
phase/query evidence. A rustc-only profiler would miss significant user-visible
latency.

**Confidence:** High.

### FERRIUM-10: The Rust project explicitly identifies an interpretation gap

**Sources**

- [2025 compiler performance survey, "Understanding why builds are slow"](https://blog.rust-lang.org/2025/09/10/rust-compiler-performance-survey-2025-results/#understanding-why-builds-are-slow)
- [compiler self-profiling guide](https://rustc-dev-guide.rust-lang.org/profiling.html)
- [`-Zself-profile` reference](https://doc.rust-lang.org/nightly/unstable-book/compiler-flags/self-profile.html)
- [`rustc-perf`](https://github.com/rust-lang/rustc-perf)

**Observation**

Existing official surfaces include `cargo build --timings`, nightly
`-Zself-profile` plus `measureme`, native profilers, and rustc-perf. The survey
states that Cargo timings provide limited information and self-profile output is
hard to interpret without compiler expertise.

The Rust project specifically describes desired tooling that answers:

- Which code recompiled after a source change?
- Which procedural macros take longest or produce the largest output?
- What actionable changes could improve the build?

**Implication**

This is direct evidence for a FERRIUM Hammer opportunity: an external,
explainable build-causality layer grounded in official telemetry. The likely
advantage is interpretation and evidence synthesis, not replacing rustc-perf or
the compiler profiler.

**Confidence:** Very high.

### FERRIUM-11: Structural compiler changes are contributor- and review-constrained

**Source**

- [2025 compiler performance survey, "What's next"](https://blog.rust-lang.org/2025/09/10/rust-compiler-performance-survey-2025-results/#whats-next)

**Observation**

The compiler team attributes the slow delivery of major build improvements to
the required domain knowledge, implementation effort, cross-platform quality
bar, review load, and a limited pool of funded contributors and reviewers.
Large changes such as parallel frontend work, alternative backends, rebuild
avoidance, and faster default linking take years to stabilize.

**Implication**

A downstream compiler fork would inherit substantial maintenance and review
cost while losing upstream compatibility. FERRIUM should collaborate upstream
and use narrow experiments rather than establishing a divergent compiler.

**Confidence:** High.

## Why rustc can feel slow

The evidence supports a layered answer:

1. **Rust asks more questions at compile time.** Type inference, trait solving,
   coherence, borrow checking, pattern analysis, and high-quality diagnostics
   provide safety and expressiveness.
2. **Generics create concrete code.** Monomorphization shifts work to compile
   time to avoid runtime indirection.
3. **LLVM optimizes aggressively.** Release-quality optimization and broad
   target support consume time.
4. **Incrementality is not free.** Dependency tracking, stable hashing,
   persistence, cache loading, and conservative invalidation cost resources.
5. **Crate graphs amplify edits.** Changes to upstream crates can trigger work
   across downstream workspace crates.
6. **Macros and build scripts are compiler inputs with execution costs.**
   Procedural macro expansion is not yet perfectly cached or explained.
7. **Linking repeats global work.** The system linker may dominate short
   incremental rebuilds.
8. **Development defaults carry costs.** Debug information, features, profiles,
   and backend choice materially change latency.
9. **The critical path is not fully parallel.** Some phases remain serial or
   scale poorly under contention.
10. **Users lack causal explanations.** Existing telemetry is fragmented across
    Cargo, rustc, rust-analyzer, backends, and linkers.

## Benchmark protocol proposed for the next research slice

No implementation is authorized by this note. A later measurement pulse should
select representative public fixtures and record:

| Workflow | Stable measurement | Optional nightly depth |
|---|---|---|
| Clean check | `cargo check --timings` | rustc self-profile |
| Clean dev build | `cargo build --timings` | query and backend event summaries |
| One-function edit | repeat check/build timing | invalidated queries and codegen units |
| Public API edit | repeat downstream build timing | dependency-graph propagation |
| Test edit | `cargo test --no-run --timings` | test-harness codegen attribution |
| Link-heavy edit | Cargo link timing | linker-native profile if required |
| Generic-heavy fixture | build timing and binary size | mono-item/LLVM-IR analysis |
| Proc-macro-heavy fixture | build timing | expansion duration and output size |

Each run should record Rust/Cargo versions, target triple, linker, backend,
profile, features, codegen units, incremental setting, debuginfo level, CPU,
memory, storage, cache state, command, wall time, and output size.

## Recommendations

### Adopt now

1. **Owner: FERRIUM Hammer lane.** Continue research on a build-causality and
   explanation contract using existing Cargo timing and rustc self-profile
   outputs.
2. Define a neutral evidence model covering crate graph, phase/query timing,
   invalidation reason, codegen, linking, configuration, and recommendations.
3. Select small, medium, generic-heavy, proc-macro-heavy, and workspace-heavy
   public fixtures before implementation.
4. Treat upstream Rust tooling and documentation as collaboration targets.

**Expected validation:** cited fixture selection, reproducible commands, separate
clean/incremental/check/build/link measurements, and review by Compiler
Performance Engineer, Rust Safety Steward, Ecosystem Strategist, and Validation
Checker.

### Prototype behind a compatibility boundary

After the benchmark and evidence contracts are reviewed, prototype a tool that
ingests exported telemetry rather than linking to `rustc_private`. Keep nightly
self-profile support optional and degrade explicitly when only stable Cargo
timings are available.

**Compatibility boundary:** files and structured command output produced by
Cargo, rustc self-profile tooling, and future supported metrics—not rustc's
unstable in-process APIs.

### Reject or defer

- **Defer a rustc fork.** The compatibility, target, bootstrap, test, and review
  burden is disproportionate to the current research question.
- **Defer a new codegen backend.** LLVM, Cranelift, and GCC efforts already own
  this layer.
- **Defer a custom linker.** First attribute link cost and evaluate existing
  LLD, mold, and Wild work.
- **Defer automatic source rewrites for compile speed.** Recommendations must
  first prove latency gains and expose runtime, binary-size, and maintenance
  tradeoffs.
- **Reject claims that borrow checking alone explains Rust build latency.**
  Bottlenecks are workload-specific and must be measured.

## Non-goals

- Vendoring or modifying rustc during the foundation wave.
- Producing universal benchmark rankings from one repository or machine.
- Treating nightly profiler formats as stable public APIs.
- Trading away safety, diagnostics, runtime performance, or debuggability
  without reporting the cost.
- Presenting compiler acceptance as proof of behavioral correctness.
