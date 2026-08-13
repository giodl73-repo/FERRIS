# Pulse 03: Platform Profile Schema Harness

Status: Authorized; implementation and validation pending
Implementation authority: Bounded to this document

## Goal and authority

Add one repository-owned, test-only Rust harness that executes the nine frozen
schema controls against the canonical `ferris.platform-profile/v1` fixture
boundary.

This pulse authorizes:

- one integration test under `crates/ferris-core/tests/`;
- strict duplicate-member detection before ordinary deserialization;
- the frozen 4 MiB byte bound;
- exact schema-version classification;
- required top-level member and unknown-member checks;
- bounded output-visible identifier policy;
- exact-one path-or-URI source-reference checks;
- evidence-state checks required by the frozen controls;
- deterministic application of the nine control mutations; and
- Windows and Unix development validation.

The harness is test support only. It is not a production parser, library API,
CLI command, JSON Schema replacement, profile generator, owner adapter, or
compatibility decision.

## Result classes

The test harness uses internal assertions for:

| Class | Meaning |
|---|---|
| `valid` | The frozen base exemplar satisfies the bounded harness policy |
| `unsupported` | A well-shaped `ferris.platform-profile/*` schema version is not v1 |
| `invalid` | JSON, duplicate, member, metadata, state, or source shape is invalid |
| `blocked` | Input bytes exceed the frozen 4 MiB bound |

These are test classifications, not new Ferris process-result behavior.

## Acceptance

- the base fixture is accepted;
- all nine controls produce their exact expected class;
- duplicate members fail at any object depth;
- oversized input blocks before deserialization;
- unsupported schema remains distinct from invalid shape;
- unknown top-level members fail closed;
- ambiguous path-plus-URI source references fail;
- unknown success-shaped evidence state fails;
- the synthetic unsafe output-visible identifier fails;
- no production source or Cargo dependency changes;
- full repository gates pass on Windows and Unix; and
- all nine roles accept the measured boundary.

## Stop conditions

Stop rather than widening this pulse if work requires:

- adding a runtime or development dependency;
- exposing schema types from `ferris-core`;
- invoking Cargo or another owner command from Ferris product code;
- completing a profile family or collecting owner stage evidence;
- interpreting evidence as compatibility, safety, support, trust, or approval;
- modifying `profile-diff`; or
- accessing hidden held-out material.

## Evidence

- [Frozen controls](../../../../tests/fixtures/platform-profiles/schema/controls.json)
- [Authorization review](../../../../docs/plans/reviews/PULSE-03-SCHEMA-HARNESS-ROLE-REVIEW.md)
