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
For function-level reuse, it separates rustc's semantic and monomorphized
identity from Cranelift's function-stencil key, function-specific finalization
parameters, cache admission, population, hit, restoration, integrity, and
daemon lifecycle. It adds a read-only function-cache opportunity and evidence
ledger to the compiler query plan while keeping a FERRIUM daemon, external
machine-code store, rustc integration, LLVM, LTO, release, persistence,
transport, and automatic restoration closed. That boundary is defined in
[function-level machine-code caching](docs/research/2026-08-09-function-level-machine-code-caching.md).
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
For linking, it separates emitted object and library identity, link-plan
construction, complete-link engines, incremental-state eligibility and reuse,
optimization policy, PDB or platform debug packaging, fallback, output bytes,
release finalization, and validation. It adds a read-only linker plan and state
ledger while keeping automatic linker, profile, `/OPT`, CGU, source, CI,
editor, ILK-lifecycle, and release changes closed. The decisive Windows public
control found a 75.5% unchanged incremental-link gain, but one Rust body edit
renamed the complete root-object set and forced a full link. Stable linker-input
identity and upstream collaboration therefore precede configuration automation.
That boundary is defined in
[linking and incremental linking](docs/research/2026-08-09-linking-incremental-linking.md).
The user-facing link capability contract covers target and ABI compatibility,
debug and symbol packaging, edit-to-runnable latency, release optimization and
finalization, reproducibility, native libraries and mixed-language support,
signing, deployment, and rollback. Linker names and flags are measured
implementation choices beneath that contract.
Its flagship architecture target is a
[labeled Rust Build Forest](docs/research/2026-08-08-rust-build-forest-opportunity.md):
an external control plane of immutable build roots, human labels, lineage,
provenance, reuse, invalidation, validation evidence, and concurrent-session
pressure above Cargo and rustc. It complements rather than replaces their
correctness and cache mechanisms.
PERF-Q30 closes the read-only forest provenance gate with signed immutable
roots, versioned and expiring mutable labels, separate action and content
identity, producer and consumer trust expectations, atomic publication,
isolated installation, revocation, and reachability retention. It authorizes a
local manifest, policy, and visualization prototype plus disposable
exact-identity transport experiments, while keeping a production remote cache,
automatic restoration, execution-cone artifacts, and cross-platform reuse
closed. That boundary is defined in
[remote artifact provenance and Rust Build Forest roots](docs/research/2026-08-09-remote-artifact-provenance.md).
PERF-Q31 closes the function-reuse research gate with a compiler-owned Rust
semantic envelope, Cranelift-owned stencil identity, measured admission and
restoration economics, corruption controls, and explicit optimization and
capability boundaries. It authorizes fixtures and upstream evaluation while
keeping implementation and artifact restoration closed. That boundary is
defined in
[function-level machine-code caching](docs/research/2026-08-09-function-level-machine-code-caching.md).
PERF-Q32 closes the partial-dependency research gate with declared-surface,
consumer-demand, whole-crate frontend, dependency-codegen, consumer-codegen,
duplication, inline-policy, and final-retention boundaries. It treats
Cargo's nightly `hint-mostly-unused` support as selective codegen slicing,
authorizes read-only candidate comparison and upstream evaluation, and keeps
full crate slicing, source transformation, automatic manifest changes,
compiler forks, and implementation closed. That boundary is defined in
[crate slicing and partial dependency compilation](docs/research/2026-08-09-crate-slicing-partial-compilation.md).
PERF-Q33 closes the system-effects research gate with host, VM, filesystem,
source, target, cache-layer, CPU-job, memory-reserve, session-pressure,
security, indexing, power, thermal, and attribution-confidence boundaries. It
authorizes read-only environment fingerprints and supported diagnostics while
keeping security exclusions, service and power changes, forced pressure,
repository migration, universal job tuning, and host automation closed. That
boundary is defined in
[system effects on Rust build latency](docs/research/2026-08-09-system-effects-build-latency.md).
PERF-Q34 closes the workspace-modularization research gate with logical,
compilation, package, workspace, parallel-width, serial-depth,
edit-containment, downstream-fan-out, invocation, metadata, generic-ownership,
test, link, storage, boundary-stability, and non-performance constraints. It
authorizes a read-only crate-boundary ledger and disposable counterfactual
evaluation while keeping automatic splitting, combining, source movement,
manifest changes, API redesign, and universal crate-count guidance closed.
That boundary is defined in
[workspace modularization and crate boundaries](docs/research/2026-08-09-workspace-modularization-crate-boundaries.md).
PERF-Q35 closes the impact-aware validation research gate with separate
package, activity, feature, target, profile, doctest, execution, repository
gate, declared-input, uncertainty, fallback, and evidence scopes. A
conservative synthetic selector preserved all eight seeded failure classes and
reduced the warm median 57.1%; a public PARLOR control preserved its documented
contract with a bounded 9.4% gain. FERRIUM now authorizes a read-only
validation-plan and coverage ledger plus a bounded package-selection
prototype, while keeping automatic gate deletion, unknown-file skipping,
full-suite confidence claims, and required-CI replacement closed. That
boundary is defined in
[impact-aware validation selection](docs/research/2026-08-09-impact-aware-validation-selection.md).
PERF-Q36 closes the 36-question Rust performance research sequence and opens a
contribution-first Phase 4. FERRIUM adopts a standard upstream performance
contribution packet and selects a rustc-perf-compatible
Relink-Don't-Rebuild body-versus-interface benchmark as the first target.
Owner alignment, Linux reproduction, stable upstream metrics, licensing,
maintenance, and explicit approval precede any external issue, comment, or
pull request. Research completion does not open the FERRIUM implementation
gate. That boundary is defined in the
[Rust performance contribution program closeout](docs/research/2026-08-09-rust-performance-contribution-program-closeout.md)
and
[contribution packet contract](docs/specs/RUST_PERFORMANCE_CONTRIBUTION_PACKET.md).
The staged capability and research questions are defined in the
[build intelligence research program](docs/plans/BUILD_INTELLIGENCE_RESEARCH_PROGRAM.md).

