# PERF-Q29: Linking and Incremental Linking

**Status:** Complete

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

## Decision

Add a read-only linker plan and state ledger to the compiler query plan and
labeled Build Forest. Keep complete linking, incremental preparation, reusable
state, object identity, optimization policy, debug packaging, fallback, output
bytes, and release finalization separate.

The measured public Windows executable showed:

- `rust-lld` shortened the complete-link median 4.4%;
- unchanged MSVC incremental linking shortened it 75.5%;
- preparation increased executable bytes 82.5%, PDB bytes 33.7%, and added a
  53.2 MB ILK; and
- one Rust body edit renamed all 181 old root object paths as 182 new paths,
  causing MSVC to perform a full link.

Prioritize stable Rust linker-input identity and upstream collaboration.
Defer automatic linker selection, profile or `/OPT` changes, persistent linker
state, source/CGU changes, CI/editor changes, and a FERRIUM linker.

## Evidence

- [Linking and incremental linking](../2026-08-09-linking-incremental-linking.md)
- [EXP-01 linker matrix](../perf-q29-linking/results/EXP-01-linker-matrix.md)

## Primary roles

Compiler Performance Engineer, Interop Boundary Auditor, Native Platform Adopter.
