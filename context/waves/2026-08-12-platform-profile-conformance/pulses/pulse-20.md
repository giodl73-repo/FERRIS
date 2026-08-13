# Pulse 20: Prospective Post-Score Diagnostic Release

Status: Complete; prospective protocol frozen
Implementation authority: Public governance, schema, fixtures, and test-only
validation only

## Goal

Preserve blind held-out certification while making future failures actionable
through a precommitted, independently produced sanitized reproducer.

This pulse does not alter Pulse 17. Its fixture remains quarantined and cannot
be accessed, retried, rescored, reused, reconstructed, or disclosed.

## Authority

Pulse 20 authorizes:

- one prospective public protocol;
- one Draft 2020-12 receipt schema;
- harmless positive and negative public fixtures; and
- test-only schema, identity, and policy validation.

It authorizes no Ferris product fix, CLI change, hidden-material access,
custodian request, certification claim, or PLATFORM-001 advancement. Any
product correction requires a later separately approved pulse.

## Decision

Future experimental held-out programs SHOULD precommit the
`sanitized-reproducer` disclosure tier before hidden fixture construction.
After the immutable first score, an independent custodian may create a new
minimal public reproducer that demonstrates only the released failure category
and passes the anti-overlap gates in the prospective protocol.

The original certification result remains immutable. The original fixture and
the released reproducer are both permanently ineligible for future
certification. A future certification claim requires a newly constructed
held-out package.

## Evidence

- [Why quarantine](../../../../docs/simulations/profile-diff-held-out/WHY_QUARANTINE.md)
- [Prospective release protocol](../../../../docs/simulations/profile-diff-held-out/POST_SCORE_DIAGNOSTIC_RELEASE.md)
- [Receipt schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.post-score-diagnostic-release.v1.schema.json)
- [Public fixtures](../../../../docs/simulations/profile-diff-held-out/fixtures/post-score-diagnostic-release.json)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-20-POST-SCORE-DIAGNOSTIC-RELEASE-ROLE-REVIEW.md)

