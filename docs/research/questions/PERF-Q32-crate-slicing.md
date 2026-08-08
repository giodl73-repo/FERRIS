# PERF-Q32: Crate Slicing and Partial Dependency Compilation

**Status:** Planned

**Area:** Advanced reuse

**Depends on:** PERF-Q17, PERF-Q20, PERF-Q24

## Question

Can rustc compile only the metadata, generic bodies, and machine code actually
consumed from a dependency?

## Starting hypothesis

Crate slicing could remove substantial unused work but conflicts with
whole-crate metadata, coherence, macros, codegen, and incremental models.

## Investigation focus

- Map which compiler stages require whole-crate knowledge.
- Measure unused dependency surface in representative graphs.
- Study metadata, coherence, codegen, and cache redesign requirements.

**Model changes if:** most dependency cost is unavoidable metadata or downstream
generic work rather than unused implementation.

## Decision informed

Whether to pursue an upstream research proposal or keep crate slicing deferred.

## Primary roles

Rust Safety Steward, Compiler Performance Engineer, Ecosystem Strategist.
