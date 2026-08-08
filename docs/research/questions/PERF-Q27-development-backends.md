# PERF-Q27: Development Codegen Backends

**Status:** Planned

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

## Primary roles

Compiler Performance Engineer, Rust Safety Steward, Native Platform Adopter.
