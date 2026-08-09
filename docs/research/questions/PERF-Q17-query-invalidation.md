# PERF-Q17: Query Dependency Precision and False Invalidation

**Status:** Complete

**Area:** Incremental compilation

**Depends on:** PERF-Q01, PERF-Q12 through PERF-Q15

## Question

Which rustc query dependencies cause edits to invalidate more semantic work than
their true effect requires?

## Starting hypothesis

Broad aggregate queries and conservative dependencies create repeatable false
invalidation patterns that controlled edit fixtures can expose.

## Investigation focus

- Compare edit semantics with recomputed query categories.
- Minimize broad invalidation cases.
- Distinguish correctness-required propagation from dependency imprecision.

**Model changes if:** observed breadth is mostly required by hidden interface or
optimization dependencies.

## Decision informed

Prioritize external explanations versus targeted upstream query changes.

## Decision

Adopt edit-frontier, provider-execution, result-containment, source-layout, and
diagnostic-dependency vocabulary now. Prototype read-only invalidation and
source-layout visualization behind a nightly compatibility boundary. Defer
span elision, automatic source rewriting, solver selection, compiler forks,
and upstream activity.

The answer and findings are recorded in
[`2026-08-08-query-dependency-precision.md`](../2026-08-08-query-dependency-precision.md).

## Primary roles

Compiler Performance Engineer, Rust Safety Steward, Rust Maintainer.
