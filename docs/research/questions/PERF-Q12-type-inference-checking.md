# PERF-Q12: Type Inference and Type Checking

**Status:** Complete

**Area:** Semantic analysis

**Depends on:** PERF-Q01, PERF-Q11

## Question

Which language and API patterns cause type inference and type checking to
consume disproportionate time or invalidate broadly?

## Starting hypothesis

Complex inference obligations, large expressions, coercions, and generated code
produce localized but significant semantic hot spots.

## Investigation focus

- Identify type-checking-dominant rustc-perf and portfolio fixtures.
- Minimize expensive expressions without changing their semantics.
- Separate inference cost from trait solving and borrow checking.

**Model changes if:** trait solving or macro-generated volume explains the
apparent type-checking cost.

## Decision informed

Whether FERRIUM should expose source-level hot spots or contribute compiler cases.

## Decision

Adopt item-WF, body-owner, inference-variable, expected-type, coercion,
pattern, fallback, writeback, result-hash, owner-width, edit-dependency, and
trait-obligation vocabulary.

Prototype a read-only per-owner type-checking report plus trait-light generic,
coercion, pattern, expected-type, owner-topology, frontend-job, and
incremental-edit fixtures behind replaceable nightly adapters.

Consider rustc-perf fixtures and finer intra-body profile events only after
explicit owner approval. Do not automatically add annotations, split
functions, change aliases or generic APIs, replace inference, or create
upstream activity.

## Results

- [Synthesis](../2026-08-08-type-inference-checking.md)
- [EXP-01: inference, owner topology, coercion, and invalidation](../perf-q12-type-inference-checking/results/EXP-01-inference-owner-coercion-invalidation.md)

Findings: `FERRIUM-137` through `FERRIUM-146`.

## Primary roles

Compiler Performance Engineer, Rust Safety Steward, Rust Maintainer.
