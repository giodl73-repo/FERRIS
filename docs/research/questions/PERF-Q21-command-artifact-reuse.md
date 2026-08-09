# PERF-Q21: Reuse Across Check, Build, Lint, Test, and Doctest

**Status:** Complete

**Area:** Artifact reuse

**Depends on:** PERF-Q02, PERF-Q04, PERF-Q17

## Question

Which semantic and codegen results can be reused across Cargo check, build,
Clippy, tests, examples, benches, and doctests?

## Starting hypothesis

Different target and compiler modes require some separation, but current
workflows repeat compatible frontend and dependency work.

## Investigation focus

- Trace artifact identities across common command sequences.
- Distinguish required compiler-mode differences from orchestration duplication.
- Review shared-artifact upstream goals and correctness tests.

**Model changes if:** mode-specific semantics make most apparent duplication
non-reusable.

## Decision informed

Whether to prioritize Cargo integration, validation planning, or documentation.

## Decision

Adopt activity, target-stage, stage-dependency, exact-artifact,
compatible-dependency, tool-specific, coverage-specific, and ephemeral-output
vocabulary. Align compiler mechanism work with Rust's accepted Incremental
Systems Rethought goal. Retain read-only cross-command explanation and defer a
FERRIUM incremental compiler, artifact aliasing, command substitution, or
validation reduction.

The answer and evidence are recorded in:

- `docs/research/2026-08-08-command-artifact-reuse.md`;
- `docs/research/perf-q21-command-artifact-reuse/results/EXP-01-command-reuse-matrix.md`.

## Primary roles

Compiler Performance Engineer, Rust Maintainer, Validation Checker.
