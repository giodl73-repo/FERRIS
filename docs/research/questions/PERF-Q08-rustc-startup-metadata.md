# PERF-Q08: rustc Startup and Metadata Loading

**Status:** Complete

**Area:** rustc startup

**Depends on:** PERF-Q01

## Question

How much compiler-invocation latency comes from process startup, crate metadata,
dependency loading, option processing, and initialization before useful queries?

## Starting hypothesis

Startup is material for many small crates and repeated short invocations but is
secondary for large semantic or backend-heavy crates.

## Investigation focus

- Compare tiny-crate and large-crate invocation profiles.
- Attribute metadata decoding and initialization costs.
- Investigate batching, daemon, and selective-loading proposals upstream.

## Evidence plan

1. Measure process launch and direct rustc invocations separately from Cargo.
2. Use tiny no-dependency crates to bound fixed compiler cost.
3. Generate dependencies with controlled metadata size and use both unused and
   semantically referenced `--extern` inputs.
4. Compare metadata-only and codegen-producing outputs.
5. Use rustc self-profile on a separate diagnostic run to identify named query
   and metadata events without making it the primary timing claim.
6. Review current rustc startup, crate loading, metadata decoding, rustc-perf,
   batching, and daemon sources and proposals.
7. Preserve operating-system process creation, filesystem cache, antivirus,
   linker, and command-harness overhead as explicit limitations.

**Model changes if:** startup is negligible across representative short builds.

## Decision informed

Whether startup deserves fixtures, upstream profiling improvements, or daemon
research.

## Decision

Adopt a read-only rustc invocation-floor and metadata-demand vocabulary.
Prototype a portable fixture that separates launcher, crate-root parsing,
expansion, sysroot registration, dependency count, lazy metadata demand,
metadata output, and minimal codegen. Join external wall time to self-profile
events while preserving unclassified time.

Do not build a persistent compiler, daemon, batching layer, shared in-memory
metadata service, or crate-merging optimizer from this evidence. Consider
rustc-perf fixture and telemetry contributions only after explicit owner
approval.

## Results

- [Synthesis](../2026-08-08-rustc-startup-metadata.md)
- [EXP-01: rustc invocation floor and metadata demand](../perf-q08-rustc-startup-metadata/results/EXP-01-invocation-floor-and-metadata.md)

Findings: `FERRIUM-98` through `FERRIUM-107`.

## Primary roles

Compiler Performance Engineer, Ecosystem Strategist, Validation Checker.
