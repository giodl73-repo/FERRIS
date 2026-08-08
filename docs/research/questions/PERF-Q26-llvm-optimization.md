# PERF-Q26: LLVM Optimization Cost

**Status:** Planned

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

## Primary roles

Compiler Performance Engineer, Rust Maintainer, Validation Checker.
