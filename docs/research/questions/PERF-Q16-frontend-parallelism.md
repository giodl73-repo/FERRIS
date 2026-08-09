# PERF-Q16: Frontend Parallelism

**Status:** Complete

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

## Answer

Frontend jobs improved crates with thousands of independently schedulable
semantic owners, but not tiny crates, parsing/expansion boundaries, or one
large body. Four to eight jobs captured most useful gains while increasing CPU
and peak memory; 16 jobs often added resource cost without improving wall
time.

Cargo's inherited jobserver coordinates workers inside one build process tree.
It does not provide one machine budget across independent terminals,
worktrees, editors, CI helpers, or AI-agent sessions. Four isolated Cargo
sessions at eight frontend jobs were slower and used more CPU and memory than
the same batch at one frontend job.

The adopted path is read-only owner, serial-region, resource, jobserver-domain,
and session-pressure explanation. Nightly diagnostics, cooperating
session-budget experiments, and rustc-perf fixtures stay behind compatibility
and approval boundaries. Production flags, automatic job tuning, source
splitting, and upstream activity remain closed.

## Evidence

- [Research synthesis](../2026-08-08-frontend-parallelism.md)
- [Experiment](../perf-q16-frontend-parallelism/results/EXP-01-owner-granularity-resource-contention.md)

## Primary roles

Compiler Performance Engineer, Native Platform Adopter, Validation Checker.
