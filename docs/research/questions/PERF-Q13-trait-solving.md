# PERF-Q13: Trait-Solving Cost and Reuse

**Status:** Planned

**Area:** Semantic analysis

**Depends on:** PERF-Q01, PERF-Q12

## Question

Which trait graphs, bounds, projections, normalization cases, and solver cycles
create disproportionate compile-time work?

## Starting hypothesis

Deep or ambiguous obligation graphs and repeated normalization produce severe
hot spots in a minority of generic-heavy crates.

## Investigation focus

- Study current and next-generation solver architecture and tracking issues.
- Measure obligation counts and query categories on controlled patterns.
- Test whether API simplification, caching, or solver changes address the cause.

**Model changes if:** monomorphization or macro expansion, rather than solving,
dominates generic-heavy examples.

## Decision informed

Define diagnostics, minimized cases, and upstream contribution targets.

## Primary roles

Compiler Performance Engineer, Rust Safety Steward, Ecosystem Strategist.
