# PERF-Q31: Function-Level Machine-Code Caching

**Status:** Planned

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
