# Ferris Seven-Program Architecture

Status: Draft after nine-role review
Implementation authority: None
Public product: Ferris
Research basis: PERF-Q01 through PERF-Q36, ECOS-Q01 through ECOS-Q12, and
BLUE-Q01 through BLUE-Q07

## Purpose

This program architecture is the final organizational spine for converting
the completed Ferris research corpus into specifications and, only after later
approval, bounded implementations.

It answers:

- which capabilities belong to Ferris;
- which capabilities remain separately useful and replaceable;
- how contracts, profiles, plans, evidence, validation, and upstream systems
  connect;
- which specification owns each boundary; and
- where `.roles` review must occur before work advances.

The architecture contains seven programs. They are not seven executables,
seven repositories, or seven competing products.

## System map

```text
                    Ferris Program
       command, selection, policy, approval, and lifecycle
                          |
                          v
Typebook contracts ───> Application Definition <─── Renewable profiles
                          ^
                          |
                 Ecosystem owner adapters
                          |
                          v
                   Blueprint Program
              model -> plan -> approved action
                          |
                          v
Owner-local Cargo, rustc, test, native, packaging, and deployment work

Query Forest supplies evidence, causality, roots, refs, and history to Ferris,
Blueprint, Profiles, Typebook projections, and Ecosystem Bridge adapters.

Conformance Program gates every arrow and every claim.
Ecosystem Bridge routes every external boundary to its current owner.
```

The governing rules are:

> **The plan is global; the work is local.**

> **Ferris coordinates owner truth; it does not replace it.**

## Program 1: Ferris

### Mission

Ferris is the public cross-workspace enterprise build system for Rust.

### Owns

- the `ferris` and `cargo ferris` command surfaces;
- the portable Enterprise Governance Plane for identity, authorization,
  policy, approval, tenancy, data, secrets, audit, budgets, and revocation;
- application, repository, workspace, policy, and lifecycle selection;
- one shared semantic command engine;
- plan presentation and approval;
- enterprise resource coordination;
- outcome, rollback, and removal workflows; and
- the public compatibility and support contract.

### Consumes

- Application Definitions;
- Typebook contracts;
- renewable profiles;
- Blueprint Plans;
- Query Forest roots and evidence;
- conformance results; and
- external owner capabilities exposed through the Ecosystem Bridge.

### Produces

- approved user-visible plans and actions;
- explanations and diagnostics;
- FERRIS Application Contracts;
- audit and lifecycle records; and
- removable repository and CI integrations.

### Primary specifications

PRODUCT-001, APPLICATION-001, RESOLUTION-001, EXECUTION-001, TRUST-001,
GOVERNANCE-001, FERRIS-001, and VIEW-001.

### Non-goals

- replacing Cargo, rustc, linkers, test runners, or deployment systems;
- one global dependency resolution across independent workspaces;
- hidden repository mutation;
- correctness claims from product branding; and
- requiring adoption of every other program implementation.

## Program 2: Typebook

### Mission

Typebook is the product-neutral semantic contract program currently represented
by the separate RUNE standards repository.

### Owns

- durable concept, operation, error, state, and lifecycle descriptors;
- deterministic registries;
- compatibility reports;
- versioned semantic projections and adapters; and
- product-neutral contract serialization.

### Consumes

- Rust APIs and Cargo SemVer;
- C ABI, WIT/component, and wire-schema definitions;
- explicit ownership, error, async, cancellation, panic, threading, and
  lifecycle semantics; and
- positive and negative conformance evidence.

### Produces

- reusable contract identities;
- compatibility and projection reports;
- adapter requirements; and
- semantic inputs that Ferris applications and profiles may reference.

### Primary specifications

CONTRACT-001 plus product-neutral specifications owned by RUNE/Typebook.

### Boundary

Typebook remains separately describable, usable by non-Ferris consumers, and
replaceable. Ferris MAY contribute a missing neutral contract upstream. Ferris
MUST NOT copy the standard into product-specific schemas or claim Typebook
compatibility from shape matching alone.

## Program 3: Profiles

### Mission

Profiles define renewable, consumer-scoped support and compatibility
commitments over exact stacks and environments.

### Owns

- consumer requirements and profile identity;
- exact releases, features, lock and active-target closures;
- Cargo/rustc, host/target, provider, native-tool, and deployment assumptions;
- independently recorded resolve, check, build, link, execute, test, package,
  deploy, and operational stages;
