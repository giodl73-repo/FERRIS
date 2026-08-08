# PERF-Q03: Cargo Graph Scheduling and Critical Paths

**Status:** Complete

**Area:** Cargo

**Depends on:** PERF-Q01, PERF-Q02

## Question

How much iteration latency comes from dependency topology, ready-queue
scheduling, serial critical paths, and target ordering?

## Starting hypothesis

A small number of high-fan-out or slow crates dominate many workspace builds,
while additional parallelism is unavailable because of graph dependencies.

## Investigation focus

- Compare elapsed time with summed unit time and critical-path estimates.
- Identify high-fan-out crates and avoidable target serialization.
- Test supported scheduling and workspace changes without source rewrites.

**Model changes if:** backend or link work dominates independently of graph
shape.

## Decision informed

Whether FERRIUM should provide a critical-path and graph-topology advisor.

## Primary roles

Compiler Performance Engineer, Rust Maintainer, Scope Keeper.

## Result

Completed in
`docs/research/2026-08-08-cargo-graph-scheduling.md`.

Cargo currently schedules ready units by fixed-cost transitive fan-out, not
measured duration. A controlled fixture showed that a slow direct dependency
could remain ready for 4.627 seconds and start thirteenth at two jobs while
shorter dependency chains advanced first. More jobs reduced queue delay but
showed diminishing wall-clock returns, and manually prebuilding the apparent
gate was slower because it removed overlap.

FERRIUM should adopt read-only queue-delay and observed-gating-chain
explanation. Duration-aware counterfactual simulation belongs behind a
versioned nightly compatibility boundary. Automatic command splitting,
manifest edges, scheduler overrides, and workspace rewrites are deferred.

No upstream issue, comment, branch, or pull request was created.
