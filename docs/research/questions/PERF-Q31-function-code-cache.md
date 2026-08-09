# PERF-Q31: Function-Level Machine-Code Caching

**Status:** Complete

**Area:** Advanced reuse

**Depends on:** PERF-Q18, PERF-Q24 through PERF-Q27

## Question

Can compiled functions be reused across compiler invocations with a complete
identity for MIR, types, target, flags, optimization, debug, and dependencies?

## Starting hypothesis

Function-level reuse has high potential for development backends but requires a
compiler-managed daemon and precise invalidation that external tooling should not
invent independently.

## Investigation focus

- Study upstream daemon and backend cache experiments.
- Define function identity and cross-function optimization boundaries.
- Measure hit potential, memory, lifecycle, debugging, and stale-code risks.

**Model changes if:** identity or optimization coupling makes hit rates too low.

## Decision informed

Whether FERRIUM should supply fixtures, sponsor upstream work, or defer.

## Primary roles

Compiler Performance Engineer, Rust Safety Steward, Ecosystem Strategist.

## Evidence collected

- Cranelift's experimental incremental cache and Wasmtime integration.
- Direct in-memory timing, key, storage, corruption, and equality-check probes.
- A 1,000-function rustc/Cranelift CGU and optimized-MIR fixture.
- A public METIS-CORE one-function control.
- PERF-Q18 and PERF-Q24 through PERF-Q30 identity, partition, backend,
  provenance, debug, and linking evidence.

## Decision

Function-level caching is technically feasible and can recover precision lost
at CGU boundaries for development Cranelift builds. rustc must own the Rust
semantic identity above Cranelift's authoritative function-stencil key.
FERRIUM will maintain fixtures, workload classification, integrity
requirements, and upstream evaluation. It will not build a daemon, external
machine-code cache, rustc fork, LLVM or LTO cache, release path, remote store,
or production integration.

The complete decision is recorded in
`docs/research/2026-08-09-function-level-machine-code-caching.md`.
