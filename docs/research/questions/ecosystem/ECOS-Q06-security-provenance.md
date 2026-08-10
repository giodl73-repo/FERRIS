# ECOS-Q06: Security and Provenance

**Status:** Complete

**Area:** Dependency assurance

**Depends on:** ECOS-Q02, ECOS-Q05

## Question

What evidence covers advisories, integrity, unsafe code, build scripts,
procedural macros, native code, licensing, and release identity?

## Starting hypothesis

No single metadata source proves dependency safety; assurance requires joined
evidence and explicit unknowns across the complete closure.

## Decision informed

Define evidence and trust boundaries without inventing a universal crate
safety score.

## Decision

Adopt joined, renewable assurance evidence across archive identity, source
revision, publication authority, advisory snapshots, active closure,
compile-time execution, unsafe and native boundaries, review attestations,
licensing, and explicit unknowns. Reject one crate safety score and
success-shaped conclusions from zero advisory matches, zero direct unsafe
syntax, or absent Cargo `links`.

See
[Rust security and provenance](../../2026-08-09-rust-security-provenance.md)
and
[EXP-01](../../ecos-q06-security-provenance/results/EXP-01-security-provenance-census.md).

## Primary roles

Rust Safety Steward, AI Assurance Skeptic, Native Platform Adopter.
