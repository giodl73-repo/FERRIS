# OSPREY-SPEC-001 Nine-Role Review

Date: 2026-08-09
Specification: OSPREY-SPEC-001 Query Forest Component Model
Disposition: Accepted as Draft after required revisions
Implementation authority: None

## Review question

Does OSPREY-SPEC-001 define sufficiently bounded, product-neutral components
for later Query Forest specifications without turning the Forest into a
monolith, weakening correctness boundaries, or opening implementation?

## Executive disposition

All nine roles accept OSPREY-SPEC-001 as the first **Draft** component
specification after the revisions recorded below.

The review does not approve implementation. The main strengths are:

- the Forest is a canonical evidence model rather than one service;
- adapters, maps, ledgers, records, engines, and views have distinct duties;
- observations, predictions, resolutions, executions, and outcomes remain
  separate;
- unknown and unsupported states are first-class;
- capabilities, validation, provenance, ownership, rollback, and evidence are
  explicit; and
- a deliberately small first proof is separated from the full component
  catalog.

The main objections required:

- a first-class interop boundary map;
- explicit AI/model action provenance;
- explicit operational adoption, training, support, removal, and audit cost;
- stronger measurement and conformance obligations;
- ordinary Cargo/editor preservation and removability as normative rules; and
- an explicit statement that OSPREY is a code name, not a committed product.

## Role dispositions

### Rust Safety Steward

**Disposition:** Accept with normative constraints.

**Accepted:**

- safety and semantic analysis appear in the Capability plane;
- compiler acceptance is not treated as behavioral proof;
- unknown and unsupported states cannot become success;
- actions require approval, validation, rollback, cleanup, and audit; and
- the spec does not introduce `unsafe` code or artifact restoration.

**Required revision:**

- add explicit safety-claim and unsafe-boundary evidence;
- require dedicated evidence for ownership, lifetime, aliasing, concurrency,
  and soundness claims; and
- forbid OSPREY from deriving a safety claim from build or test success alone.

**Result:** Incorporated into canonical nodes, the Assurance Ledger, and
foundational requirements.

### Compiler Performance Engineer

**Disposition:** Accept as a component model, not a performance claim.

**Accepted:**

- Critical-Path, Cost, Reuse, Artifact, Environment, and Validation structures
  distinguish user latency from total work;
- compiler phases, queries, monomorphization, CGUs, backends, emission, and
  linking remain separate; and
- planned and observed work cannot be conflated.

**Required revision:**

- measurable claims must inherit the build-latency measurement contract;
- workflow, cache state, commands, hardware, environment, repetitions,
  variance, failures, and limitations must remain attached to results; and
- convenient microbenchmarks cannot stand in for representative workflows.

**Result:** Incorporated into foundational requirements and the Cost Ledger.

### Interop Boundary Auditor

**Disposition:** Accept after adding a dedicated boundary projection.

**Accepted:**

- native dependencies, ABI, panic, unwind, allocator, target, linker, and debug
  capabilities are represented; and
- adapter ownership remains upstream-specific.

**Required revision:**

- language and process boundaries need a first-class Interop Boundary Map;
- ownership, lifetime, exception, panic, threading, allocation, calling
  convention, layout, generated bindings, and negative tests must be explicit;
  and
- C-shaped interfaces cannot imply preservation of richer source semantics.

**Result:** Incorporated through Interop Boundary and ABI Contract nodes,
`CROSSES_BOUNDARY`, and the Interop Boundary Map.

### AI Assurance Skeptic

**Disposition:** Accept after model actions become first-class evidence.

**Accepted:**

- observed, inferred, predicted, unknown, resolved, and executed states remain
  separate;
- failures and limitations are preserved;
- high-risk execution requires human approval; and
- FERRIS is an evidence packet, not a correctness certificate.

**Required revision:**

- record model identity, prompt or instruction reference, proposed action,
  human approval, commands, results, rejection, and rollback;
- require dedicated evidence for security, performance, safety, and soundness;
  and
- prohibit success-shaped fallbacks.

**Result:** Incorporated through Model Identity and Agent Action nodes,
`PROPOSED_BY`, the Assurance Ledger, Model Action Record, and requirements.

