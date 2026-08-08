# PERF-Q34: Workspace Modularization and Crate Boundaries

**Status:** Planned

**Area:** Repository architecture

**Depends on:** PERF-Q03, PERF-Q17, PERF-Q20, PERF-Q24

## Question

When does splitting, combining, or isolating crates improve total iteration
time, and when does it add metadata, startup, generic, and link overhead?

## Starting hypothesis

Crate boundaries improve incremental isolation only when they align with stable
change boundaries; excessive fragmentation increases invocation and graph costs.

## Investigation focus

- Compare matched monolith and workspace controls.
- Model fan-out, edit locality, generic boundaries, startup, and linking.
- Test reversible topology changes in synthetic fixtures only.

**Model changes if:** no repeatable topology patterns transfer across
repositories.

## Decision informed

Whether FERRIUM can provide a measured modularization advisor.

## Primary roles

Compiler Performance Engineer, Rust Maintainer, Scope Keeper.
