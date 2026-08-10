# ECOS-Q06: Security and Provenance

**Status:** Planned

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

## Primary roles

Rust Safety Steward, AI Assurance Skeptic, Native Platform Adopter.
