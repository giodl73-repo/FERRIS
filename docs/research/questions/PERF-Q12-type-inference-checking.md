# PERF-Q12: Type Inference and Type Checking

**Status:** Planned

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

## Primary roles

Compiler Performance Engineer, Rust Safety Steward, Rust Maintainer.
