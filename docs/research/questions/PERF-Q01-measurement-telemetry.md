# PERF-Q01: Rust Latency Measurement and Causal Telemetry

**Status:** Complete

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

## Result

Completed in
`docs/research/2026-08-07-rust-latency-telemetry.md`.

The decision is to use a layered evidence stack:

- minimally instrumented repeated wall-clock measurements;
- Cargo metadata for the declared graph;
- Cargo JSON messages for observed artifacts, freshness, and cached or current
  build-script output;
- separately labelled Cargo timing diagnostics for unit duration and
  scheduling;
- optional nightly rustc self-profile for query-level evidence;
- rustc-perf for compiler-change and upstream claims.

The primary benchmark remains separate from instrumentation calibration. A
`build-script-executed` JSON message is not proof that the script ran because
Cargo may replay cached output. Nightly self-profile overhead remains an
explicit follow-up rather than a prerequisite for stable Cargo analysis.
