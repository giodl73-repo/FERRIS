# FERRIUM Specification Registry

Status: Active
Implementation authority: None unless a separately approved pulse says
otherwise

## Purpose

FERRIUM plans explain direction and sequencing. Specifications define precise,
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

## Existing FERRIUM contracts

| Contract | Status | Purpose |
|---|---|---|
| [Build latency measurement contract](BUILD_LATENCY_MEASUREMENT_CONTRACT.md) | Adopted for research | Defines fixtures, workloads, evidence, statistics, privacy, validation-selection, and prototype gates |
| [Rust performance contribution packet](RUST_PERFORMANCE_CONTRIBUTION_PACKET.md) | Adopted by PERF-Q36 | Defines one reviewable upstream performance contribution artifact |
| [OSPREY Query Forest component model](OSPREY_QUERY_FOREST_COMPONENT_MODEL.md) | Planning; OSPREY-SPEC-001 | Defines the precise OSPREY components and forbids a monolithic Forest |

## OSPREY specification sequence

The
[OSPREY program](../plans/OSPREY_PROGRAM.md) defines program sequencing.
Specifications are developed in this order.

| ID | Specification | Status | Primary dependency |
|---|---|---|---|
| OSPREY-SPEC-001 | Query Forest component model | Planning; recorded | PERF-Q01 through PERF-Q36 |
| OSPREY-SPEC-002 | Canonical nodes, edges, states, and serialization schema | Planned | SPEC-001 and Crates Series |
| OSPREY-SPEC-003 | Scope, identity, compatibility, and lineage | Planned | SPEC-002 |
| OSPREY-SPEC-004 | Evidence adapter and upstream ownership contracts | Planned | SPEC-002 and Crates Series |
| OSPREY-SPEC-005 | Maps, ledgers, projections, and consistency rules | Planned | SPEC-002 through SPEC-004 |
| OSPREY-SPEC-006 | Causality, confidence, unknowns, and source attribution | Planned | SPEC-005 |
| OSPREY-SPEC-007 | Prediction and held-out evaluation | Planned | SPEC-006 |
| OSPREY-SPEC-008 | Resolution policy and human decision contract | Planned | SPEC-006 and SPEC-007 |
| OSPREY-SPEC-009 | Action approval, execution, rollback, cleanup, and audit | Planned | SPEC-008 |
| OSPREY-SPEC-010 | Validation coverage and capability preservation | Planned | Crates Series, PERF-Q35, SPEC-005 |
| OSPREY-SPEC-011 | Provenance, trust, privacy, security, retention, and deletion | Planned | PERF-Q30 and SPEC-003 |
| OSPREY-SPEC-012 | FERRIS evidence packet and upstream packet integration | Planned | SPEC-005 through SPEC-011 |
| OSPREY-SPEC-013 | User views and explanation contract | Planned | SPEC-005 through SPEC-012 |
| OSPREY-SPEC-014 | Held-out workflow, conformance, and acceptance tests | Planned | SPEC-001 through SPEC-013 |

No implementation is authorized by listing a specification here.

## Crates Series gate

OSPREY-SPEC-002, SPEC-004, SPEC-010, and the final conformance specification
depend directly on ECOS-Q01 through ECOS-Q12.

The Crates Series must define:

- crate and dependency scope;
- ecosystem capability ownership;
- feature and version closures;
- interchange and async-runtime contracts;
- native, build-script, macro, unsafe, and platform boundaries;
- stewardship, provenance, licensing, advisory, and lifecycle evidence; and
- compatibility-profile renewal and removal.

These concepts cannot be retrofitted safely after an OSPREY schema or product
has already hardened.

## Review requirements

Every OSPREY specification receives:

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
