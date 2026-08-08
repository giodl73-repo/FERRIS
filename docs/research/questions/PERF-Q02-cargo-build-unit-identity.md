# PERF-Q02: Cargo Build-Unit Identity

**Status:** Complete

**Area:** Cargo

**Depends on:** PERF-Q01

## Question

Which inputs make two Cargo package-target compilations identical, compatible,
or necessarily distinct?

## Starting hypothesis

Toolchain, target, profile, features, rustflags, dependency identities, source,
and build-script outputs explain most missed artifact reuse.

## Investigation focus

- Enumerate identity inputs from Cargo sources and controlled builds.
- Change one identity dimension at a time.
- Separate required invalidation from accidental path or workspace coupling.

**Model changes if:** apparently identical units differ for undocumented or
unobservable reasons.

## Decision informed

Define the cache-identity model used by analysis and later cache experiments.

## Primary roles

Compiler Performance Engineer, Rust Safety Steward, Validation Checker.

## Result

Completed in
`docs/research/2026-08-07-cargo-build-unit-identity.md`.

Cargo identity is layered:

- the graph unit identifies work inside an invocation;
- `Metadata::unit_id` identifies the artifact namespace;
- `Metadata::c_metadata` identifies symbol disambiguation;
- the fingerprint decides whether the artifact is fresh.

FERRIUM should build read-only identity and session comparison before any cache
intervention. Shared writable target directories across unrelated repositories
are explicitly rejected because current local path-package identities can
collide. Nightly Cargo build analysis is a prototype compatibility boundary,
not a stable dependency.

No upstream issue, comment, branch, or pull request was created.
