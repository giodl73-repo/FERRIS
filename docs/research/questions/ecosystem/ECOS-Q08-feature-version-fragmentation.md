# ECOS-Q08: Feature and Version Fragmentation

**Status:** Planned

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

## Primary roles

Compiler Performance Engineer, Ecosystem Strategist, Rust Maintainer.
