# PERF-Q04: Feature, Profile, Target, and Test Multiplication

**Status:** Complete

**Area:** Cargo

**Depends on:** PERF-Q01, PERF-Q02

## Question

How much duplicate work is caused by feature divergence, profile differences,
multiple targets, examples, doctests, benches, and test compilation modes?

## Starting hypothesis

Legitimate identity differences explain some multiplication, but workspace and
CI command composition frequently builds more variants than maintainers expect.

## Investigation focus

- Inventory build units across check, build, test, lint, bench, and release.
- Attribute each duplicate package to an identity difference.
- Test command consolidation and feature alignment.

**Model changes if:** duplicated units are mostly required by correctness or
target semantics.

## Decision informed

Define actionable duplicate-work diagnostics without recommending unsafe
feature unification.

## Primary roles

Compiler Performance Engineer, Rust Maintainer, Validation Checker.

## Result

Completed in
`docs/research/2026-08-08-cargo-build-unit-multiplication.md`.

Package-version duplicates are only one multiplication source. Test, bench,
all-target, feature-role, profile, platform, and compiler-driver variants can
multiply one package-target legitimately. Resolver 2 intentionally compiled a
controlled dependency twice to preserve build-time and runtime feature
semantics; resolver 1 removed a unit by enabling a dev-only feature everywhere
and changed ordinary program output.

FERRIUM should build a read-only unit-variant and observed-artifact diff.
Automatic feature unification, profile merging, target removal, prewarming, and
validation reduction are rejected or deferred.

No upstream issue, comment, branch, or pull request was created.
