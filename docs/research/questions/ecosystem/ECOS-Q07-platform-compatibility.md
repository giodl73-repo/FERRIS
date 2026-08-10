# ECOS-Q07: Platform Compatibility

**Status:** Planned

**Area:** Portability

**Depends on:** ECOS-Q02, ECOS-Q06

## Question

How consistently do foundational crates support MSRV, operating systems,
architectures, `no_std`, WASM, embedded targets, and cross-compilation?

## Starting hypothesis

Top-level support claims can fail in optional features, transitive
dependencies, native providers, tests, examples, or build scripts.

## Decision informed

Define measured platform-compatibility evidence and unsupported-state handling.

## Primary roles

Native Platform Adopter, Validation Checker, Interop Boundary Auditor.
