# Pulse 49 withdrawn prelaunch authority

Disposition: `invalid-prelaunch-authority-integrity` (permanent,
non-retryable, null-conclusion).

## Independent prelaunch blocker

Pulse 49 authority commit
`80f78fa4edb2d3497a830b2879ea9ff9c6f0aea5` is permanently withdrawn before
launch at `prelaunch-authority-validation`. The blocker is
`P49-P35-CASE-PROCESS-CARDINALITY-CONFLICT`.

The exact historical declaration both requires Pulse 35's exact public
descriptors and asserts `processes_per_platform: 70` and
`total_processes: 140`. A fixed synthetic public-seed run of the exact public
Pulse 35 materializer deterministically produces 70 descriptors: 69
`launch-ready` and one final `no-launch`. The final descriptor has
`not-materialized` `before` and `after` states and requires
`external-immutable-binary-freeze`. It is a case disposition, not an OS
process. Therefore a platform cannot both honor all 70 descriptors and create
70 OS processes.

The independent custodian refused before launch. Launch count, P47
invocations, P43 invocations, ordered execution, materializer invocation,
candidate pair/process, private-operation, result-root, witness-root, and
runtime/public-root-transfer counts remain zero. No private operation, data,
or artifact exists; no inference is made. All category, diagnostic, product,
and fix conclusions are null.

## Historical integrity and successor requirements

The declaration, schema, and mutations are retained byte-for-byte as
prelaunch historical artifacts at
`80f78fa4edb2d3497a830b2879ea9ff9c6f0aea5`. The historical declaration
identity remains
`sha256:01101bb7d2a63b657940f82f80eb3edcd3ab7bba05cb8cd54e4dd0c87ce8a3ee`;
its `authorized-unexecuted` status records what was withdrawn, not live
execution authority.

A future successor must be independently authorized and must explicitly
separate per-platform case and process quantities: 70 case dispositions, 69
processes, and one no-launch disposition. Its two-platform totals must be 140
case dispositions, 138 processes, and two no-launch dispositions. It cannot
retry, resume, consume, reconstruct, or infer from this withdrawn authority.

## Bound historical artifacts and validation

- [Historical declaration](fixtures/process-exit-diagnostic-pulse-49-authority.json)
- [Historical closed schema](schemas/ferris.process-exit-diagnostic-pulse-49-authority.v1.schema.json)
- [Historical exhaustive mutations](fixtures/process-exit-diagnostic-pulse-49-authority-mutations.json)
- [Nine-role closeout review](../../plans/reviews/PULSE-49-PUBLIC-CATALOG-SUCCESSOR-AUTHORITY-ROLE-REVIEW.md)
- [Historical authority validator](../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_49_authority.rs)
- [Prelaunch closeout validator](../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_49_closeout.rs)
