# FERRIS Specification Registry

Status: Active
Implementation authority: None unless a separately approved pulse says
otherwise

## Purpose

FERRIS plans explain direction and sequencing. Specifications define precise,
reviewable contracts that a future implementation would have to satisfy.

A specification does not authorize code. Implementation requires:

1. completion of its named research dependencies;
2. a proposed or adopted specification;
3. all applicable role reviews;
4. measurable acceptance and stop criteria;
5. adoption, removal, rollback, and maintenance plans; and
6. a separately approved implementation pulse.

## Status vocabulary

| Status | Meaning |
|---|---|
| Planning | Scope and required decisions are being defined |
| Draft | Normative requirements are being written and reviewed |
| Proposed | Complete enough for role and owner approval |
| Adopted | Approved as the governing contract for later bounded work |
| Implemented | At least one approved implementation conforms to the spec |
| Superseded | Replaced by a named later specification |
| Retired | No longer active and not replaced |

`Implemented` never means every optional capability is complete.

## Normative language

In specification documents:

- **MUST** and **MUST NOT** define required boundaries.
- **SHOULD** and **SHOULD NOT** define defaults that require a documented reason
  to override.
- **MAY** defines an optional capability.

Planning documents may use ordinary prose and do not become normative merely
because they link to a specification.

## Existing FERRIS contracts

| Contract | Status | Purpose |
|---|---|---|
| [Build latency measurement contract](BUILD_LATENCY_MEASUREMENT_CONTRACT.md) | Adopted for research | Defines fixtures, workloads, evidence, statistics, privacy, validation-selection, and prototype gates |
| [Rust performance contribution packet](RUST_PERFORMANCE_CONTRIBUTION_PACKET.md) | Adopted by PERF-Q36 | Defines one reviewable upstream performance contribution artifact |
| [Query Forest component model](FOREST_COMPONENT_MODEL.md) | Draft; FOREST-001 | Defines the precise Blueprint components and forbids a monolithic Forest |

## Ferris program specification sequence

The
[Ferris program](../plans/FERRIS_PROGRAM.md) defines product sequencing. The
[seven-program architecture](../plans/FERRIS_SEVEN_PROGRAM_ARCHITECTURE.md)
defines program ownership and cross-program contracts. The
[Blueprint planning engine program](../plans/BLUEPRINT_PROGRAM.md) defines the
internal planning architecture.
Specifications are developed in this order.

| ID | Specification | Status | Primary dependency |
|---|---|---|---|
| PRODUCT-001 | [Ferris public product, category, namespace, entrypoints, authority, and removal contract](FERRIS_PRODUCT_CONTRACT.md) | Draft after nine-role review | BLUE-Q01 through BLUE-Q05 and Crates Series |
| GOVERNANCE-001 | [Enterprise principals, authorization, policy, approval, tenancy, data, secrets, audit, budgets, and revocation](FERRIS_ENTERPRISE_GOVERNANCE_CONTRACT.md) | Draft after nine-role review | PRODUCT-001 and BLUE-Q07 |
| CONTRACT-001 | [Rust API, Typebook/RUNE, ABI, component, wire, data, and projection identity and compatibility](FERRIS_CONTRACT_IDENTITY_AND_COMPATIBILITY.md) | Draft after nine-role review | ECOS-Q03, ECOS-Q04, ECOS-Q07 through ECOS-Q12, Typebook/RUNE v1 |
| PLATFORM-001 | [Renewable profile selection, exact closure, environment, staged validation, support, servicing, substitution, renewal, removal, and rollback](FERRIS_PLATFORM_PROFILE_CONTRACT.md) | Draft after nine-role review | CONTRACT-001 and ECOS-Q05 through ECOS-Q12 |
| APPLICATION-001 | [Application Definition, normalized Blueprint Model, Cargo integration, immutable roots, typed refs, and FERRIS Application Contract](FERRIS_APPLICATION_MODEL_CONTRACT.md) | Draft after nine-role review | PRODUCT-001, CONTRACT-001, PLATFORM-001, BLUE-Q01, and GOVERNANCE-001 |
| FOREST-001 | Query Forest component model | Draft after nine-role review | PERF-Q01 through PERF-Q36 |
| SCOPE-001 | [Multi-dimensional scope coordinates, cross-command typed mappings, cardinality, conditions, AI narrowing controls, widening, and scope budgets](FERRIS_SCOPE_CONTRACT.md) | Draft after nine-role review | APPLICATION-001, FOREST-001, BLUE-Q03, and Crates Series |
| FOREST-002 | [Canonical nodes, edges, states, observations, roots, extensions, migrations, and serialization](FOREST_CANONICAL_SCHEMA.md) | Draft after nine-role review | FOREST-001, SCOPE-001, APPLICATION-001, BLUE-Q01, and Crates Series |
| IDENTITY-001 | [Distinct identity domains, comparison, immutable roots, typed refs, generations, retention, and lineage](FERRIS_IDENTITY_AND_LINEAGE_CONTRACT.md) | Draft after nine-role review | CONTRACT-001, APPLICATION-001, SCOPE-001, FOREST-002, and BLUE-Q01 |
| EVIDENCE-001 | [Read-only owner evidence adapters, manifests, claims, normalization, isolation, failure, and upstream ownership](FERRIS_EVIDENCE_ADAPTER_CONTRACT.md) | Draft after nine-role review | GOVERNANCE-001, SCOPE-001, FOREST-002, and Crates Series |
| FOREST-003 | [Maps, ledgers, projection identity, snapshot isolation, reproducibility, aggregation, and consistency](FOREST_PROJECTION_CONSISTENCY_CONTRACT.md) | Draft after nine-role review | FOREST-002, IDENTITY-001, and EVIDENCE-001 |
| CAUSALITY-001 | Confidence, unknowns, and source attribution | Planned | FOREST-003 |
| PREDICTION-001 | Prediction and held-out evaluation | Planned | CAUSALITY-001 |
| VALIDATION-001 | Validation coverage and capability preservation | Planned | Crates Series, PERF-Q35, FOREST-003 |
| PLANNING-001 | Federated Blueprint Plan, owner-specific scope closures, per-command Cargo plans, resource envelopes, fallback, and adaptive replanning | Planned | PREDICTION-001, VALIDATION-001, SCOPE-001, IDENTITY-001, BLUE-Q02, and BLUE-Q03 |
| RESOLUTION-001 | Blueprint Plan selection, resolution policy, alternatives, and human decision contract | Planned | CAUSALITY-001, PREDICTION-001, and PLANNING-001 |
| EXECUTION-001 | Action approval, executable plan projection, execution, rollback, cleanup, and audit | Planned | RESOLUTION-001, PLANNING-001, and GOVERNANCE-001 |
| TRUST-001 | Provenance, trust, privacy, security, ref authority, retention, revocation, and deletion | Planned | GOVERNANCE-001, PERF-Q30, BLUE-Q01, and IDENTITY-001 |
| CONNECTOR-001 | [Replaceable connector manifests, maturity, owner semantics, failure, lifecycle, Microsoft profiles, and governed MCP](FERRIS_CONNECTOR_CONTRACT.md) | Draft after nine-role review | PRODUCT-001, CONTRACT-001, EVIDENCE-001, TRUST-001, and GOVERNANCE-001 |
| FERRIS-001 | Evidence packet and upstream packet integration | Planned | FOREST-003, TRUST-001, GOVERNANCE-001, and CONNECTOR-001 |
| VIEW-001 | [Shared `ferris`, `cargo ferris`, and governed MCP command, scope-default, output, and explanation contract](FERRIS_VIEW_CONTRACT.md) | Draft after nine-role review | PRODUCT-001, FERRIS-001, GOVERNANCE-001, and CONNECTOR-001 |
| CONFORMANCE-001 | [Held-out workflow, CLI/MCP parity, scope, AI, governance, connector, fallback, failure, removal, platform, and acceptance contract](FERRIS_CONFORMANCE_CONTRACT.md) | Draft framework after nine-role review | All preceding specifications |

