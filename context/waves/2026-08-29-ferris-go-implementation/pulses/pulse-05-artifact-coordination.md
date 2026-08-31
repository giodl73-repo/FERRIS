# Pulse 05: Artifact Coordination

Status: Proposed; blocked by pulse 03

Implementation authority: None

Parent work package: GO-WP-006

## Outcome

Build once and fan out only when producer and consumer contexts are explicitly
compatible.

## Authorized Slice

- Model producer attempt, source, toolchain, platform, target, profile,
  features, configuration, manifest, and digest identity.
- Support one bounded local fan-out/fan-in fixture.
- Add no remote artifact service or generic cache claim.

## Accepted Fixture

One owner producer artifact feeds multiple compatible test consumers and joins
with complete lineage.

## Rejected Fixture

Any compatibility dimension differs, the producer fails, lineage is incomplete,
or a consumer requests an undeclared artifact.

## Verification

- L0: one mutation per compatibility field.
- L1: nextest-shaped build/archive/fan-out/fan-in fixture.
- L2: one owner-approved adopter artifact lane.

## Stop Conditions

Stop if reuse depends only on a digest, uses stale source, crosses credential
authority, or alters owner publication.

## Removal

Disable artifact reuse; each owner consumer rebuilds through its native
entrypoint.
