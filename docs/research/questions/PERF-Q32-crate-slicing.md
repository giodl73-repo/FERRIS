# PERF-Q32: Crate Slicing and Partial Dependency Compilation

**Status:** Complete

**Area:** Advanced reuse

**Depends on:** PERF-Q17, PERF-Q20, PERF-Q24

## Question

Can rustc compile only the metadata, generic bodies, and machine code actually
consumed from a dependency?

## Answer

Rust's current nightly `hint-mostly-unused` support performs selective codegen
slicing for eligible dependency functions. It can materially improve sparse
public non-generic use, but it leaves whole-crate frontend correctness work in
place and can regress dense or repeated consumer demand.

Generic definitions and private unreachable functions are already
comparatively lazy. Full stub-rlib crate slicing targets a larger frontend and
scheduling opportunity, but the 2026 goal is not accepted and its coherence,
macro, generated-code, dynamic-dispatch, diagnostic, and incremental
boundaries remain compiler-owned and unresolved.

## Investigation focus

- Map which compiler stages require whole-crate knowledge.
- Measure unused dependency surface in representative graphs.
- Study metadata, coherence, codegen, and cache redesign requirements.

**Model changes if:** most dependency cost is unavoidable metadata or downstream
generic work rather than unused implementation.

## Decision informed

Whether to pursue an upstream research proposal or keep crate slicing deferred.

## Decision

Adopt a read-only dependency-surface and partial-compilation eligibility
ledger. Preserve positive, negative, public, and whole-crate correctness
fixtures and contribute evaluation to Cargo issue `#15644`. Defer full crate
slicing, source transformation, compiler forks, automatic profile rewrites,
and implementation.

## Result

The decision, findings, measurements, and role review are recorded in
[crate slicing and partial dependency compilation](../2026-08-09-crate-slicing-partial-compilation.md).

## Primary roles

Rust Safety Steward, Compiler Performance Engineer, Ecosystem Strategist.
