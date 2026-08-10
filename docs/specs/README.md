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
| CAUSALITY-001 | [Causal claim classes, source attribution, stage-specific explanations, confidence, confounders, unknowns, and AI boundaries](FERRIS_CAUSALITY_CONTRACT.md) | Draft after nine-role review | FOREST-003 and Build Latency Measurement Contract |
| PREDICTION-001 | [Immutable predictions, evidence cutoffs, held-out evaluation, calibration, error analysis, fallback, and model accountability](FERRIS_PREDICTION_CONTRACT.md) | Draft after nine-role review | CAUSALITY-001, SCOPE-001, and Build Latency Measurement Contract |
| VALIDATION-001 | [Requirements, selection, coverage dimensions, mandatory gates, capability preservation, full-reference comparison, and fallback](FERRIS_VALIDATION_COVERAGE_CONTRACT.md) | Draft after nine-role review | PLATFORM-001, SCOPE-001, FOREST-003, PERF-Q35, and Crates Series |
| PLANNING-001 | [Versioned non-executable Blueprint Plans, owner closures, Cargo activities, validation, resources, artifact economics, barriers, and replanning](FERRIS_BLUEPRINT_PLANNING_CONTRACT.md) | Draft after nine-role review | APPLICATION-001, SCOPE-001, IDENTITY-001, PREDICTION-001, VALIDATION-001, BLUE-Q02, and BLUE-Q03 |
| RESOLUTION-001 | [Blueprint Plan eligibility, hard-constraint ordering, alternatives, immutable decisions, AI boundaries, and human resolution](FERRIS_RESOLUTION_CONTRACT.md) | Draft after nine-role review | CAUSALITY-001, PREDICTION-001, PLANNING-001, and GOVERNANCE-001 |
| TRUST-001 | [Provenance, integrity, consumer trust, privacy, ref authority, retention, revocation, deletion, and artifact-use controls](FERRIS_TRUST_CONTRACT.md) | Draft after nine-role review | GOVERNANCE-001, IDENTITY-001, PERF-Q30, and BLUE-Q01 |
| EXECUTION-001 | [Action requests, exact executable projections, approval binding, preflight, isolation, deviations, rollback, cleanup, and audit](FERRIS_EXECUTION_CONTRACT.md) | Draft after nine-role review | RESOLUTION-001, PLANNING-001, TRUST-001, and GOVERNANCE-001 |
| CONNECTOR-001 | [Replaceable connector manifests, maturity, owner semantics, failure, lifecycle, Microsoft profiles, and governed MCP](FERRIS_CONNECTOR_CONTRACT.md) | Draft after nine-role review | PRODUCT-001, CONTRACT-001, EVIDENCE-001, TRUST-001, and GOVERNANCE-001 |
| FERRIS-001 | [Portable evidence packets, completeness, redaction, trust, action history, owner-aligned upstream adaptation, and publication boundaries](FERRIS_EVIDENCE_PACKET_CONTRACT.md) | Draft after nine-role review | FOREST-003, RESOLUTION-001, EXECUTION-001, TRUST-001, GOVERNANCE-001, and CONNECTOR-001 |
| VIEW-001 | [Shared `ferris`, `cargo ferris`, and governed MCP command, decision, action, scope-default, output, explanation, and exit contract](FERRIS_VIEW_CONTRACT.md) | Draft after final nine-role review | PRODUCT-001, SCOPE-001, PLANNING-001, RESOLUTION-001, TRUST-001, EXECUTION-001, GOVERNANCE-001, CONNECTOR-001, and FERRIS-001 |
| CONFORMANCE-001 | [Executable product, schema, identity, evidence, planning, prediction, validation, governance, trust, action, connector, packet, platform, failure, and removal proof](FERRIS_CONFORMANCE_CONTRACT.md) | Draft after final nine-role review | All preceding specifications |

No implementation is authorized by listing a specification here.

## Review records

| Specification | Review | Disposition |
|---|---|---|
| FOREST-001 | [Nine-role review](reviews/FOREST-001-ROLE-REVIEW.md) | Accepted as Draft; implementation and Proposed status withheld |
| PRODUCT-001, VIEW-001, CONFORMANCE-001 | [Ferris public-contract review](reviews/FERRIS-PUBLIC-CONTRACTS-ROLE-REVIEW.md) | Accepted as Draft; exact fixtures, commands, schemas, thresholds, and support commitments remain blockers |
| GOVERNANCE-001, CONNECTOR-001 | [Microsoft integration review](../plans/reviews/FERRIS-MICROSOFT-INTEGRATION-ROLE-REVIEW.md) | Accepted as Draft; exact policies, connector versions, tenant controls, MCP schemas, and fixtures remain blockers |
| CONTRACT-001, PLATFORM-001, APPLICATION-001 | [Gate A review](reviews/FERRIS-GATE-A-ROLE-REVIEW.md) | Accepted as Draft; canonical schemas, exact contract/profile/application fixtures, cross-platform execution, migration, renewal, rollback, and removal remain blockers |
| SCOPE-001, FOREST-002, IDENTITY-001, EVIDENCE-001, FOREST-003 | [Gate B foundations review](reviews/FERRIS-GATE-B-FOUNDATIONS-ROLE-REVIEW.md) | Accepted as Draft; frozen schemas, portable fixtures, cross-platform evidence, concurrency controls, and independent projection reproduction remain blockers |
| CAUSALITY-001, PREDICTION-001, VALIDATION-001, PLANNING-001 | [Gate B completion review](reviews/FERRIS-GATE-B-COMPLETION-ROLE-REVIEW.md) | Accepted as Draft; frozen schemas, held-out workflows, seeded failures, calibrated error, resource measurements, replanning, and removal remain blockers |
| RESOLUTION-001, TRUST-001, EXECUTION-001, FERRIS-001 | [Gate C review](reviews/FERRIS-GATE-C-ROLE-REVIEW.md) | Accepted as Draft; exact policy, trust, isolation, failure, rollback, deletion, packet, connector, MCP, and external-publication controls remain blockers |
| PRODUCT-001 through CONFORMANCE-001 | [Final specification convergence review](reviews/FERRIS-SPECIFICATION-CONVERGENCE-ROLE-REVIEW.md) | Complete 22-specification Draft spine accepted; all Proposed statuses and implementation remain withheld |

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
