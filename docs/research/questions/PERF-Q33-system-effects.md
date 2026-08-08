# PERF-Q33: Filesystem, Memory, Virtualization, and Hardware Effects

**Status:** Planned

**Area:** System effects

**Depends on:** PERF-Q01

## Question

How much Rust latency variation comes from storage, memory pressure, antivirus,
indexing, virtualization, thermal state, and CPU topology rather than compiler
algorithms?

## Starting hypothesis

Incremental caches, metadata, object files, and linking make Rust builds
sensitive to storage and memory conditions, especially on constrained systems.

## Investigation focus

- Record system conditions and controlled environment comparisons.
- Separate reproducible compiler work from environmental variance.
- Define environment warnings without prescribing unsafe exclusions.

**Model changes if:** variation remains small relative to compiler-component
differences.

## Decision informed

Define benchmark controls and environment diagnostics.

## Primary roles

Compiler Performance Engineer, Native Platform Adopter, Validation Checker.
