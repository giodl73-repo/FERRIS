# Rust Performance Contribution Packet

Status: Adopted by PERF-Q36
Owner: FERRIS
Purpose: convert one measured Rust performance case into a reviewable,
maintainable upstream contribution draft.

## Boundary

A packet is a local evidence artifact. It does not authorize FERRIS to create
an external issue, comment, branch, benchmark, pull request, or funding
commitment. External action requires repository-owner approval and the
applicable upstream intake process.

The packet supports rustc, rustc-perf, Cargo, rust-analyzer, Cranelift, LLVM,
linker, and related upstream work. It does not replace their contribution
templates, benchmark conventions, reviewers, CI, triage, or ownership.

## Required fields

### 1. Identity

- packet ID and version;
- originating PERF question and finding IDs;
- upstream repository, issue, goal, or owner;
- packet maintainer;
- status and last verification date;
- source and fixture licenses; and
- public, synthetic, or private provenance.

### 2. Maintainer question

State one question the upstream owner can answer, for example:

- Is this a useful Secondary rustc-perf benchmark?
- Does this reproduce the accepted Cargo issue?
- Which compiler query or phase should own this invalidation?
- Is this behavior expected, a diagnostic gap, or a regression?

Do not submit a general request to review the complete FERRIS research
program.

### 3. Reproducer

- smallest current fixture that preserves the distinguishing behavior;
- exact source revision or generated-fixture recipe;
- dependency graph and relevant target types;
- deterministic edit or patch sequence;
- required input files and environment dependencies;
- lockfile and offline requirements; and
- minimization history from the original observation.

After every reduction, rerun the distinguishing positive and negative
controls. A smaller fixture that no longer represents the original mechanism
is rejected.

### 4. Environment

- rustc, Cargo, rustup, backend, linker, and LLVM versions as relevant;
- host and target triples;
- operating system and execution substrate;
- source, target, Cargo home, and temporary-storage placement;
- profile, features, targets, incremental state, and configuration;
- CPU, memory, job, and concurrent-session context;
- security, indexing, power, thermal, or VM uncertainty where relevant; and
- upstream benchmark machine differences.

### 5. Benchmark vocabulary

For rustc-perf packets, use the upstream vocabulary:

- profiles: Check, Debug, Opt, Doc, DocJson, or Clippy;
- scenarios: Full, IncrFull, IncrUnchanged, or IncrPatched;
- metrics: instructions, cycles, wall time, or peak RSS; and
- stable statistical or relevance interpretation from rustc-perf.

For Cargo packets, use the accepted issue's requested test or benchmark form
and Cargo's Criterion conventions where applicable.

Local FERRIS terms may be included only with an explicit mapping to upstream
terms.

### 6. Commands

Record copyable commands for:

- fixture creation or checkout;
- baseline build or benchmark;
- edit or patch application;
- comparison build or benchmark;
- correctness and negative controls;
- optional profiling;
- cleanup and rerun; and
- upstream-prescribed local benchmark validation.

Commands name working directory, toolchain, target directory, environment
variables, and expected exit status.

### 7. Evidence

Separate:

- observed results;
- inferred mechanism;
- predicted upstream outcome;
- unknowns;
- failed or unsupported runs; and
- user impact versus stable benchmark metrics.

Include distributions or the upstream-required result format. Do not promote a
single wall-time observation.

### 8. Correctness and negative controls

List the cases that must continue to fail, rebuild, invalidate, relink, or run
broader validation. Examples include:

- inline, generic, const, macro, layout, ABI, or symbol changes;
- hidden build-script or procedural-macro inputs;
- release, target, feature, doctest, lint, native, or debugger capability;
- corrupted or mismatched cache entries;
- shared runtime inputs and repository gates; and
- unsupported platforms or toolchains.

A performance packet without its correctness frontier is incomplete.

### 9. Requested upstream action

Choose one:

- confirm expected behavior;
- accept a benchmark or test;
- classify or prioritize an issue;
- review profiling evidence;
- advise on benchmark shape;
- review a focused patch;
- identify the correct owner; or
- decline or keep the case external.

The request must be smaller than the evidence archive.

### 10. Maintenance and lifecycle

Name:

- FERRIS packet owner;
- expected upstream owner;
- response and review commitment;
- dependency or fixture update obligation;
- noise or regression investigation expectation;
- retirement, supersession, or external-only condition; and
- links to accepted upstream artifacts.

## Status vocabulary

| Status | Meaning |
|---|---|
| Draft | Local evidence is being assembled |
| Reproduced | The case reruns on its declared environment |
| Minimized | Reduction preserves required controls |
| Owner-aligned | Upstream owner confirmed usefulness and destination |
| Submission-ready | Upstream format, license, commands, and local validation are complete |
| Submitted | Approved external issue, comment, or PR exists |
| Accepted | Upstream artifact or decision was accepted |
| External | Owner prefers the case remain outside the upstream project |
| Superseded | A newer issue, benchmark, mechanism, or packet replaces it |
| Retired | The behavior no longer reproduces or the artifact is no longer maintained |

## Promotion gate

A packet may become submission-ready only when:

1. the upstream home and maintainer question are explicit;
2. the reproducer is licensed and public-safe;
3. the distinguishing behavior survives minimization;
4. commands rerun from a clean declared environment;
5. stable metrics or the upstream-required benchmark output exist;
6. correctness and negative controls pass;
7. limitations and unsupported cases remain visible;
8. the requested upstream action is bounded;
9. maintenance ownership is accepted; and
10. external submission is approved.

## Rustc-perf adaptation checklist

For a new compile benchmark, follow the current upstream documentation:

- discuss benchmark usefulness when uncertain;
- split benchmark code and registration/configuration into two commits;
- add `[workspace]` to the benchmark manifest;
- configure `perf-config.json`;
- add incremental patch files where applicable;
- register the benchmark;
- update the compile-benchmark README;
- add `REUSE.toml` licensing;
- commit `Cargo.lock`;
- run the documented check, debug, and optimized timing comparison;
- run `collector bench_local` against the benchmark and `helloworld`; and
- request an authorized official perf run through the upstream owner.

Authoritative references:

- <https://github.com/rust-lang/rustc-perf/blob/main/collector/compile-benchmarks/README.md>
- <https://github.com/rust-lang/rustc-perf/blob/main/collector/README.md>
- <https://rustc-dev-guide.rust-lang.org/tests/perf.html>
- <https://rustc-dev-guide.rust-lang.org/profiling/with-rustc-perf.html>
- <https://perf.rust-lang.org/help.html>

## Cargo adaptation checklist

- start from an explicitly accepted issue;
- confirm the desired test, benchmark, or documentation form;
- use Cargo's existing test and Criterion patterns;
- preserve platform and filesystem controls;
- distinguish Cargo runtime from rustc compilation;
- include local benchmark and correctness evidence; and
- keep workflow, resolver, fingerprint, and cache semantics under Cargo owner
  review.

Authoritative references:

- <https://github.com/rust-lang/cargo/blob/master/CONTRIBUTING.md>
- <https://doc.crates.io/contrib/>
- <https://github.com/rust-lang/cargo/blob/master/benches/README.md>

## Non-goals

- automatic external posting;
- replacing upstream templates or review;
- treating benchmark admission as product validation;
- bundling unrelated issues into one packet;
- publishing private source or environment details;
- claiming cross-platform behavior from one host;
- defining rustc, Cargo, backend, or linker architecture downstream; and
- measuring contribution success only by merged code.
