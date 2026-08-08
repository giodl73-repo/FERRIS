# PERF-Q30: Prewarmed and Remote Artifact Provenance

**Status:** Planned

**Area:** Advanced cache

**Depends on:** PERF-Q05, PERF-Q23

## Question

What identity, provenance, trust, transport, platform, and invalidation model is
required to reuse native Rust artifacts produced elsewhere?

## Starting hypothesis

Remote reuse is feasible only after local cross-workspace identity stabilizes;
build scripts, native dependencies, and environment inputs remain major blockers.

## Investigation focus

- Define signed artifact identity and producer/consumer compatibility.
- Test intentional misses, poisoning, revocation, and reproducibility.
- Compare transfer and verification cost with local compilation.

**Model changes if:** practical identity requires capturing an unbounded
environment or transfer costs erase gains.

## Decision informed

Whether remote caching should be prototyped, contributed upstream, or deferred.

## Primary roles

Rust Safety Steward, Native Platform Adopter, AI Assurance Skeptic.
