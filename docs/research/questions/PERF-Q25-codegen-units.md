# PERF-Q25: Codegen-Unit Partitioning

**Status:** Planned

**Area:** Code generation

**Depends on:** PERF-Q24

## Question

How should codegen-unit partitioning balance parallel compilation, incremental
reuse, optimization quality, memory, and link cost?

## Starting hypothesis

Development builds often benefit from more stable parallel units, while release
builds benefit from fewer units and stronger cross-unit optimization.

## Investigation focus

- Sweep supported codegen-unit settings across workload classes.
- Measure wall time, CPU, memory, binary size, runtime, and incremental reuse.
- Study partition stability after controlled edits.

**Model changes if:** linker or LLVM pass behavior dominates independently of
partitioning.

## Decision informed

Define profile guidance and upstream partitioning fixtures.

## Primary roles

Compiler Performance Engineer, Native Platform Adopter, Validation Checker.
