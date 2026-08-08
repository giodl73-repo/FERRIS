# PERF-Q06: CI Cache Topology and Duplicate Work

**Status:** Planned

**Area:** CI

**Depends on:** PERF-Q02, PERF-Q05

## Question

Which CI job, branch, profile, target, runner, and cache-key boundaries discard
compatible Rust work or retain excessive incompatible artifacts?

## Starting hypothesis

CI cache keys are often either too broad for correctness or too narrow for
reuse, causing low hit rates and large caches.

## Investigation focus

- Model cache identity separately from transport and retention.
- Compare job graphs and artifact overlap on public fixtures.
- Test cache restoration cost, hit rate, corruption behavior, and cleanup.

**Model changes if:** compilation is cheap relative to transfer and cache
management.

## Decision informed

Define a CI cache diagnosis surface and evidence-based key recommendations.

## Primary roles

Compiler Performance Engineer, Native Platform Adopter, Validation Checker.
