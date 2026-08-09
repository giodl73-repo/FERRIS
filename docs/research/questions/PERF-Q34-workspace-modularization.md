# PERF-Q34: Workspace Modularization and Crate Boundaries

**Status:** Complete

**Area:** Repository architecture

**Depends on:** PERF-Q03, PERF-Q17, PERF-Q20, PERF-Q24

## Question

When does splitting, combining, or isolating crates improve total iteration
time, and when does it add metadata, startup, generic, and link overhead?

## Starting hypothesis

Crate boundaries improve incremental isolation only when they align with stable
change boundaries; excessive fragmentation increases invocation and graph costs.

## Investigation focus

- Compare matched monolith and workspace controls.
- Model fan-out, edit locality, generic boundaries, startup, and linking.
- Test reversible topology changes in synthetic fixtures only.

**Model changes if:** no repeatable topology patterns transfer across
repositories.

## Decision informed

Whether FERRIUM can provide a measured modularization advisor.

## Decision

Adopt a read-only, workload-weighted crate-boundary ledger and
counterfactual-advisor model. Measure sibling width, serial depth, edit
containment, downstream fan-out, rustc invocation and metadata multiplication,
generic ownership, test and link targets, storage, and non-performance
boundary reasons. Prototype only through disposable counterfactuals and
held-out repository evaluation. Reject automatic crate splitting, combining,
source movement, manifest rewrites, API redesign, and universal crate-count
guidance.

See
[Workspace modularization and crate boundaries](../2026-08-09-workspace-modularization-crate-boundaries.md)
and the
[crate-boundary response matrix](../perf-q34-workspace-modularization/results/EXP-01-crate-boundary-matrix.md).

## Primary roles

Compiler Performance Engineer, Rust Maintainer, Scope Keeper.
