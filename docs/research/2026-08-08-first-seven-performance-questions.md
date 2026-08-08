# What the First Seven Performance Questions Established

Date: 2026-08-08
Scope: PERF-Q01 through PERF-Q07
Status: Research checkpoint

## Executive conclusion

The first seven questions changed FERRIUM's framing of Rust performance.

Rust build latency is not one slow-compiler problem. It is a systems problem
spanning measurement, Cargo identity, graph scheduling, command semantics,
artifact provenance, CI transport, rust-analyzer, and foreground developer
work.

The strongest current FERRIUM opportunity is not a compiler, cache backend, or
Cargo replacement. It is a read-only build-intelligence layer that can explain:

- what work ran and why;
- which identity or coverage boundary prevented reuse;
- which repeated work was required, compatible and reused, suspicious, or
  unknown;
- which graph edge, queue, lock, cache, or subprocess delayed feedback;
- whether a cache or isolated target reduced latency or merely moved and
  duplicated cost;
- which correctness, validation, trust, or resource trade accompanied an
  apparent speedup.

The implementation gate remains closed while the research moves from Cargo and
orchestration into rustc startup and frontend phases.

## Seven decisions

### PERF-Q01: latency requires layered evidence

No single Rust telemetry surface explains end-to-end latency. Primary claims
need minimally instrumented repeated wall-clock runs. Cargo metadata, JSON
messages, timings, rustc self-profile, and rustc-perf remain distinct
diagnostic layers because they answer different questions and can have
different observer effects.

Decision:
[Rust latency telemetry](2026-08-07-rust-latency-telemetry.md).

### PERF-Q02: Cargo identity is layered

Cargo graph units, artifact namespaces, symbol metadata, and freshness
fingerprints are related but different identities. Source changes commonly
rebuild an existing artifact identity, while feature, profile, mode, target,
toolchain, flag, or dependency changes may require a separate identity.

Decision:
[Cargo build-unit identity](2026-08-07-cargo-build-unit-identity.md).

### PERF-Q03: scheduling delay is evidence, not guaranteed savings

Cargo's observed ready-unit priority used fixed unit costs and transitive
dependent fan-out rather than measured compilation duration. A slow root-gating
unit could wait behind shorter high-fan-out chains, but manually prebuilding
that unit was slower because it removed overlap.

Decision:
[Cargo graph scheduling](2026-08-08-cargo-graph-scheduling.md).

### PERF-Q04: most visible multiplication needs semantic classification

Tests, benches, features, profiles, target roles, doctests, compiler wrappers,
and validation coverage can require distinct Cargo units. Package counts,
package-version duplicates, command names, and even matching unit graphs do not
prove artifact compatibility. Reducing unit count can change behavior or
coverage.

Decision:
[Cargo build-unit multiplication](2026-08-08-cargo-build-unit-multiplication.md).

### PERF-Q05: shared artifact reuse is a correctness system

Exact immutable registry dependencies can be reused across workspaces, but a
shared writable target directory is not a safe general cache. Controlled path
packages with colliding local identities produced successful wrong-artifact
reuse. Cargo freshness also did not provide artifact integrity verification.

Decision:
[Cross-workspace artifact reuse](2026-08-08-cross-workspace-artifact-reuse.md).

### PERF-Q06: a CI cache hit is not a Cargo reuse result

Cargo compatibility, CI transport keys, immutable first-writer behavior,
branch trust, retention, payload composition, restore cost, and workflow
placement are independent. Exact hits can still rebuild roots, restore unused
payloads, or occur after the dominant failing setup step.

Decision:
[CI cache topology](2026-08-08-ci-cache-topology.md).

### PERF-Q07: editor responsiveness and total work are different objectives

rust-analyzer semantic analysis, Cargo build data, flycheck, and foreground
commands are distinct work layers. Shared targets can coalesce identical
checks but delay an incompatible build. Isolated targets restore overlap by
duplicating compiler work, target storage, and machine demand. Build scripts
and procedural macros remain correctness inputs.

Decision:
[Editor and Cargo contention](2026-08-08-editor-cargo-contention.md).

## Cumulative model

The seven questions establish this causal chain:

```text
developer intent
  -> editor and validation coverage
  -> Cargo command, graph, unit identity, and schedule
  -> artifact freshness, locks, and target topology
  -> rustc invocation and compiler phases
  -> local or transported cache state
  -> diagnostics, artifacts, tests, or executable result
```

Every layer can add latency, duplicate work, preserve necessary semantics, or
hide a correctness boundary. An optimization claim must name the changed
layer and preserve the others explicitly.

## What is now defensible

### Adopt now

- A shared evidence vocabulary for wall time, units, identity, freshness,
  queueing, locks, caches, coverage, and diagnostic readiness.
- Read-only identity, variant, schedule, cache, and editor-loop explanations.
- Positive and negative fixtures that distinguish useful reuse from unsafe or
  merely apparent reuse.
- Foreground latency and total machine work as separate objectives.

### Prototype behind a compatibility boundary

- A joined Cargo and rust-analyzer activity timeline.
- Identity and artifact-freshness diffs between commands or sessions.
- Critical-path and queue-delay explanations with clearly labelled
  counterfactuals.
- CI cache topology and net-benefit reports.
- Shared-versus-isolated target what-if reports.

No prototype is approved yet.

### Reject or defer

- A new compiler, Cargo fork, language server, or cache backend before the
  bounded diagnostic need is validated.
- Automatic feature unification, profile merging, target removal, command
  splitting, validation reduction, or editor coverage reduction.
- Shared writable target directories across unrelated repositories.
- Treating lock messages, cache hits, package duplication, or lower unit counts
  as performance results by themselves.
- Upstream activity without explicit owner approval.

## What remains unknown

PERF-Q01 through PERF-Q07 primarily established the orchestration and reuse
model around rustc. They did not yet isolate:

- fixed cost per rustc process;
- compiler option and session initialization;
- sysroot and dependency metadata discovery, mapping, and decoding;
- parsing, expansion, resolution, HIR, type checking, trait solving, borrow
  checking, and MIR costs;
- incremental query hashing, loading, and false invalidation;
- monomorphization, codegen, debug emission, backend, and linker costs.

PERF-Q08 begins that compiler-internal sequence by measuring rustc startup and
metadata loading separately from useful frontend work.
