# Pulse 31: Public Profile-Evidence Input Contract

Status: Complete; public contract and schema released
Implementation authority: Governance, public schema/fixtures, review, and
test-only validation only

## Goal

Close the public-rule availability gap identified by the invalid Pulse 30
result by publishing the complete existing `ferris.profile-evidence/v0` input
acceptance boundary without changing production behavior.

Pulse 31 does not retry, resume, reseed, rescore, reuse, continue, correlate,
or infer from Pulse 30. Further Pulse 30 launches remain prohibited.

## Released contract

The release contains:

- one normative public input contract;
- one recursive Draft 2020-12 parsed-value schema;
- six complete positive input fixtures covering scalar, array, object, nested,
  one-character, and 256-character metadata/member-name boundaries;
- 33 declared-invalid controls covering filesystem, byte-size, JSON framing,
  duplicates, recursive keys, schema, shape, and metadata classifications;
- one dependency-free Rust integration test with a strict duplicate-preserving
  parser and custom acceptance validator; and
- one nine-role review.

The root object is closed to exactly `schema`, `profile_id`, `revision`,
`consumer`, and `sections`. `sections` is closed to exactly twelve required
keys. Section values may be any recursive JSON value, while every object
member name at every depth and every identity string uses 1 through 256
visible ASCII characters.

The complete file limit is 1,048,576 bytes. JSON Schema validates parsed
values; regular-file availability, raw size, malformed JSON, duplicate
members, and recursive parsing precedence remain normative companion rules.

## Evidence

- [Normative input contract](../../../../docs/simulations/profile-diff-held-out/INPUT_PROFILE_EVIDENCE.md)
- [Recursive schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.profile-evidence.v0.schema.json)
- [Fixture index](../../../../docs/simulations/profile-diff-held-out/fixtures/README.md)
- [Negative controls](../../../../docs/simulations/profile-diff-held-out/fixtures/profile-evidence-v0-mutations.json)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-31-PROFILE-EVIDENCE-INPUT-CONTRACT-ROLE-REVIEW.md)
- [Test-only validator](../../../../crates/ferris-cli/tests/profile_evidence_input_contract.rs)

Schema raw SHA-256:
`sha256:67946b1a392d2d7537d487d343bee31439606c76b2d71862b97ff46641c3d62b`

## Decision

The public input contract is complete enough for an independent generator to
construct accepted and declared-invalid inputs without source or test access.
The Pulse 30 public blocker is documented and prospectively closed as a public
contract gap only. Pulse 30 remains invalid with a null conclusion, and this
release grants no diagnostic launch, product fix, score, certification,
support, or PLATFORM-001 advancement authority.

## Stop conditions

Stop rather than modify production parsing, add a dependency, generate a
profile, relaunch Pulse 30, access hidden material, weaken duplicate/key/size
rules, infer a category conclusion, or change PLATFORM-001 status.
