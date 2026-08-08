# PERF-Q13: Trait-Solving Cost and Reuse

**Status:** Complete

**Area:** Semantic analysis

**Depends on:** PERF-Q01, PERF-Q12

## Question

Which trait graphs, bounds, projections, normalization cases, and solver cycles
create disproportionate compile-time work?

## Starting hypothesis

Deep or ambiguous obligation graphs and repeated normalization produce severe
hot spots in a minority of generic-heavy crates.

## Investigation focus

- Study current and next-generation solver architecture and tracking issues.
- Measure obligation counts and query categories on controlled patterns.
- Test whether API simplification, caching, or solver changes address the cause.

**Model changes if:** monomorphization or macro expansion, rather than solving,
dominates generic-heavy examples.

## Decision informed

Define diagnostics, minimized cases, and upstream contribution targets.

## Decision

Adopt solver-mode, canonical-goal, candidate-width, call-count, supertrait,
projection, query-visibility, and impl-set invalidation vocabulary.

Prototype a read-only trait-topology report plus repeated-versus-unique goal,
method-candidate, call-count, structural-depth, solver-mode, and incremental
edit fixtures behind replaceable nightly adapters.

Consider rustc-perf fixtures and solver-aware diagnostic events only after
explicit owner approval. Do not automatically rewrite traits, bounds, methods,
associated types, imports, or generic APIs; enable unstable solver modes in
product builds; implement a custom solver; or create upstream activity.

## Results

- [Synthesis](../2026-08-08-trait-solving-cost-reuse.md)
- [EXP-01: trait topology and solver reuse](../perf-q13-trait-solving/results/EXP-01-trait-topology-solver-reuse.md)

Findings: `FERRIUM-147` through `FERRIUM-156`.

## Primary roles

Compiler Performance Engineer, Rust Safety Steward, Rust Maintainer.