- provenance, advisories, licensing, stewardship, and residual unknowns; and
- expiry, renewal, substitution, removal, and rollback.

### Produces

- portable profile records;
- profile diffs and renewal proposals;
- explicit supported, unsupported, failed, stale, and unknown states; and
- one input to the FERRIS Application Contract.

### Primary specifications

PLATFORM-001, with dependencies on CONTRACT-001, EVIDENCE-001,
VALIDATION-001, TRUST-001, and CONFORMANCE-001.

### Boundary

A profile is not a distribution, lockfile mandate, certification, universal
stack recommendation, or installation authority. Profile authors own their
support commitments; Ferris records and enforces only declared policy.

## Program 4: Blueprint

### Mission

Blueprint converts application intent and owner evidence into a federated,
versioned, non-executable plan.

### Owns

- the normalized Blueprint Model;
- multi-dimensional scope coordinates and typed mappings;
- owner-specific affected closures;
- one Cargo invocation plan per activity;
- contract, native, link, validation, packaging, and lifecycle work;
- resource envelopes and concurrency policy;
- uncertainty, observation barriers, fallback, and replanning; and
- projection into an approved Action Plan.

### Produces

- Blueprint Models;
- Blueprint Plans;
- candidate alternatives and resolution inputs; and
- expected evidence and root outputs.

### Primary specifications

APPLICATION-001, SCOPE-001, PREDICTION-001, VALIDATION-001, PLANNING-001,
RESOLUTION-001, and EXECUTION-001.

### Boundary

Blueprint does not execute merely because a plan exists. It does not recreate
Cargo resolution, compiler query semantics, test enumeration, native discovery,
or deployment-provider semantics. Unknown mappings widen to a named safe
boundary.

## Program 5: Query Forest

### Mission

Query Forest is the canonical evidence, causality, identity, and history
program beneath Ferris and Blueprint.

### Owns

- canonical nodes, edges, states, maps, ledgers, and records;
- source, Cargo, compiler, artifact, validation, environment, contract,
  native, action, and outcome identity;
- observed, inferred, predicted, resolved, approved, executed, and yielded
  state separation;
- causal attribution, confidence, unknowns, and source ownership;
- immutable roots, typed branches, tags, channels, aliases, pins, leases,
  tombstones, and metadata labels;
- ref history, compare-and-set updates, retention, revocation, and lineage; and
- projections for explanations, critical paths, scope, validation, operations,
  and assurance.

### Primary specifications

FOREST-001 through FOREST-003, IDENTITY-001, EVIDENCE-001, CAUSALITY-001,
PREDICTION-001, and TRUST-001.

### Boundary

The Forest is not one runtime service, one universal hash, a cache key,
compatibility proof, validation proof, or correctness certificate. Labels
never confer authority. Artifact restoration remains separately gated.

## Program 6: Conformance

### Mission

Conformance turns Ferris claims into reproducible positive, negative, failure,
unsupported, stale, version-skew, cross-platform, rollback, and removal tests.

### Owns

- build-latency and edit-to-confidence measurement;
- frozen repositories, revisions, environments, commands, and expected
  outputs;
- selected-only versus full-reference comparison;
- Typebook and profile compatibility fixtures;
- scope, plan, approval, action, evidence, and ref consistency tests;
- AI proposal, policy, approval, rejection, and rollback evidence;
- Windows and Unix behavior;
- adoption, upgrade, support, recovery, rollback, and removal tests; and
- fixed success, failure, and stop thresholds.

### Primary specifications

VALIDATION-001, VIEW-001, and CONFORMANCE-001, inheriting the Build Latency
Measurement Contract.

### Boundary

Conformance is a capability, not a final phase added after implementation.
No implementation may claim Ferris compatibility from compilation, a selected
test pass, one platform, or one successful demonstration.

## Program 7: Ecosystem Bridge

### Mission

The Ecosystem Bridge preserves external ownership while making Cargo, rustc,
crates, native systems, standards, tools, and upstream contribution paths
usable through Ferris.

### Owns

- versioned adapters and capability discovery;
- the connector manifest and conformance model;
- governed MCP client/server integration;
- Microsoft and other enterprise connector profiles;
- current-owner and upstream-home records;
- the Crate Ecosystem Ledger;
- capability, stewardship, assurance, fragmentation, native, discovery, and
  selection evidence;
