# Ferris Declared Graph Entry Review

Date: 2026-08-11
Scope: Pulse 02 local declared-workspace `graph`
Disposition: Approved
Implementation authority: Bounded

## Decision

Approve one read-only extension that projects Cargo-declared workspace
packages and dependency declarations.

This approval does not authorize dependency resolution, affected scope,
invalidation, scheduling, execution, mutation, Query Forest persistence,
connectors, MCP, AI, approval, or remote evidence.

## Role dispositions

### Rust Safety Steward

Approve. The pulse remains safe Rust and metadata-only. A declaration edge is
not evidence of compilation, safety, reachability, or runtime behavior.

### Compiler Performance Engineer

Approve with no performance or scheduling claim. The graph cannot be presented
as a build unit, critical path, invalidation, or freshness graph.

### Interop Boundary Auditor

Approve. Dependency kind, alias, optional state, target condition, and
unresolved target remain explicit. No ABI or native edge is inferred.

### AI Assurance Skeptic

Approve. No model participates. Unknown and unresolved targets cannot be
filled by generated guesses.

### Ecosystem Strategist

Approve. Cargo metadata remains authoritative; Ferris does not implement a
resolver or replace `cargo tree`.

### Rust Maintainer

Approve. Output uses package and dependency vocabulary, retains ordinary Cargo
fallback, and remains removable without manifest changes.

### Native Platform Adopter

Approve for the recorded Windows and Unix renewal environments. No native,
SDK, packaging, deployment, or service graph is claimed.

### Scope Keeper

Approve. One command, one experimental schema, fixed bounds, and the existing
two crates are the maximum scope.

### Validation Checker

Approve. Positive, unresolved, alias, condition, ordering, bound, failure,
path-privacy, and cross-platform fixtures are required.

## Authority

Implementation authority is granted only to
`context/waves/2026-08-11-read-only-planning/pulses/pulse-02.md`.
