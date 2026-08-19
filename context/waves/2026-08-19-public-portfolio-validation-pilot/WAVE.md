# Wave: Public Portfolio Validation Pilot

Status: Complete
Implementation authority: One measurement-only pulse
Successor authority: None

## Decision

Determine whether the unchanged `federated-validation-plan` can compose
meaningful validation scope across clean exact revisions of real public Rust
repositories while preserving each repository's Cargo and validation
authority.

## Authorized slice

- run one local read-only pilot over exact public FERRIS, PARLOR, RUNE, and
  ICELINES revisions;
- require locked/offline Cargo metadata independently for every workspace;
- model PARLOR and RUNE as explicit conservative Ferris contract-migration
  consumers and ICELINES as an unrelated control;
- measure producer-contract, consumer-package, unrelated-control, and
  application-owned scenarios;
- retain no absolute paths, source, Cargo output, or local semantic identities
  in public evidence;
- mutate no child repository; and
- make no product-code, schema, dependency, process-control, or execution
  change.

## Relationship boundary

PARLOR and RUNE pin exact Ferris validation-plan contracts. They do not have a
live Cargo dependency on the current Ferris checkout. The pilot's explicit
`depends_on` edges therefore mean:

> when evaluating a Ferris validation-plan contract change for migration,
> conservatively require the two named consumer scopes.

The edges do not claim automatic source invalidation, dependency resolution,
or required validation for arbitrary Ferris implementation changes.

## Stop conditions

Stop rather than expand if the pilot requires dirty owner state, child-repo
edits, network access, inferred relationships, validation execution, private
repositories, a new dependency, or a production/support claim.

## Completion

Completion requires the exact revisions, scenarios, outputs, environment,
boundaries, and cleanup recorded in
[`Pulse 01`](pulses/pulse-01.md) and the
[machine-readable receipt](../../../docs/plans/validation/FERRIS-PUBLIC-PORTFOLIO-PILOT-RECEIPT.json).

## Removal

Delete this wave, the validation record, and its receipt. No product or child
repository state changes.
