# Ferris Specification Convergence Nine-Role Review

Date: 2026-08-10
Scope: PRODUCT-001 through CONFORMANCE-001
Specification count: 22
Disposition: Complete Draft spine accepted; Proposed status withheld
Implementation authority: None

## Review question

Does the complete Ferris specification spine define one coherent,
owner-aligned cross-workspace enterprise build system for Rust, with executable
proof obligations for every claim and no hidden path from research or planning
into implementation authority?

## Executive disposition

All nine roles accept the 22-specification spine as a coherent Draft.

The review confirms:

- one public product, Ferris, with Blueprint as its internal model;
- one semantic engine across `ferris`, `cargo ferris`, and governed MCP;
- Cargo and external systems retaining owner-local authority;
- separate product, governance, contract, profile, application, scope,
  evidence, identity, causality, prediction, validation, planning, resolution,
  trust, execution, connector, packet, view, and conformance contracts;
- acyclic dependencies;
- conservative unknown handling and full-reference fallback;
- immutable evidence, predictions, plans, resolutions, roots, and histories;
- approval, trust, action, external publication, and packet readiness remaining
  distinct;
- Windows and Unix proof obligations; and
- ordinary Cargo, owner workflows, rollback, and removal preserved.

No role authorizes Proposed status or implementation.

## Role dispositions

### Rust Safety Steward

Accept the Draft spine. Safety, soundness, ownership, lifetime, aliasing,
concurrency, panic, unwind, unsafe, macro, build-script, native, and toolchain
claims have dedicated evidence and negative-test obligations. Compilation,
signatures, trust, or approval cannot substitute for those claims.

### Compiler Performance Engineer

Accept the Draft spine. Representative workflows, cache states, commands,
environment, variance, selected-versus-full work, resource envelopes,
prediction error, fallback, retries, rollback, cleanup, and maintenance cost
are required before performance claims.

### Interop Boundary Auditor

Accept the Draft spine. Rust source, Typebook, ABI, WIT, wire, native,
generated-binding, linker, runtime, data, deployment, migration, and rollback
identities and validation remain distinct.

### AI Assurance Skeptic

Accept the Draft spine. AI cannot establish owner truth, upgrade causal claim
classes, remove mandatory work, approve policy, access durable secrets,
rewrite immutable records, execute actions, or publish externally. Model
identity, prompts, evidence, errors, rejection, and rollback remain
attributable.

### Ecosystem Strategist

Accept the Draft spine. Ferris fills the application-level coordination gap
without replacing Cargo, rustc, Typebook/RUNE, linkers, test systems, native
tools, deployment providers, or upstream contribution processes. Connectors
and engines remain replaceable.

### Rust Maintainer

Accept the Draft spine. Public commands, diagnostics, plans, explanations,
validation, approvals, failures, and removal lead with Cargo and owner
vocabulary. Ordinary Cargo and editor workflows remain the escape hatch and
correctness baseline.

### Native Platform Adopter

Accept the Draft spine for specification only. Operational adoption remains
blocked on exact Windows and Unix tools, SDKs, ABIs, packaging, deployment,
credentials, support, recovery, rollback, deletion, and maintenance fixtures.

### Scope Keeper

Accept the Draft spine. The seven programs and 22 specifications remain
bounded, Typebook remains product-neutral, Microsoft remains a connector
profile, and implementation remains behind a separately approved pulse.

### Validation Checker

Accept the Draft spine. Proposed status remains blocked on frozen schemas,
repositories, revisions, commands, expected output, numeric exit codes,
positive and negative fixtures, seeded failures, cross-platform execution,
measurable thresholds, and complete removal.

## Dependency convergence

The final dependency direction is:

```text
product and governance
  -> contracts, profiles, and application
    -> scope, canonical evidence, identity, adapters, and projections
      -> causality, prediction, validation, and planning
        -> resolution, trust, approved execution, connectors, and packets
          -> public views
            -> conformance
              -> separately approved implementation pulse
```

No normative specification depends on a later gate. No connector or Microsoft
service is required for canonical Ferris identity or owner-local correctness.

## Remaining Proposed-status blockers

Before any specification advances to Proposed:

1. freeze all canonical schemas and machine projections;
2. freeze three public repositories, exact revisions, and synthetic controls;
3. freeze Windows and Unix environments and held-out edits;
4. assign numeric exit codes and exact CLI/MCP schemas;
5. execute positive, negative, failure, unsupported, stale, version-skew,
   permission, tenant, revocation, rollback, deletion, and removal fixtures;
6. measure selected-versus-full correctness, prediction error, calibration,
   latency, resources, variance, investigation time, and maintenance cost;
7. execute native, ABI, contract, connector, MCP, packet, trust, and external
   publication controls;
8. demonstrate complete removal and ordinary owner workflows;
9. define adoption, support, servicing, incident, upgrade, recovery, and
   retirement ownership; and
10. repeat all nine role reviews over measured artifacts.

## Decision

Accept the Ferris 22-specification spine as complete Draft architecture.

Do not advance any specification to Proposed.

Do not authorize implementation.
