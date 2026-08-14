# Pulse 02: Canonical Platform Profile Fixture Contract

Status: Complete; immutable-cutoff schema validation passed
Implementation authority: Schema documents and frozen controls only

## Goal and authority

Freeze the canonical controlled-fixture record used by the PLATFORM-001
conformance program while keeping the existing
`ferris.profile-evidence/v0` command input experimental and semantically
uninterpreted.

This pulse authorizes only:

- the `ferris.platform-profile/v1` JSON Schema;
- canonical serialization, digest, strictness, extension, privacy, and size
  rules;
- one explicitly incomplete valid schema exemplar;
- exact malformed, duplicate, unsupported, ambiguous, unsafe-metadata,
  unknown-state, and oversized controls;
- the lossy projection map into experimental v0 diff sections;
- one exact RUNE controlled-fixture revision; and
- the nine-role schema review.

It does not authorize a parser, validator, test harness, owner command,
completed family, generated profile, product behavior, hidden held-out
material, support decision, or specification status change.

## Frozen identities

| Subject | Identity |
|---|---|
| Platform profile schema | `ferris.platform-profile/v1` |
| Schema dialect | JSON Schema 2020-12 |
| Maximum canonical record | 4 MiB |
| Canonical digest | SHA-256 over the documented domain frame |
| Experimental diff input | `ferris.profile-evidence/v0`, unchanged |
| RUNE repository revision | `194449444624fb10add4137cb0da8d0327164fa7` |
| RUNE neutral profile | `rune.neutral_descriptor_json/v0` |

This pulse did not itself establish RUNE v1 status. Pulse 21 later recognized
the same already-bound revision as satisfying the accepted RUNE v1 contract
and release-readiness baseline while retaining Cargo `0.1.0`, collection
`v0`, profile `v0`, and no-tag facts.

## Acceptance

- canonical and experimental schemas are visibly distinct;
- unknown members fail closed except namespaced extensions;
- duplicate JSON members are rejected before deserialization;
- every required profile identity and section has a typed schema;
- required family, evidence, stage, contract, environment, support, and
  lifecycle states remain distinct;
- source references require exactly one path or URI;
- output-visible metadata has bounded syntax and an explicit secret boundary;
- canonicalization and digest framing are deterministic;
- the valid exemplar is syntactically valid JSON and conforms to the schema;
- exact negative-control mutations are frozen; and
- all nine roles accept the schema boundary.

## Stop conditions

Stop rather than widening this pulse if work requires:

- production parsing or validation code;
- owner command execution;
- copying RUNE implementation into FERRIS;
- declaring RUNE v1 complete;
- treating the schema exemplar as a completed family;
- interpreting a profile as compatibility, trust, support, approval, or
  correctness;
- changing the existing profile-diff command; or
- advancing PLATFORM-001.

## Evidence

- [Schema contract](../../../../docs/schemas/platform-profile/README.md)
- [JSON Schema](../../../../docs/schemas/platform-profile/ferris.platform-profile.v1.schema.json)
- [Schema controls](../../../../tests/fixtures/platform-profiles/schema/README.md)
- [Validation receipt](../../../../docs/plans/validation/PULSE-02-PLATFORM-PROFILE-SCHEMA.md)
- [Nine-role review](../../../../docs/plans/reviews/PLATFORM-PROFILE-SCHEMA-ROLE-REVIEW.md)
