# Ferris Validation Plan Schemas

Status: Product-facing closed structural success schemas
Dialect: JSON Schema Draft 2020-12

This directory publishes checked-in schemas for the current successful bounded
`validation-plan` machine output. The schemas are for downstream consumers
that need to validate Ferris JSON; they do not authorize Cargo execution,
infer repository-owned gates, or convert selected-package planning into a
full-suite, release, platform, support, or CI-equivalence claim.

These are closed structural success schemas, not exact serializers. They
validate the representable JSON shape and scalar/array constraints described
below. Runtime semantic conformance still requires Ferris-aware checks for
relationships JSON Schema cannot portably express across arrays and fields.

## Schemas

- [`ferris.validation-plan.v0.schema.json`](ferris.validation-plan.v0.schema.json)
  is the closed structural success schema for the non-null
  `ferris.validation-plan/v0` record.
- [`ferris.command-result.v2.schema.json`](ferris.command-result.v2.schema.json)
  is the closed structural `validation-plan` success specialization of the
  shared `ferris.command-result/v2` Rust envelope.

The existing `profile-diff` specialization remains documented separately in
[`../../simulations/profile-diff-held-out/schemas/`](../../simulations/profile-diff-held-out/schemas/README.md).

The command-result specialization intentionally binds
`semantic_command_id:"validation-plan"`, `result_class:"success"`,
`process_exit_code:0`, an empty diagnostics array, and a non-null
`ferris.validation-plan/v0` record. It does not publish a stable schema
promise for `invalid`, `unsupported`, `incomplete`, `blocked`, or `internal`
envelopes; those remain runtime-visible but outside this bounded
consumer-facing contract.

## Contract boundary

The schemas freeze the current public success structure only. They do not
establish:

- execution of `cargo check`, `cargo test`, Clippy, formatting, or any other
  repository-owned validation gate;
- repository-specific selection rules beyond the current explicit package and
  path inputs;
- a generic schema for every Ferris `ferris.command-result/v2` command;
- stable schemas for non-success `validation-plan` failure envelopes;
- diagnostic release custody, scorer records, or held-out profile-diff
  infrastructure; or
- support for future `validation-plan` schema versions.

## Semantic conformance outside portable JSON Schema

Successful Ferris serialization additionally guarantees runtime relationships
that these documents intentionally do not claim to encode exactly:

- each selected activity's `package_identities` equals the selected package
  identity list in selected-package serializer order;
- each fallback activity's `package_identities` equals the fallback package
  identity list in fallback-package serializer order;
- `fallback.required_by_inputs` is derived from whether any input disposition
  is `full_workspace_fallback`;
- selected and fallback package identities are unique by their `identity` key,
  not merely unique as whole JSON objects;
- selected package identities and input `package_identity` values refer to
  fallback package identities; and
- evidence `workspace_id` and manifest command argument agree with their
  corresponding record fields.

The schemas check lexical identity/digest forms but do not rederive identities
or digests from other fields. Consumers that require serializer-semantic
conformance must perform those checks separately.

## Strictness

Every object schema is closed with `additionalProperties:false`. The
command-result specialization requires:

- `result_class:"success"` and `process_exit_code:0`;
- `diagnostics:[]`; and
- `record` to conform to `ferris.validation-plan/v0`.

The validation-plan record schema freezes the current visible enums,
non-executable boundary, Cargo metadata evidence framing, and the
selected-package versus full-workspace fallback split. Representable current
serializer invariants include bounded non-empty inputs, whole-item uniqueness,
unique reason and activity identity arrays, fixed current unknown/limitation
cardinality, two activities for every non-empty package scope, and lexical
Cargo workspace package identities. `uniqueItems:true` does not replace the
separate identity-key uniqueness rule above.

The dedicated test reads and applies both checked-in documents through a
dependency-free test-local validator for exactly the used Draft 2020-12
keyword subset. It resolves root and local `$ref` values and rejects any
unsupported schema keyword. Real selected-package and fallback CLI successes,
negative structural mutations, and separate semantic-conformance controls are
all exercised against this boundary.
