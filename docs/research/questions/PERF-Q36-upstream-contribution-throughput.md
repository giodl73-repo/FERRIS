# PERF-Q36: Rust Performance Contribution Throughput

**Status:** Complete

**Area:** Upstream program

**Depends on:** Measured results from relevant PERF questions

## Question

How can FERRIUM convert portfolio evidence into fixtures, rustc-perf cases,
issue reports, reviews, funding, and upstream patches that maintainers can use?

## Starting hypothesis

Minimized cases, reproducible commands, correctness tests, and sustained review
support are more valuable than a downstream compiler fork.

## Investigation focus

- Study accepted performance contributions and maintainer review requirements.
- Measure the effort from real-world observation to minimized upstream case.
- Define ownership, maintenance, communication, and funding options.

**Model changes if:** upstream processes cannot absorb representative cases or
the problems are repository-specific.

## Decision informed

Create the Phase 4 contribution program and select its first upstream target.

## Primary roles

Ecosystem Strategist, Rust Maintainer, Compiler Performance Engineer.

## Decision

Close the initial 36-question performance research ladder and open a
contribution-first Phase 4. Adopt one issue-specific upstream contribution
packet at a time. Select the PERF-Q20 Relink-Don't-Rebuild body-versus-interface
matrix as the first rustc-perf-compatible target, subject to Linux
reproduction, stable local metrics, owner alignment, licensing, maintenance,
and approval before external submission.

Research completion does not open the FERRIUM product implementation gate.

See the
[Rust performance contribution program closeout](../2026-08-09-rust-performance-contribution-program-closeout.md).
