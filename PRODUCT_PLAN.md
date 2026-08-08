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
The staged capability and research questions are defined in the
[build intelligence research program](docs/plans/BUILD_INTELLIGENCE_RESEARCH_PROGRAM.md).

The component sequence and contribution boundaries are defined in the
[Rust latency component roadmap](docs/research/2026-08-07-rust-latency-component-roadmap.md).
The exact reuse scopes, invalidation boundaries, and external-versus-upstream
interventions are defined in
[Rust incremental reuse scopes and contribution boundaries](docs/research/2026-08-07-rust-incremental-reuse-boundaries.md).
The fixture tiers, workload matrix, edit scenarios, evidence fields, and
prototype gate are defined in the
[build latency measurement contract](docs/specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md).

### Wave 4: Assurance and observability

Prototype evidence packets for generated Rust changes and one async/concurrency
diagnostic surface.

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