- unsupported and version-skew behavior;
- issue-specific upstream contribution packets; and
- adapter adoption, maintenance, expiry, replacement, and removal.

### Primary specifications

CONTRACT-001, EVIDENCE-001, TRUST-001, GOVERNANCE-001, CONNECTOR-001,
FERRIS-001, and CONFORMANCE-001.

### Boundary

The bridge does not certify crates, rank maintainers, create a Ferris standard
library or distribution, take over stewardship, post externally without
approval, or replace mature owner systems.

## Cross-program contracts

| Producer | Consumer | Required contract |
|---|---|---|
| Typebook | Profiles, Blueprint | Stable semantic identity plus explicit projection loss |
| Profiles | Blueprint, Ferris | Exact scope, evidence date, expiry, owner, validation, removal, rollback |
| Ecosystem Bridge | Profiles, Blueprint, Query Forest | Versioned owner evidence with unsupported and stale states |
| Blueprint | Ferris | Non-executable plan, alternatives, uncertainty, fallback, approval requirements |
| Query Forest | Every program | Typed identity, source, confidence, lifecycle, and immutable history |
| Conformance | Every program | Reproducible proof and explicit limitations |
| Ferris | Maintainers and operators | One coherent command, policy, explanation, and removal contract |
| Governance Plane | Ferris, Blueprint, Connectors | Principal, authorization, approval, data, secret, tenant, audit, budget, revocation |
| Connector Framework | Ferris and external owners | Capability, protocol, maturity, auth, failure, telemetry, lifecycle, fallback |

No program may infer another program's success from the existence of an input
record. Every boundary carries identity, scope, version, owner, evidence,
expiry, unknowns, and fallback.

## Specification and review sequence

### Stage 1: Program and product boundary

- PRODUCT-001;
- CONTRACT-001;
- PLATFORM-001; and
- APPLICATION-001.

**Review:** all parliament roles during drafting; Rust Maintainer and Native
Platform Adopter before onboarding is fixed; Scope Keeper and Validation
Checker before Proposed status; all nine roles at stage closure.

### Stage 2: Evidence and planning truth

- SCOPE-001;
- FOREST-001 through FOREST-003;
- IDENTITY-001;
- EVIDENCE-001;
- CAUSALITY-001;
- PREDICTION-001;
- VALIDATION-001; and
- PLANNING-001.

**Review:** all nine roles before schema or plan semantics freeze. Compiler
Performance Engineer, AI Assurance Skeptic, and Validation Checker must approve
held-out and full-reference controls.

### Stage 3: Action, trust, and external ownership

- RESOLUTION-001;
- EXECUTION-001;
- TRUST-001; and
- GOVERNANCE-001;
- CONNECTOR-001; and
- FERRIS-001.

**Review:** Rust Safety Steward, Interop Boundary Auditor, AI Assurance Skeptic,
Rust Maintainer, and Native Platform Adopter have explicit vetoes over unsafe,
native, automated, credentialed, mutating, or externally posted actions.

### Stage 4: Public contract and proof

- VIEW-001; and
- CONFORMANCE-001.

**Review:** all nine roles re-review frozen commands, schemas, repositories,
fixtures, platforms, support commitments, thresholds, rollback, and removal.

### Stage 5: Bounded implementation

Only a separately approved pulse may implement one accepted workflow. Program
review, specification review, or conformance design does not open this stage.

## Completeness rule

The research corpus is considered captured only when:

- every PERF, ECOS, and BLUE question maps to at least one program;
- every program maps to named specifications;
- every cross-program edge has a versioned contract and owner;
- every claim maps to a conformance class;
- every mutable action has approval, audit, rollback, and removal; and
- every deferred capability remains explicitly visible.

The closure matrix is recorded in
[EXP-01](../research/ferris-seven-programs/results/EXP-01-research-closure-matrix.md).

## Non-goals

- seven independent public products or CLIs;
- a Ferris-owned semantic standard when Typebook remains independently useful;
- a curated Rust distribution or universal stack;
- one monolithic graph, planner, cache, agent, or service;
- implementation before the specification sequence;
- hiding unknown, failed, unsupported, or stale evidence;
- automatic dependency, profile, source, environment, or CI mutation; and
- replacing current upstream maintainers or authority.
