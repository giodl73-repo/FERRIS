# PERF-Q04: Feature, Profile, Target, and Test Multiplication

**Status:** Planned

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
