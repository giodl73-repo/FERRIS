# PERF-Q16: Frontend Parallelism

**Status:** Planned

**Area:** Parallelism

**Depends on:** PERF-Q08 through PERF-Q15

## Question

Where does parallel rustc frontend execution improve latency, and where do
contention, overhead, or serial dependencies limit scaling?

## Starting hypothesis

Large crates with independent semantic work benefit most, while small crates and
global frontend phases may regress or scale poorly.

## Investigation focus

- Compare supported serial and parallel configurations.
- Measure wall time, CPU, memory, contention, and correctness.
- Add representative cases to upstream testing and rustc-perf.

**Model changes if:** memory or synchronization cost outweighs gains on most
representative crates.

## Decision informed

Which workloads and upstream goals FERRIUM should support.

## Primary roles

Compiler Performance Engineer, Native Platform Adopter, Validation Checker.
