# Ferris Read-Only Implementation Entry Review

Date: 2026-08-11
Scope: `plan` and `explain` over local Cargo metadata
Disposition: Pulse 01 approved
Implementation authority: Bounded

## Decision

Approve one implementation pulse for a local read-only semantic slice.

This approval does not advance a specification to Proposed and does not
authorize action, mutation, owner execution, network access, connectors, MCP,
AI prediction, approval, remote evidence, or held-out oracle access.

## Role dispositions

### Rust Safety Steward

Approve. The pulse requires safe Rust, forbids `unsafe`, invokes no compiler or
owner code, and makes no soundness claim from metadata or compilation.

### Compiler Performance Engineer

Approve. The pulse makes no performance claim. Cargo metadata latency may be
recorded later, but it cannot justify affected-only execution or caching.

### Interop Boundary Auditor

Approve. Cargo JSON is an explicit owner boundary. The implementation must
preserve unsupported and missing fields rather than inventing richer Rust,
ABI, native, or runtime meaning.

### AI Assurance Skeptic

Approve. No model participates in runtime behavior. Generated code is accepted
only through repository tests and the bounded contract; held-out oracles remain
under independent custody.

### Ecosystem Strategist

Approve. The pulse consumes the stable Cargo metadata interface and does not
replace Cargo resolution, workspace discovery, freshness, or execution.

### Rust Maintainer

Approve. Commands lead with Cargo workspace and package vocabulary, preserve
ordinary Cargo workflows, require actionable diagnostics, and are removable
without manifest changes.

### Native Platform Adopter

Approve for Windows development and a required Unix renewal. No ABI, SDK,
deployment, service, credential, or native-tool support is claimed.

### Scope Keeper

Approve. Two crates and two read-only commands are the maximum scope. Every
later command and integration remains explicitly deferred.

### Validation Checker

Approve. The pulse fixes its toolchain, commands, fixtures, process codes,
negative controls, validation commands, stop conditions, and removal rule.

## Remaining gates

- implementation and development-fixture validation;
- independent held-out package receipt;
- Unix renewal;
- measured nine-role completion review; and
- a new pulse before any capability beyond local `plan` and `explain`.

## Authority

Implementation authority is granted only to
`context/waves/2026-08-11-read-only-planning/pulses/pulse-01.md`.
