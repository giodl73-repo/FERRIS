# PERF-Q14: Borrow-Checking Cost and Incrementality

**Status:** Planned

**Area:** Semantic analysis

**Depends on:** PERF-Q01, PERF-Q12

## Question

When does borrow checking materially affect iteration time, and which results
can be reused after body-local edits?

## Starting hypothesis

Borrow checking is essential but usually not the dominant whole-build cost;
pathological control-flow or generated MIR can make it locally significant.

## Investigation focus

- Select borrow-checking-dominant and ordinary controls.
- Separate MIR construction, region inference, and diagnostics.
- Compare body-local edits with query reuse.

**Model changes if:** representative application crates show borrow checking as
a repeated critical path.

## Decision informed

Whether borrow-check visibility or upstream optimization merits dedicated work.

## Primary roles

Rust Safety Steward, Compiler Performance Engineer, Validation Checker.
