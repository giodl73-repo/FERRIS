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
| [Query Forest component model](FOREST_COMPONENT_MODEL.md) | Draft; FOREST-001 | Defines the precise OSPREY components and forbids a monolithic Forest |

## OSPREY program specification sequence

The
[OSPREY program](../plans/OSPREY_PROGRAM.md) defines program sequencing.
Specifications are developed in this order.

| ID | Specification | Status | Primary dependency |
|---|---|---|---|
| FOREST-001 | Query Forest component model | Draft after nine-role review | PERF-Q01 through PERF-Q36 |
| CONTRACT-001 | Rust API, RUNE, ABI, component, and wire contract identity and compatibility | Planned | ECOS-Q03, ECOS-Q07 through ECOS-Q12, RUNE v1 |
| PLATFORM-001 | Enterprise profile selection, support, servicing, substitution, renewal, removal, and rollback | Planned | CONTRACT-001 and ECOS-Q11 through ECOS-Q12 |
| FOREST-002 | Canonical nodes, edges, states, and serialization schema | Planned | FOREST-001 and Crates Series |
| IDENTITY-001 | Scope, identity, compatibility, and lineage | Planned | FOREST-002 |
| EVIDENCE-001 | Evidence adapter and upstream ownership contracts | Planned | FOREST-002 and Crates Series |
| FOREST-003 | Maps, ledgers, projections, and consistency rules | Planned | FOREST-002, IDENTITY-001, EVIDENCE-001 |
| CAUSALITY-001 | Confidence, unknowns, and source attribution | Planned | FOREST-003 |
| PREDICTION-001 | Prediction and held-out evaluation | Planned | CAUSALITY-001 |
| RESOLUTION-001 | Resolution policy and human decision contract | Planned | CAUSALITY-001 and PREDICTION-001 |
| EXECUTION-001 | Action approval, execution, rollback, cleanup, and audit | Planned | RESOLUTION-001 |
| VALIDATION-001 | Validation coverage and capability preservation | Planned | Crates Series, PERF-Q35, FOREST-003 |
| TRUST-001 | Provenance, trust, privacy, security, retention, and deletion | Planned | PERF-Q30 and IDENTITY-001 |
| FERRIS-001 | Evidence packet and upstream packet integration | Planned | FOREST-003 through TRUST-001 |
| VIEW-001 | User views and explanation contract | Planned | FOREST-003 through FERRIS-001 |
| CONFORMANCE-001 | Held-out workflow, conformance, and acceptance tests | Planned | All preceding specifications |

No implementation is authorized by listing a specification here.

## Review records

| Specification | Review | Disposition |
|---|---|---|
| FOREST-001 | [Nine-role review](reviews/FOREST-001-ROLE-REVIEW.md) | Accepted as Draft; implementation and Proposed status withheld |

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

These concepts cannot be retrofitted safely after an OSPREY schema or product
has already hardened. The dependency is now satisfied for specification work;
no implementation authority follows from gate completion.

## Review requirements

Every OSPREY program specification receives:

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
