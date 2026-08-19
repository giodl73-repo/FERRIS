# Wave: Federated Validation Value Proof

Status: Complete
Implementation authority: One measurement-only pulse
Successor authority: None

## Decision

Determine whether the shipped read-only `federated-validation-plan` produces
materially narrower, explainable validation scope on a representative bounded
application topology without weakening its conservative fallback.

## Authorized slice

- add one public development fixture containing eight independent single-package
  Cargo workspaces and explicit consumer-owned relationships;
- add one integration test covering a leaf change, shared-workspace change,
  and application-owned change;
- report workspace-scope reduction separately from local planning overhead;
- record exact commands, environment, results, limitations, and removal; and
- make no product-code, schema, command, dependency, or execution change.

## Baseline

The comparison baseline is owner validation of all eight declared workspace
scopes. This pulse does not assume one command per workspace, predict build
time, or claim that an avoided scope equals a fixed latency or cost saving.

## Stop conditions

Stop rather than expand if measurement requires validation execution,
repository discovery, inferred relationships, production repositories,
network access, a new dependency, a product output change, or a performance
threshold in the normal test suite.

## Completion

Completion requires:

- deterministic structural assertions for all three scenarios;
- an explicit opt-in seven-sample local overhead report;
- no claim that planning latency is validation latency;
- normal Cargo behavior and all existing V0 contracts unchanged;
- targeted test, workspace check, formatting, and diff hygiene; and
- one measured result in [`Pulse 01`](pulses/pulse-01.md).

## Removal

Delete the fixture, focused integration test, measured result, and this wave.
No product behavior or consumer contract changes.
