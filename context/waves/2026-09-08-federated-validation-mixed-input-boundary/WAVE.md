# Wave: Federated Validation Mixed-Input Boundary

Status: Complete
Implementation authority: One test-only pulse
Successor authority: None

## Decision

Preserve the existing maximum explicit-input contract with a generated
multi-package application regression that exercises the combined changed-path
and changed-package boundary.

## Authorized slice

- generate one disposable 16-workspace application with eight independent Cargo
  packages per workspace;
- submit all 128 workspace-qualified package names and all 128 matching source
  paths in one request;
- prove forward, reverse, and rotated input order produce the same complete JSON
  result and exactly 128 selected packages across 16 direct workspace plans;
- prove a 257th input returns the documented blocked result before Cargo
  metadata loading;
- use relative child-process arguments so the maximum-input test remains
  executable within Windows command-line limits; and
- make no product-code, public fixture, schema, dependency, execution,
  performance, timeout, caching, or parallelism change.

## Stop conditions

Stop rather than expand if the regression requires production repositories,
network access, inferred relationships, validation execution, a new dependency,
or a change to the existing input limit or result contract.

## Completion

Completion requires the structural results recorded in
[`Pulse 01`](pulses/pulse-01.md), the focused integration test, targeted
Clippy, focused rustfmt, diff hygiene, and clean review.

## Removal

Delete the generated-fixture helper, focused regression, and this wave.
Existing product behavior and prior scaling fixtures remain unchanged.
