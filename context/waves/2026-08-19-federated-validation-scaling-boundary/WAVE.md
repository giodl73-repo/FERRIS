# Wave: Federated Validation Scaling Boundary

Status: Complete
Implementation authority: One measurement-only pulse
Successor authority: None

## Decision

Verify the declared 2-16 workspace application boundary and measure local
planning overhead as sequential Cargo metadata work grows from 2 to 16
independent workspaces.

## Authorized slice

- generate disposable 2-, 4-, 8-, 16-, and 17-workspace applications inside
  a focused integration test;
- use one-package independent Cargo workspaces in an explicit relationship
  chain;
- prove leaf and root selection behavior at every accepted size;
- prove 17 workspaces return the documented typed error before owner metadata
  loading;
- report five-sample local leaf-planning medians at accepted sizes; and
- make no product-code, public fixture, schema, dependency, parallelism,
  caching, timeout, or execution change.

## Stop conditions

Stop rather than expand if the measurement requires production repositories,
network access, inferred relationships, validation execution, changed process
controls, a new dependency, or a performance threshold in the normal suite.

## Completion

Completion requires the structural and timing results recorded in
[`Pulse 01`](pulses/pulse-01.md), existing value and federated-validation
regressions, workspace check, targeted Clippy, focused rustfmt, and diff
hygiene.

## Removal

Delete the focused generated-fixture test and this wave. Existing product
behavior and prior value fixtures remain unchanged.