### Ecosystem Strategist

**Disposition:** Accept as a defensible coordination layer.

**Accepted:**

- Cargo, rustc, rust-analyzer, Cranelift, LLVM, linkers, crate maintainers, and
  upstream projects retain ownership;
- the Crates Series is a predecessor gate;
- the model supports upstream contribution packets; and
- replacement compilers, package managers, caches, and distributions remain
  non-goals.

**Required revision:**

- identify OSPREY as a code name rather than a committed product;
- preserve product-neutral canonical contracts and owner-specific adapters;
  and
- implement only the components needed by one accepted consumer workflow.

**Result:** Incorporated into specification metadata and foundational
requirements.

### Rust Maintainer

**Disposition:** Accept with a simplicity and removability constraint.

**Accepted:**

- the minimal proof uses stable Cargo, environment, and validation evidence;
- views answer bounded questions rather than exposing the complete model;
- actions and mutations are not part of the first proof; and
- raw evidence and unknowns remain visible.

**Required revision:**

- ordinary Cargo and editor workflows must remain functional without OSPREY;
- diagnostics must not require maintainers to learn internal graph vocabulary;
- every explanation must link to its evidence; and
- repository integration must be removable without correctness changes.

**Result:** Incorporated into foundational requirements and view rules.

### Native Platform Adopter

**Disposition:** Accept for planning; reject operational adoption.

**Accepted:**

- environment, capability, validation, provenance, action, rollback, and audit
  concepts are present;
- cross-platform evidence is required before implementation; and
- no host tuning, deployment, or cache restoration is authorized.

**Required revision:**

- add an Adoption and Operations Ledger covering platform support, deployment,
  compliance, training, support ownership, removal, rollback, recovery, and
  audit burden; and
- record unsupported platforms and operational costs as first-class outcomes.

**Result:** Incorporated into the ledger and record catalog.

### Scope Keeper

**Disposition:** Accept with explicit modularity constraints.

**Accepted:**

- one canonical model is separated from its projections and engines;
- the minimal first proof is substantially smaller than the full catalog;
- non-goals remain visible; and
- Crates Series and implementation gates prevent premature product work.

**Required revision:**

- no implementation may be required to implement every component;
- consumer-specific workflow semantics remain in adapters or policies, not the
  canonical core; and
- later specs may split components further but may not collapse their duties.

**Result:** Incorporated into foundational requirements.

### Validation Checker

**Disposition:** Accept as Draft; Proposed status withheld.

**Accepted:**

- component responsibilities and unknown states are explicit;
- the first-proof subset is testable;
- role objections are now recorded; and
- measurable evidence is linked to the existing measurement contract.

**Required revision:**

- SPEC-014 must define conformance fixtures and commands;
- every adapter requires positive, negative, failure, unsupported, and
  version-skew cases;
- serialization, projection consistency, prediction separation, approval,
  rollback, and FERRIS packet completeness require executable tests; and
- no measurable implementation claim may be made before those tests exist.

**Result:** Incorporated as foundational requirements and retained as a gate
for Proposed status.

## Required revisions completed

OSPREY-SPEC-001 now includes:

- code-name and product-neutral status;
- normative removability and ordinary-workflow requirements;
- measurement-contract inheritance;
- Interop Boundary and ABI Contract nodes;
- Model Identity, Agent Action, and Safety Claim nodes;
- `CROSSES_BOUNDARY`, `PROPOSED_BY`, `VERIFIED_BY`, and
  `ROLLED_BACK_BY` edges;
- an Interop Boundary Map;
- an Assurance Ledger;
- an Adoption and Operations Ledger;
- Model Action and Adoption records; and
- conformance obligations for later SPEC-014.

## Remaining gates

OSPREY-SPEC-001 remains Draft until:

1. the Crates Series supplies the ecosystem concepts consumed by later specs;
2. SPEC-002 defines the canonical schema without collapsing component duties;
3. SPEC-014 defines executable conformance and held-out workflow tests;
4. the minimal first-proof consumer and repository set are frozen; and
5. all dependent role objections are re-reviewed before Proposed status.

## Decision

Advance OSPREY-SPEC-001 from Planning to Draft.

Do not authorize implementation.
