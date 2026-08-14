# Pulse 21: RUNE v1 Dependency Reconciliation

Status: Complete; CONTRACT-001 dependency satisfied
Implementation authority: Documentation, machine-readable evidence, review,
and test-only validation only

## Goal

Reconcile the RUNE evidence already bound by the controlled platform-profile
fixtures with RUNE's accepted v1 contract and release-readiness baseline,
without changing either repository's implementation or any frozen fixture.

## Bounded authority

This pulse authorizes only:

- one exact dependency decision;
- one deterministic machine-readable receipt and closed Draft 2020-12 schema;
- public negative-control mutations and test-only validation;
- one nine-role review; and
- documentation updates that close the RUNE dependency blocker.

It authorizes no RUNE change, FERRIS production behavior, fixture
regeneration, profile identity or digest change, held-out access, retry,
rescore, support claim, compatibility expansion, runtime-host claim, or
PLATFORM-001 advancement.

## Exact decision

FERRIS recognizes RUNE revision
`194449444624fb10add4137cb0da8d0327164fa7`, already bound by the controlled
semantic fixtures, as satisfying CONTRACT-001's Typebook/RUNE v1
**contract-baseline dependency**.

Here, RUNE v1 means the accepted contract and release-readiness baseline:

- `docs/release-readiness.md` says RUNE v1 is ready as publishable contract
  infrastructure;
- the v1 release-readiness wave is closed; and
- `docs/vtrace/SPECIFICATION_BASELINE.md` contains eight accepted
  specification rows.

This decision does not claim Cargo SemVer `1.0.0` publication. The RUNE Cargo
workspace remains `0.1.0`, the controlled descriptor collection and neutral
profile remain `v0`, and no Git `v1.0.0` tag is present or claimed.

## Unchanged binding

The FERRIS fixture binding remains:

| Field | Value |
|---|---|
| Repository | `https://github.com/giodl73-repo/RUNE.git` |
| Revision | `194449444624fb10add4137cb0da8d0327164fa7` |
| Crate version | `0.1.0` |
| Descriptor collection | `v0` |
| Neutral profile | `rune.neutral_descriptor_json` |
| Neutral profile version | `v0` |

No fixture bytes are regenerated. No profile identity or digest changes. No
RUNE or FERRIS production behavior changes.

## PLATFORM-001 effect

The RUNE dependency is satisfied and is no longer a PLATFORM-001 blocker.
PLATFORM-001 remains Draft solely because the valid Pulse 17 first score
failed `process-exit-agreement`.

Pulse 19's bounded `no-reproduction` result and Pulse 20's prospective
sanitized-reproducer protocol remain unchanged. Neither erases, retries,
rescores, or explains the closed Pulse 17 result.

## Evidence

- [Dependency receipt](../../../../docs/plans/validation/PULSE-21-RUNE-V1-DEPENDENCY-RECEIPT.json)
- [Receipt schema](../../../../docs/plans/validation/ferris.rune-v1-dependency-receipt.v1.schema.json)
- [Mutation controls](../../../../docs/plans/validation/PULSE-21-RUNE-V1-DEPENDENCY-RECEIPT-MUTATIONS.json)
- [Validation index](../../../../docs/plans/validation/PULSE-21-RUNE-V1-DEPENDENCY.md)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-21-RUNE-V1-DEPENDENCY-ROLE-REVIEW.md)
- [Test-only validator](../../../../crates/ferris-cli/tests/rune_v1_dependency_receipt.rs)

## Stop conditions

Stop rather than widening this pulse if reconciliation would require:

- changing RUNE or FERRIS product code;
- changing semantic fixture bytes, identities, digests, or versions;
- claiming SemVer `1.0.0`, a Git v1 tag, broad ecosystem compatibility,
  runtime-host behavior, or unrecorded support;
- changing the valid Pulse 17 failure; or
- advancing PLATFORM-001 despite that remaining mandatory failure.
