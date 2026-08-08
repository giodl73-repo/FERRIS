# PERF-Q21: Reuse Across Check, Build, Lint, Test, and Doctest

**Status:** Planned

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

## Primary roles

Compiler Performance Engineer, Rust Maintainer, Validation Checker.
