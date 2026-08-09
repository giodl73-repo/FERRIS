# PERF-Q26: LLVM Optimization Cost

**Status:** Complete

**Area:** Backend

**Depends on:** PERF-Q01, PERF-Q24, PERF-Q25

## Question

Which LLVM passes, IR shapes, optimization levels, and LTO modes dominate Rust
development and release compilation?

## Starting hypothesis

LLVM is a major release-build cost and can dominate generic-heavy development
builds, but the responsible passes and IR patterns vary widely.

## Investigation focus

- Attribute backend time by pass and crate shape.
- Compare optimization levels, LTO, debuginfo, and codegen units.
- Preserve runtime and binary-size tradeoffs in every recommendation.

**Model changes if:** frontend or linking dominates the same workflows.

## Decision informed

Prioritize supported profile guidance, minimized LLVM cases, or backend research.

## Decision

Adopt a read-only LLVM cost ledger that separates IR translation, pre-link and
LTO optimization, nested pass scope, machine passes, emission, and linking.
Join pass cost to Rust shape, exact toolchain, CGU topology, observer effect,
CPU, memory, final size, and runtime. Prototype exact-nightly trace and
self-profile adapters plus isolated profile comparison. Defer automatic
profile, pass, vectorization, LTO, target-feature, backend, and source changes.

## Evidence

- [LLVM optimization cost](../2026-08-09-llvm-optimization-cost.md)
- [EXP-01 LLVM pass cost matrix](../perf-q26-llvm-optimization/results/EXP-01-llvm-pass-cost-matrix.md)

## Primary roles

Compiler Performance Engineer, Rust Maintainer, Validation Checker.
