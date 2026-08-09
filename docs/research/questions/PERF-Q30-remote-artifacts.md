# PERF-Q30: Prewarmed and Remote Artifact Provenance

**Status:** Complete

**Area:** Advanced cache

**Depends on:** PERF-Q05, PERF-Q06, PERF-Q18, PERF-Q23

## Question

What identity, provenance, trust, transport, platform, and invalidation model is
required to reuse native Rust artifacts or atomic incremental generations
produced elsewhere, and which parts can safely participate in a labeled
Rust Build Forest?

## Starting hypothesis

Remote reuse is feasible only after local cross-workspace identity stabilizes;
build scripts, native dependencies, environment inputs, and rustc's unstable
incremental format remain major blockers. Human labels may point to immutable
forest roots, but labels must not weaken artifact identity.

## Investigation focus

- Define signed artifact identity and producer/consumer compatibility.
- Test intentional misses, poisoning, revocation, and reproducibility.
- Compare transfer and verification cost with local compilation.
- Define immutable forest roots, mutable labels, ancestry, pinning, retention,
  and garbage-collection semantics.
- Determine whether complete rustc incremental generations can ever be
  referenced or transported safely without composing their internal files.

**Model changes if:** practical identity requires capturing an unbounded
environment or transfer costs erase gains.

## Decision informed

Whether remote caching and the artifact-bearing portion of the
[Rust Build Forest](../2026-08-08-rust-build-forest-opportunity.md) should be
prototyped, contributed upstream, or deferred.

## Primary roles

Rust Safety Steward, Native Platform Adopter, AI Assurance Skeptic.

## Decision

Adopt signed immutable roots, separate action and content identity, explicit
producer and consumer expectations, atomic publication, isolated installation,
versioned and expiring labels, revocation policy, and reachability retention as
the Rust Build Forest control-plane model.

Authorize a read-only local manifest, policy evaluator, and visualization plus
disposable exact-identity transport experiments. Align reusable Cargo units
with Cargo's upstream cross-workspace cache. Defer a FERRIUM remote artifact
service, automatic restoration, build-script and proc-macro publication,
unknown native execution cones, and cross-platform reuse.

The complete evidence and role review are recorded in
[remote artifact provenance and Rust Build Forest roots](../2026-08-09-remote-artifact-provenance.md).
