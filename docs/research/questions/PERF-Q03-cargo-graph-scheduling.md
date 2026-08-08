# PERF-Q03: Cargo Graph Scheduling and Critical Paths

**Status:** Planned

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
