# Pulse 01: Validation Plan Schema Publication

Status: Complete after the single permitted corrective pass
Implementation authority: Bounded to this document and review correction
Budget: One publication attempt, one corrective pass on `f183157`, no successor

## Goal

Publish checked-in Draft 2020-12 closed structural success schemas and
deterministic schema-contract tests for the current successful bounded
`validation-plan` JSON output so downstream consumers can validate the
representable contract without changing Ferris runtime behavior.

## Authorized files

- `docs/schemas/validation-plan/`;
- `crates/ferris-cli/tests/`;
- `README.md`;
- `docs/simulations/profile-diff-held-out/schemas/README.md`;
- this wave, its review record, and directly related documentation.

## Required behavior

- add one closed structural success schema for the
  `ferris.validation-plan/v0` record;
- add one closed structural schema for the `validation-plan`
  `ferris.command-result/v2` success specialization;
- keep the boundary product-facing rather than diagnostic-release custody;
- validate the checked-in schema documents with a dependency-free test-local
  validator that supports exactly their Draft 2020-12 keyword subset, resolves
  root/local `$ref` values, and fails on unsupported keywords;
- apply those documents to real CLI success outputs for selected-package
  closure and full-workspace fallback;
- include deterministic negative mutation controls for extra fields, missing
  required fields, invalid enums, cardinality, and whole-item uniqueness;
- retain separate semantic-conformance assertions for cross-field identity
  order/equality, identity-key uniqueness, fallback derivation, and record
  references that portable JSON Schema does not express; and
- document that the schema is command-specific and separate from the existing
  `profile-diff` specialization.

## Prohibited behavior

- changing runtime command behavior, typed results, or existing CLI semantics;
- adding a new validator dependency or external schema service;
- widening into a generic command-result publication program;
- inventing repository-owned validation gates, execution behavior, or new
  runtime data;
- promising stable non-success `validation-plan` failure envelopes;
- extending diagnostic custody infrastructure or held-out scoring; or
- creating a successor pulse.

The corrective pass also prohibits describing either document as an exact
serializer. They are closed structural success schemas; runtime semantic
relationships remain separately asserted and documented.

## Acceptance

- `cargo test -p ferris-cli --test validation_plan_schema`;
- `cargo test -p ferris-cli --test cli validation_plan_`;
- `cargo check --workspace --locked`;
- `rustfmt --edition 2024 --check crates\ferris-cli\tests\validation_plan_schema.rs`;
- JSON parsing of both checked-in schema documents; and
- `git diff --check`.

## Stop conditions

Stop the pulse rather than widening scope if it requires:

- a runtime code change;
- a new validator dependency;
- another corrective pass;
- a successor pulse or another architectural layer; or
- unstable generic command-result duplication large enough to outweigh the
  downstream validation value.

## Removal

Removal requires deleting the schema directory, the schema-contract test file,
this pulse's documentation, and directly related README updates. It MUST NOT
require changing Ferris runtime code or any fixture Cargo manifest.

## Review

Measured outcome is recorded in
`docs/plans/reviews/FERRIS-VALIDATION-PLAN-SCHEMA-REVIEW.md`.