The component sequence and contribution boundaries are defined in the
[Rust latency component roadmap](docs/research/2026-08-07-rust-latency-component-roadmap.md).
The exact reuse scopes, invalidation boundaries, and external-versus-upstream
interventions are defined in
[Rust incremental reuse scopes and contribution boundaries](docs/research/2026-08-07-rust-incremental-reuse-boundaries.md).
The completed initial performance backlog contains 36 independently executable
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

This sequence is now named the **Crates Series** and is the next research
priority. ECOS-Q01 through ECOS-Q12 must complete before any OSPREY
implementation or other build-intelligence product code begins.

ECOS-Q01 establishes five coverage classes: Guaranteed, Official, Ecosystem
available, Fragmented, and Material gap. It finds broad capability outside
`std`, with the primary unresolved problem in interchange, provider, platform,
data, stewardship, security, and lifecycle governance rather than wholesale
library absence. Individual crate ranking remains closed until ECOS-Q02. The
decision is recorded in
[Rust capability coverage](docs/research/2026-08-09-rust-capability-coverage.md).

ECOS-Q02 defines foundational status by structural role rather than popularity.
It selects nineteen exact releases across contract, construction, platform,
build, and implementation-substrate roles for deeper ECOS-Q03 through
ECOS-Q09 verification. The set is explicitly not an approved stack or
dependency recommendation. See
[Rust foundational crate census](docs/research/2026-08-09-rust-foundational-crate-census.md).

ECOS-Q03 proves that composition must be represented in layers: exact package
and re-export identity, trait coherence, effective features, adapter ownership,
conversion loss, semantic preservation, and runtime behavior. Duplicate HTTP,
rand_core, and syn versions failed at exposed type or trait boundaries, while
Serde's facade/core re-export and typed-error aggregation composed. See
[Rust interchange contracts](docs/research/2026-08-09-rust-interchange-contracts.md).

ECOS-Q04 establishes operation-level async contracts. Future, executor, spawn,
I/O, time, cancellation, blocking work, synchronization, context, and platform
must remain separate. Measured fixtures distinguish runtime-neutral futures
from context-panicking spawn/timer operations, nominal I/O traits, explicit
adapters, and task-handle lifecycle. See
[Rust async portability](docs/research/2026-08-09-rust-async-portability.md).

### OSPREY: Query Forest and Build Intelligence

OSPREY means **Observe, Show, Predict, Resolve, Execute, Yield**.

It organizes the compiler query plan, labeled Build Forest, validation plan,
ecosystem dependency model, controlled actions, and FERRIS evidence packets
into one architecture. OSPREY planning begins with specifications, schemas,
held-out workflow design, and role review—not code.

The detailed architecture planes, phases, predecessor gates, required plans,
success measures, and non-goals are defined in the
[OSPREY program](docs/plans/OSPREY_PROGRAM.md).
The Forest is decomposed into adapters, a canonical typed graph, maps, ledgers,
plans and records, replaceable engines, and bounded views in the
[Query Forest component model](docs/specs/FOREST_COMPONENT_MODEL.md).
This is registered as FOREST-001. The complete planned normative sequence
from schema and identity through actions, evidence, views, and conformance is
tracked in the
[FERRIUM specification registry](docs/specs/README.md).

Program order:

```text
PERF-Q01 through PERF-Q36
  -> Crates Series ECOS-Q01 through ECOS-Q12
    -> OSPREY Query Forest architecture
      -> held-out maintainer workflow
        -> separately approved bounded prototype
```

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
