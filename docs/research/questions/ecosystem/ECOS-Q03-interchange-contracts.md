# ECOS-Q03: Interchange Contracts

**Status:** Complete

**Area:** Composition

**Depends on:** ECOS-Q01, ECOS-Q02

## Question

Where do competing types, traits, error models, and feature policies prevent
otherwise mature crates from composing?

## Starting hypothesis

Rust often has multiple capable implementations but lacks stable interchange
contracts at runtime, I/O, error, identity, time, TLS, database, and telemetry
boundaries.

## Decision informed

Whether documentation, shared traits, adapters, compatibility tests, or
upstream standardization are justified.

## Primary roles

Interop Boundary Auditor, Ecosystem Strategist, Rust Maintainer.

## Decision

Adopt a layered interchange model covering exact and re-export identity, trait
coherence, conversion, wrappers, serialization, effective features, semantic
preservation, and runtime behavior.

Measured fixtures verified Serde facade/core identity and typed-error
aggregation, while duplicate HTTP, rand_core, and syn versions failed at their
public type or trait boundaries. Rust coherence requires upstream ownership,
an adapter crate, a local trait, or a local newtype for many bridges.

See
[Rust interchange contracts](../../2026-08-09-rust-interchange-contracts.md).
