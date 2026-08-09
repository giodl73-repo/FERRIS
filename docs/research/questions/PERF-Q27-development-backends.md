# PERF-Q27: Development Codegen Backends

**Status:** Complete

**Area:** Backend

**Depends on:** PERF-Q01, PERF-Q26

## Question

For which repositories and workflows does Cranelift or another supported
development backend improve trustworthy iteration time?

## Starting hypothesis

Cranelift can materially improve debug codegen for supported targets, while
LLVM remains necessary for release optimization and some platform coverage.

## Investigation focus

- Compare check, debug build, test, and representative runtime behavior.
- Record target, feature, debug, and diagnostic limitations.
- Evaluate backend switching as configuration, not a FERRIUM backend.

**Model changes if:** frontend and linking erase backend gains or compatibility
gaps affect common fixtures.

## Decision informed

Define evidence-based backend selection guidance and upstream fixtures.

## Decision

Treat Cranelift as a nightly, target- and workflow-specific development backend
candidate. Add a read-only backend eligibility and outcome ledger that records
exact component identity, isolated roots, clean and incremental outcomes,
runtime, panic, failure, target, intrinsic, ABI, debug, and LLVM validation
controls. Prototype advisory comparison only. Defer automatic configuration,
CI defaults, mixed-backend artifact reuse, release use, and a FERRIUM backend.

## Evidence

- [Development codegen backends](../2026-08-09-development-codegen-backends.md)
- [EXP-01 development backend matrix](../perf-q27-development-backends/results/EXP-01-development-backend-matrix.md)

## Primary roles

Compiler Performance Engineer, Rust Safety Steward, Native Platform Adopter.
