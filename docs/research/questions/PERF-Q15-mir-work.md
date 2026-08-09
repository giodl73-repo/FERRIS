# PERF-Q15: MIR Construction and Optimization

**Status:** Complete

**Area:** MIR

**Depends on:** PERF-Q12, PERF-Q14

## Question

Which MIR construction, transformation, optimization, and validation passes
repeat unnecessarily or dominate particular workflows?

## Starting hypothesis

MIR work is usually narrower than frontend crate-wide work but can grow with
large generated bodies and optimization-heavy configurations.

## Investigation focus

- Attribute MIR build, borrow-check, optimization, and validation categories.
- Compare check, debug build, and release workflows.
- Identify pass-level hot spots and invalidation boundaries.

**Model changes if:** backend codegen consistently absorbs the apparent MIR cost.

## Decision informed

Whether to contribute rustc-perf cases or external phase explanations.

## Answer

MIR repetition is primarily successive whole-body traversal and body
multiplication, not duplicate execution of one memoized query. Construction
scaled with one large body's locals and statements. Level-2 inlining,
destination propagation, and GVN became material on 10,000-operation bodies;
partial moves made drop elaboration material; promotions multiplied bodies;
and coroutine state transformation grew with suspension and saved-local
topology.

`optimized_mir` reused cleanly for untouched and identical rewrites. Local
edits stayed local, while a shared const invalidated 1,000 owners. Ordinary
incremental builds disabled MIR inlining; forcing it expanded one helper edit
to 1,002 optimized-MIR misses.

The adopted path is a read-only MIR topology and pass explanation plus
orthogonal rustc-perf candidates. Source rewrites, production MIR flags,
validation reduction, compiler forks, and upstream activity remain closed.

## Evidence

- [Research synthesis](../2026-08-08-mir-construction-optimization.md)
- [Experiment](../perf-q15-mir-work/results/EXP-01-mir-topology-pass-reuse.md)

## Primary roles

Compiler Performance Engineer, Rust Safety Steward, Validation Checker.
