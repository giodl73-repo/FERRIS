# PERF-Q19: Early-Phase Incrementality

**Status:** Complete

**Area:** Incremental compilation

**Depends on:** PERF-Q09 through PERF-Q11, PERF-Q17

## Question

Which parsing, expansion, resolution, and lowering results can be safely reused
at finer granularity across compiler invocations?

## Starting hypothesis

Earlier phases offer meaningful reuse opportunities but have global ordering,
namespace, hygiene, and diagnostic dependencies that make boundaries harder
than body-oriented queries.

## Investigation focus

- Map upstream designs and unresolved correctness constraints.
- Test module, import, macro, visibility, and body edits.
- Identify fixture shapes that expose broad early-phase recomputation.

**Model changes if:** early-phase cost is too small or globally coupled to repay
incremental bookkeeping.

## Decision informed

Which early-phase incremental initiatives deserve FERRIUM fixtures or funding.

## Decision

Prioritize a high-owner HIR-reconstruction fixture and per-file parsing
research. Retain narrow disk-cached derive expansion and module-item queries as
precedents. Defer general persistent declarative expansion, name resolution,
AST serialization, compiler forks, and upstream activity.

The synthesis and findings are recorded in
[Early-phase incrementality](../2026-08-08-early-phase-incrementality.md).

## Primary roles

Compiler Performance Engineer, Rust Safety Steward, Ecosystem Strategist.
