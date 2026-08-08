# PERF-Q24: Monomorphization and Generic-Instance Reuse

**Status:** Planned

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

## Primary roles

Compiler Performance Engineer, Rust Safety Steward, Rust Maintainer.
