# FERRIUM Product Plan

## Thesis

Rust has established the leading integrated platform for memory-safe native
development, but important adoption and productivity gaps remain around mixed
language systems, build latency, AI-generated code assurance, concurrency
debugging, regulated delivery, and accelerator programming.

FERRIUM will first investigate those gaps through cited research and benchmark
design. It will prefer focused tools and compatibility boundaries over a
speculative new general-purpose language. Prototype work begins only after the
foundation research gate selects a bounded problem.

## Product principles

1. **Compiler grounded:** Use Cargo, rustc, rust-analyzer, Clippy, Miri,
   sanitizers, fuzzers, and benchmarks as evidence surfaces.
2. **Boundary first:** Improve existing Rust and native systems before requiring
   wholesale rewrites.
3. **Measured claims:** Establish reproducible baselines before optimization.
4. **Product-neutral cores:** Keep portfolio-specific workflows in adapters or
   consumer repositories.
5. **Evidence carrying:** Preserve commands, versions, hashes, findings, and
   limitations with generated work.

These product principles are governed by the complete
[FERRIUM engineering principles](docs/governance/ENGINEERING_PRINCIPLES.md),
including explicit pitfalls, countermeasures, stop conditions, and the required
prototype gate.

## Waves

### Wave 1: Lab foundation

Establish the research protocol, review roles, opportunity map, and the proposed
shape of a compiler-grounded Ferris evidence contract without implementing it.

### Wave 2: Boundary proof

Select one bounded C++/Rust interoperability problem, establish fixtures and
risk criteria, and prototype a verifiable migration or binding workflow.

### Wave 3: Build intelligence

Measure rebuild causality on representative Rust workspaces and prototype an
explainable recommendation surface for crate graphs, features, linking, and
caching.

