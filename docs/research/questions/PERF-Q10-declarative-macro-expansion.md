# PERF-Q10: Declarative Macro Expansion

**Status:** Complete

**Area:** Frontend

**Depends on:** PERF-Q01, PERF-Q09

## Question

Which declarative macro patterns create disproportionate expansion, generated
syntax, diagnostics, or invalidation?

## Starting hypothesis

Large expansions and repeated matching can create frontend critical paths whose
cost is poorly visible to crate maintainers.

## Investigation focus

- Measure expansion time and generated token volume.
- Compare local macro edits with downstream invalidation.
- Identify diagnostic and visibility improvements before source recommendations.

**Model changes if:** procedural execution or later type checking explains most
macro-associated cost.

## Decision informed

Define external macro-cost diagnostics and candidate upstream profiler cases.

## Decision

Adopt matcher, transcription, cumulative-output, invocation, arm-prefix,
recursion, fragment, incremental-fanout, frontend-thread, and failure
vocabulary. Prototype a read-only declarative macro census and parametric
fixture behind nightly compatibility boundaries.

Consider rustc-perf coverage, structured macro statistics, and finer matcher,
transcriber, output-parser, hygiene, and integration events only after
explicit owner approval. Do not automatically rewrite macros, raise recursion
limits, cache expansion, parallelize expansion, check in expanded source, or
enter procedural macro scope.

## Results

- [Synthesis](../2026-08-08-declarative-macro-expansion.md)
- [EXP-01: matcher, transcription, and invalidation shapes](../perf-q10-declarative-macro-expansion/results/EXP-01-matcher-transcription-and-invalidation.md)

Findings: `FERRIUM-117` through `FERRIUM-126`.

## Primary roles

Compiler Performance Engineer, Rust Maintainer, AI Assurance Skeptic.
