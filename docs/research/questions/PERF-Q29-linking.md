# PERF-Q29: Linking and Incremental Linking

**Status:** Planned

**Area:** Linking

**Depends on:** PERF-Q01, PERF-Q28

## Question

When does native linking dominate Rust iteration time, and which existing
linkers or incremental-linking designs reduce it safely?

## Starting hypothesis

Large binaries, native dependencies, and debug data make linking a dominant
post-compiler cost that rustc query improvements cannot solve.

## Investigation focus

- Isolate compile, object, and link time across representative binaries.
- Compare supported linker configurations and incremental behavior.
- Test relink-only edits, correctness, portability, and diagnostics.

**Model changes if:** object generation rather than linker execution dominates
the apparent link phase.

## Decision informed

Choose configuration guidance, upstream fixtures, or collaboration with
existing incremental-linker projects.

## Primary roles

Compiler Performance Engineer, Interop Boundary Auditor, Native Platform Adopter.
