# PERF-Q25: Codegen-Unit Partitioning

**Status:** Complete

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

## Decision

Adopt a read-only codegen-unit ledger that separates requested maximum, initial
stable and volatile partitions, merge lineage, actual CGUs, inline-copy
placement, backend work-product reuse, LTO scope, memory, link, size, runtime,
and partition stability. Prototype repository-specific what-if comparison
behind an exact-toolchain boundary. Defer automatic profile changes,
partitioning heuristics, compiler forks, and implementation.

## Evidence

- [Codegen-unit partitioning](../2026-08-09-codegen-unit-partitioning.md)
- [EXP-01 codegen-unit partition and reuse matrix](../perf-q25-codegen-units/results/EXP-01-codegen-unit-partition-matrix.md)

## Primary roles

Compiler Performance Engineer, Native Platform Adopter, Validation Checker.