No implementation is authorized by listing a specification here.

## Review records

| Specification | Review | Disposition |
|---|---|---|
| FOREST-001 | [Nine-role review](reviews/FOREST-001-ROLE-REVIEW.md) | Accepted as Draft; implementation and Proposed status withheld |
| PRODUCT-001, VIEW-001, CONFORMANCE-001 | [Ferris public-contract review](reviews/FERRIS-PUBLIC-CONTRACTS-ROLE-REVIEW.md) | Accepted as Draft; exact fixtures, commands, schemas, thresholds, and support commitments remain blockers |
| GOVERNANCE-001, CONNECTOR-001 | [Microsoft integration review](../plans/reviews/FERRIS-MICROSOFT-INTEGRATION-ROLE-REVIEW.md) | Accepted as Draft; exact policies, connector versions, tenant controls, MCP schemas, and fixtures remain blockers |
| CONTRACT-001, PLATFORM-001, APPLICATION-001 | [Gate A review](reviews/FERRIS-GATE-A-ROLE-REVIEW.md) | Accepted as Draft; canonical schemas, exact contract/profile/application fixtures, cross-platform execution, migration, renewal, rollback, and removal remain blockers |
| SCOPE-001, FOREST-002, IDENTITY-001, EVIDENCE-001, FOREST-003 | [Gate B foundations review](reviews/FERRIS-GATE-B-FOUNDATIONS-ROLE-REVIEW.md) | Accepted as Draft; frozen schemas, portable fixtures, cross-platform evidence, concurrency controls, and independent projection reproduction remain blockers |

## Crates Series gate

FOREST-002, EVIDENCE-001, VALIDATION-001, and CONFORMANCE-001
depend directly on ECOS-Q01 through ECOS-Q12.

The Crates Series completed on 2026-08-10 and defined:

- crate and dependency scope;
- ecosystem capability ownership;
- feature and version closures;
- interchange and async-runtime contracts;
- native, build-script, macro, unsafe, and platform boundaries;
- stewardship, provenance, licensing, advisory, and lifecycle evidence; and
- compatibility-profile renewal and removal.

These concepts cannot be retrofitted safely after a Blueprint schema or product
has already hardened. The dependency is now satisfied for specification work;
no implementation authority follows from gate completion.

## Review requirements

Every Ferris program specification receives:

- Rust Safety Steward review;
- Compiler Performance Engineer review;
- Interop Boundary Auditor review;
- AI Assurance Skeptic review;
- Ecosystem Strategist review;
- Rust Maintainer review;
- Native Platform Adopter review;
- Scope Keeper review; and
- Validation Checker review.

Role objections remain part of the specification record.

## Change control

A specification change records:

- the motivating finding, question, or held-out failure;
- affected components and dependent specifications;
- compatibility and migration consequences;
- new validation or conformance cases;
- rollback or supersession path; and
- role approvals required.

Future implementation code MUST reference the specification version it claims
to implement.