The leading opportunity is an evidence-backed build and validation planner that
forecasts a proposed change's build and test blast radius, explains observed
rebuilds, recommends the smallest sufficient validation plan without concealing
risk, diagnoses cache and workspace causes, and emits a Ferris evidence packet.
It presents planned and observed work as a compiler query plan: dependencies,
cacheability, invalidation, expected and actual cost, serial and parallel
regions, and selected reuse.
For cross-crate edits, the plan separates the upstream rebuild, semantic
interface decision, retained-artifact compatibility, downstream compile
pruning, and final relink decision. The evidence and upstream boundary are
defined in
[Relink-Don't-Rebuild and cross-crate interfaces](docs/research/2026-08-08-relink-dont-rebuild.md).
Across check, build, Clippy, test, documentation, and doctest, it separates
exact artifact reuse from compatible compiler-stage reuse, tool-specific work,
coverage-specific work, ephemeral outputs, and execution. That boundary is
defined in
[cross-command artifact reuse](docs/research/2026-08-08-command-artifact-reuse.md).
For procedural macros, it separates native macro execution, invocation
topology, declared and hidden inputs, cached derive output, generated Rust
work, and later compiler cost. The current experimental rustc derive cache is
explicitly rejected because it reused stale output across tracked-input
changes. The observability and compatibility boundary is defined in
[procedural-macro cost, inputs, and reuse](docs/research/2026-08-08-procedural-macro-cost-input-reuse.md).
For build scripts, it separates host compilation, run identity,
package-wide versus declared inputs, hidden inputs, replayed instructions,
generated-output ownership, native metadata, and downstream fan-out. It uses
Cargo's nightly build-analysis as upstream evidence behind a versioned
boundary and keeps caching, unchanged-output suppression, cleanup, and
sandbox enforcement closed. That boundary is defined in
[build-script input, output, and rerun precision](docs/research/2026-08-09-build-script-input-output-precision.md).
For generics, it separates definition families, concrete substitutions,
collection, owner crate, emitted symbols, upstream reuse, sibling and
cross-workspace duplication, linker folding, final retention, and runtime
controls. It treats current rustc generic sharing as dependency-directional
compiler behavior, adds a read-only monomorphization ledger to the compiler
query plan and labeled Build Forest, and keeps API rewriting, automatic
sharing overrides, dispatch changes, and machine-code caching closed. That
boundary is defined in
[monomorphization and generic-instance reuse](docs/research/2026-08-09-monomorphization-generic-instance-reuse.md).
For backend partitioning, it separates requested maximums, initial stable and
volatile partitions, inline local copies, merge lineage, actual CGUs,
pre- and post-LTO work products, memory, link cost, output size, runtime, and
partition stability. It treats one-function precision and generic- or
module-driven merge churn as different edit classes, adds a read-only CGU
ledger to the compiler query plan and Build Forest, and keeps automatic profile
rewrites, partitioning algorithms, module changes, compiler forks, and
implementation closed. That boundary is defined in
[codegen-unit partitioning](docs/research/2026-08-09-codegen-unit-partitioning.md).
For LLVM optimization, it separates IR translation, pre-link optimization,
ThinLTO or fat-LTO work, nested module/function/loop pass events, machine
instruction selection and register allocation, emission, and linking. It joins
those regions to Rust shape, CGUs, exact toolchain, observer-effect
calibration, CPU, memory, final size, and runtime; adds a read-only LLVM cost
ledger to the compiler query plan and Build Forest; and keeps automatic
profile, pass, target-feature, LTO, backend, and source changes closed. That
boundary is defined in
[LLVM optimization cost](docs/research/2026-08-09-llvm-optimization-cost.md).
For development backends, it separates shared frontend and MIR work from
replaceable codegen, target and capability eligibility, backend artifact
identity, panic and failure behavior, clean and incremental outcomes, test
compilation and execution, runtime, and mandatory LLVM validation. It adds a
read-only backend eligibility ledger to the compiler query plan and Build
Forest while keeping automatic profile, repository, CI, editor, release, and
artifact-sharing changes closed. That boundary is defined in
[development codegen backends](docs/research/2026-08-09-development-codegen-backends.md).
For debug information and native emission, it separates rustc debug
construction, effective line, procedure, local, and type capability, LLVM
processing, COFF/DWARF object sections, archives, incremental storage, linker
input, PDB/dSYM/DWP packaging, final stripping, and interactive debugger
validation. It adds a read-only debug-emission ledger to the compiler query
plan and Build Forest while keeping automatic profile, split-debug, strip,
CGU, linker, source, CI, editor, and artifact-sharing changes closed. That
boundary is defined in
[debug information and object emission](docs/research/2026-08-09-debug-information-object-emission.md).
The user-facing abstraction is a debug capability contract covering source
locations, locals, types, profiling symbols, crash symbols, panic and unwind
diagnosis, and mixed-language debugging. Compiler debug levels are measured
implementation choices beneath that contract, not the statement of need.
Its flagship architecture target is a
[labeled Rust Build Forest](docs/research/2026-08-08-rust-build-forest-opportunity.md):
an external control plane of immutable build roots, human labels, lineage,
provenance, reuse, invalidation, validation evidence, and concurrent-session
pressure above Cargo and rustc. It complements rather than replaces their
correctness and cache mechanisms.
The staged capability and research questions are defined in the
[build intelligence research program](docs/plans/BUILD_INTELLIGENCE_RESEARCH_PROGRAM.md).

The component sequence and contribution boundaries are defined in the
[Rust latency component roadmap](docs/research/2026-08-07-rust-latency-component-roadmap.md).
The exact reuse scopes, invalidation boundaries, and external-versus-upstream
interventions are defined in
[Rust incremental reuse scopes and contribution boundaries](docs/research/2026-08-07-rust-incremental-reuse-boundaries.md).
The complete performance backlog is decomposed into 36 independently executable
questions in the
[Rust performance research-question registry](docs/research/questions/README.md).
The cross-program `.roles` checkpoint after PERF-Q20 keeps the implementation
gate closed while accepting the read-only compiler query plan and labeled
Build Forest as the converged direction:
[performance program role checkpoint](docs/research/2026-08-08-performance-program-role-checkpoint.md).
The fixture tiers, workload matrix, edit scenarios, evidence fields, and
prototype gate are defined in the
[build latency measurement contract](docs/specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md).

### Wave 4: Assurance and observability

Prototype evidence packets for generated Rust changes and one async/concurrency
diagnostic surface.

### Cross-cutting ecosystem and library research

Map the boundary between Rust's deliberately compact standard library and the
crates.io application ecosystem. Evaluate capability coverage, foundational
crates, interchange contracts, async portability, maintenance, security,
platform support, feature fragmentation, native dependencies, discovery, and
compatibility-tested stack profiles before considering a curated distribution
or new library implementation.

The questions, evidence model, and intervention gates are defined in the
[ecosystem and library research program](docs/plans/ECOSYSTEM_LIBRARY_RESEARCH_PROGRAM.md).

## Initial consumers and onboarding targets

- Rust maintainers evaluating AI-generated patches.
- C++ teams introducing Rust incrementally.
- Infrastructure teams operating large Cargo workspaces.
- Embedded and regulated teams requiring reproducible assurance evidence.
- Portfolio Rust repositories willing to provide bounded, non-production
  fixtures after the contracts stabilize.

## Success measures

- Every promoted capability begins with a cited research note and benchmark.
- No implementation package is created before a research recommendation names
  its problem, consumer, compatibility boundary, and validation contract.
- Generated patches can name the compiler and validation evidence supporting
  them.
- The first interop proof detects at least one boundary defect that ordinary
  compilation does not explain clearly.
- The first build proof attributes meaningful rebuild cost to specific graph,
  feature, macro, linker, or cache causes.
- A build-impact forecast is evaluated against held-out edits before it can
  influence validation planning.
- Any reduced validation plan names its coverage, uncertainty, mandatory gates,
  and human approval boundary.
- Consumer onboarding does not require TRACKER-relative build paths.

## Non-goals

- Replacing Cargo, rustc, rust-analyzer, or established ecosystem tools without
  measured justification.
- Shipping a new language during the foundation wave.
- Writing product code during the research foundation.
- Providing automatic approval for `unsafe`, cryptographic, medical, automotive,
  aerospace, or other safety-critical code.
- Building proprietary dependencies into the product-neutral core.
- Promising autonomous code correctness.
