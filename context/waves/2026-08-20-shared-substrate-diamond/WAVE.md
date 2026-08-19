# Wave: Shared-Substrate Diamond

Status: Complete
Implementation authority: One measurement and licensing pulse
Successor authority: None

## Decision

Test the unchanged `federated-validation-plan` against a real public
shared-substrate diamond and align Ferris licensing with the common Rust
Project `Apache-2.0 OR MIT` convention.

## Authorized slice

- use clean exact FLETCH, METIS-CORE, BISECT, ROUTE, and ICELINES revisions;
- require locked/offline Cargo metadata independently for every workspace;
- model only the observed producer-to-consumer relationships;
- measure producer, dual-producer, consumer-local, and application-owned
  scenarios;
- compare producer checkout revisions with consumer lockfile revisions;
- retain no absolute paths or local semantic identities in public evidence;
- change no planner behavior, schema, dependency, or child repository; and
- rename the MIT license file, add the standard Apache 2.0 text, and publish
  `Apache-2.0 OR MIT` package metadata.

## Relationship boundary

The explicit relationships mean that a change to a named shared substrate
requires conservative review in the named consumers. Cargo remains
authoritative for each consumer's actual resolved revision. A relationship
does not prove that the producer checkout matches the consumer lock, that a
change is compatible, or that every consumer command must run.

## Stop conditions

Stop rather than expand if the pilot requires dirty owner state, child edits,
network access, inferred semantic compatibility, validation execution, a new
dependency, or an affiliation, support, production, or build-time savings
claim.

## Completion

Completion requires the exact revisions, scenarios, resolved-revision
comparison, environment, boundaries, licensing result, and cleanup recorded
in [`Pulse 01`](pulses/pulse-01.md) and the
[machine-readable receipt](../../../docs/plans/validation/FERRIS-SHARED-SUBSTRATE-DIAMOND-RECEIPT.json).

## Removal

Delete this wave and its validation records, restore the former license
declaration and file name, and remove `LICENSE-APACHE`. No product or child
repository state changes.
