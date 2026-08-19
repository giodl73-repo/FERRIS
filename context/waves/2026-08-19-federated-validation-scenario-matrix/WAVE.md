# Wave: Federated Validation Scenario Matrix

Status: Complete
Implementation authority: One measurement-only pulse
Successor authority: None

## Decision

Test whether the existing eight-workspace value fixture behaves coherently
across graph depth, fan-out, independent changed branches, mixed input kinds,
workspace-manifest fallback, and changed-package ordering.

## Authorized slice

- reuse the existing synthetic eight-workspace fixture unchanged;
- add deterministic scenarios to the existing focused value test;
- require selected scope to equal the union of direct inputs and their
  explicit transitive reverse dependents;
- prove a workspace-manifest change widens that workspace internally without
  becoming an application-wide fallback;
- prove changed-package order does not change semantic identities; and
- make no product-code, fixture-topology, schema, dependency, timing, or
  execution change.

## Stop conditions

Stop rather than expand if a scenario requires inferred relationships,
production repositories, validation execution, Git discovery, network access,
new dependencies, or a changed V0 contract.

## Completion

Completion requires the response curve and union scenarios recorded in
[`Pulse 01`](pulses/pulse-01.md), focused tests, existing federated-validation
regressions, workspace check, targeted Clippy, focused rustfmt, JSON parsing,
and diff hygiene.

## Removal

Delete the added test cases and this wave. The original value fixture and
first measured pulse remain valid independently.
