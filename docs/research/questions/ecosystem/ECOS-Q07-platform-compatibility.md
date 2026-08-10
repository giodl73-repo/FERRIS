# ECOS-Q07: Platform Compatibility

**Status:** Complete

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

## Decision

Adopt renewable compatibility evidence for exact package, feature and
dependency closure, Cargo/rustc pair, host/target pair, target tier,
`core`/`alloc`/`std` and architecture capability, provider configuration,
native prerequisites, and independently observed resolution, check, link,
execution, and test stages. Preserve expected unsupported, failed,
not-observed, stale, and unknown states. Reject one portable, embedded-ready,
WASM-ready, or MSRV label.

See
[Rust platform compatibility](../../2026-08-09-rust-platform-compatibility.md)
and
[EXP-01](../../ecos-q07-platform-compatibility/results/EXP-01-platform-compatibility-matrix.md).

## Primary roles

Native Platform Adopter, Validation Checker, Interop Boundary Auditor.
