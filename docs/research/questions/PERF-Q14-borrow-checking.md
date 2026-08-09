# PERF-Q14: Borrow-Checking Cost and Incrementality

**Status:** Complete

**Area:** Semantic analysis

**Depends on:** PERF-Q01, PERF-Q12

## Question

When does borrow checking materially affect iteration time, and which results
can be reused after body-local edits?

## Starting hypothesis

Borrow checking is essential but usually not the dominant whole-build cost;
pathological control-flow or generated MIR can make it locally significant.

## Investigation focus

- Select borrow-checking-dominant and ordinary controls.
- Separate MIR construction, region inference, and diagnostics.
- Compare body-local edits with query reuse.

**Model changes if:** representative application crates show borrow checking as
a repeated critical path.

## Decision informed

Whether borrow-check visibility or upstream optimization merits dedicated work.

## Decision

Adopt promoted-MIR, move-path, loan-lifetime, place-conflict,
region-constraint, CFG, nested-body, Polonius-mode, and borrow-edit
vocabulary.

Prototype a read-only per-owner borrow-topology report plus immediate versus
overlapping loans, reborrow, owner, CFG, move, closure, await, incremental,
and diagnostic fixtures behind replaceable nightly adapters.

Consider rustc-perf fixtures and finer borrow-check counters only after
explicit owner approval. Do not automatically shorten borrows, clone values,
change ownership or lifetimes, split functions, add `unsafe`, enable
experimental Polonius modes, implement a custom checker, or create upstream
activity.

## Results

- [Synthesis](../2026-08-08-borrow-checking-cost-incrementality.md)
- [EXP-01: loan, region, move, and incremental topology](../perf-q14-borrow-checking/results/EXP-01-loan-region-move-incrementality.md)

Findings: `FERRIUM-157` through `FERRIUM-166`.

## Primary roles

Rust Safety Steward, Compiler Performance Engineer, Validation Checker.
