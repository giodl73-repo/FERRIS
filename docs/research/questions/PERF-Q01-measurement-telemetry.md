# PERF-Q01: Rust Latency Measurement and Causal Telemetry

**Status:** Planned

**Area:** Foundation

**Depends on:** None

## Question

Which stable and optional-nightly evidence surfaces can separate Cargo,
frontend, incremental, macro, codegen, emission, link, and validation latency?

## Starting hypothesis

Stable Cargo metadata, JSON messages, timings, and wall-clock records can explain
build-unit work, but query-level causality requires selected self-profile data.

## Investigation focus

- Validate telemetry against synthetic controls with known causes.
- Measure instrumentation overhead and missing categories.
- Define the smallest evidence set needed by every later question.

**Model changes if:** stable evidence cannot distinguish direct, downstream,
codegen, and link work reliably.

## Decision informed

Freeze the shared evidence contract and determine when nightly is justified.

## Primary roles

Compiler Performance Engineer, AI Assurance Skeptic, Validation Checker.
