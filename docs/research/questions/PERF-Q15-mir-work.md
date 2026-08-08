# PERF-Q15: MIR Construction and Optimization

**Status:** Planned

**Area:** MIR

**Depends on:** PERF-Q12, PERF-Q14

## Question

Which MIR construction, transformation, optimization, and validation passes
repeat unnecessarily or dominate particular workflows?

## Starting hypothesis

MIR work is usually narrower than frontend crate-wide work but can grow with
large generated bodies and optimization-heavy configurations.

## Investigation focus

- Attribute MIR build, borrow-check, optimization, and validation categories.
- Compare check, debug build, and release workflows.
- Identify pass-level hot spots and invalidation boundaries.

**Model changes if:** backend codegen consistently absorbs the apparent MIR cost.

## Decision informed

Whether to contribute rustc-perf cases or external phase explanations.

## Primary roles

Compiler Performance Engineer, Rust Safety Steward, Validation Checker.
