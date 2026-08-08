# PERF-Q05: Cross-Workspace Artifact Reuse

**Status:** Planned

**Area:** Cargo cache

**Depends on:** PERF-Q02, PERF-Q04

## Question

Which ordinary Rust artifacts can be safely reused across workspaces, and what
prevents that reuse today?

## Starting hypothesis

Ordinary non-workspace crates are the safest first reuse level once Cargo has a
stable build-unit identity and cleanup model.

## Investigation focus

- Track upstream Cargo cross-workspace cache design and experiments.
- Measure repeated dependencies across public fixtures.
- Test hits, intentional misses, disk growth, cleanup, and stale-artifact cases.

**Model changes if:** path, metadata, or environment coupling makes practical
reuse too fragile.

## Decision informed

Whether FERRIUM should contribute fixtures, build adapters, or defer to upstream.

## Primary roles

Compiler Performance Engineer, Rust Safety Steward, Ecosystem Strategist.
