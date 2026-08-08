# PERF-Q02: Cargo Build-Unit Identity

**Status:** Planned

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
