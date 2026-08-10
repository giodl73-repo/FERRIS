# ECOS-Q08: Feature and Version Fragmentation

**Status:** Complete

**Area:** Dependency graph

**Depends on:** ECOS-Q02, ECOS-Q03, ECOS-Q07

## Question

How much graph duplication, compile cost, binary cost, and incompatibility
comes from feature policies and simultaneous major versions?

## Starting hypothesis

Some multiplicity is required by semver and target needs; other multiplicity
comes from avoidable feature or interchange fragmentation.

## Decision informed

Define evidence-backed fragmentation diagnostics without automatic dependency
rewrites.

## Decision

Adopt typed, renewable fragmentation evidence across package identity,
requesting version constraints, duplicate and shared closure, public exposure,
requested and effective features, resolver/target/dependency-kind scope,
compiler and artifact cost, interchange consequences, and remediation
ownership. Diagnose required, removable, migratory, adapter-owned, deferred,
and unknown multiplicity without automatically widening requirements,
rewriting lockfiles, changing features, or migrating APIs.

See
[Rust feature and version fragmentation](../../2026-08-09-rust-feature-version-fragmentation.md)
and
[EXP-01](../../ecos-q08-feature-version-fragmentation/results/EXP-01-fragmentation-cost-matrix.md).

## Primary roles

Compiler Performance Engineer, Ecosystem Strategist, Rust Maintainer.
