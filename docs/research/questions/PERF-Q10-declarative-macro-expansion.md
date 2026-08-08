# PERF-Q10: Declarative Macro Expansion

**Status:** Planned

**Area:** Frontend

**Depends on:** PERF-Q01, PERF-Q09

## Question

Which declarative macro patterns create disproportionate expansion, generated
syntax, diagnostics, or invalidation?

## Starting hypothesis

Large expansions and repeated matching can create frontend critical paths whose
cost is poorly visible to crate maintainers.

## Investigation focus

- Measure expansion time and generated token volume.
- Compare local macro edits with downstream invalidation.
- Identify diagnostic and visibility improvements before source recommendations.

**Model changes if:** procedural execution or later type checking explains most
macro-associated cost.

## Decision informed

Define external macro-cost diagnostics and candidate upstream profiler cases.

## Primary roles

Compiler Performance Engineer, Rust Maintainer, AI Assurance Skeptic.
