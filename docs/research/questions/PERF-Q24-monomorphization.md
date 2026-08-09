# PERF-Q24: Monomorphization and Generic-Instance Reuse

**Status:** Complete

**Area:** Generics

**Depends on:** PERF-Q01, PERF-Q17

## Question

Which generic APIs and concrete type combinations create disproportionate
mono-item collection, duplicate codegen, optimization work, and binary size?

## Starting hypothesis

Generic-heavy downstream crates repeatedly instantiate compatible code, but
sharing is constrained by inlining, LTO, target, flags, and symbol ownership.

## Investigation focus

- Attribute mono items and LLVM IR to generic definitions and consumers.
- Measure duplicate instances across crates and workspaces.
- Compare polymorphization, API changes, and upstream sharing experiments.

**Model changes if:** duplication is small or runtime optimization losses exceed
compile-time gains.

## Decision informed

Whether to build diagnostics, contribute polymorphization cases, or defer shared
instance caching.

## Decision

Adopt a read-only monomorphization ledger that separates generic families,
concrete instances, owner crates, upstream reuse, emitted symbols, linker
folding, and final retention. Surface non-generic-core candidates for human
review and contribute minimized upstream cases. Defer automatic sharing,
inlining, LTO, dispatch, API, workspace, cache, compiler-fork, and
implementation changes.

## Evidence

- [Monomorphization and generic-instance reuse](../2026-08-09-monomorphization-generic-instance-reuse.md)
- [EXP-01 mono-item growth, sharing, and link-retention matrix](../perf-q24-monomorphization/results/EXP-01-mono-item-sharing-matrix.md)

## Primary roles

Compiler Performance Engineer, Rust Safety Steward, Rust Maintainer.
