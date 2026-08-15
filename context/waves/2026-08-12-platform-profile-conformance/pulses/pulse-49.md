# Pulse 49: withdrawn prelaunch process-exit authority

Disposition: `invalid-prelaunch-authority-integrity` (permanent,
non-retryable, null-conclusion).

## Prelaunch authority closeout

Pulse 49's prelaunch authority at commit
`80f78fa4edb2d3497a830b2879ea9ff9c6f0aea5` is permanently withdrawn before
execution. The blocker is `P49-P35-CASE-PROCESS-CARDINALITY-CONFLICT` at
`prelaunch-authority-validation`.

The exact historical declaration authorized 70 cases and 70 OS processes per
platform (140 processes total) while requiring exact Pulse 35 descriptors.
The independently reproduced public Pulse 35 materializer yields exactly 70
descriptors per platform: 69 `launch-ready` process cases and one final
`no-launch` case. That final descriptor has `not-materialized` `before` and
`after` roles and external prerequisite
`external-immutable-binary-freeze`. Honoring that descriptor cannot create
70 OS processes per platform, so no conforming execution exists.

No launch was consumed. Independent custody refused before launch: all
ordered-gate, materializer, candidate, process, Pulse 47, and Pulse 43
invocations remain zero. There is no private operation, data, artifact,
runtime root transfer, result root, witness root, or inference. The category,
diagnostic, product, and fix conclusions are null. This is not a retry,
resume, reconstruction, reseed, reuse, correlation, or inference of any
earlier pulse.

## Historical artifacts and successor boundary

The prelaunch declaration, closed schema, and mutation controls remain exact
historical artifacts from authority commit
`80f78fa4edb2d3497a830b2879ea9ff9c6f0aea5`; they are not amended to make
the invalid authority appear executable. Their historical declaration identity
is `sha256:01101bb7d2a63b657940f82f80eb3edcd3ab7bba05cb8cd54e4dd0c87ce8a3ee`.

Any future successor needs fresh, explicit authority for each platform:
70 case dispositions, 69 launch-ready processes, and one no-launch
disposition. Across Windows and Ubuntu that is 140 case dispositions, 138
processes, and two no-launch dispositions. It must not inherit or consume
Pulse 49's withdrawn launch or publication authority.

## Evidence

- [Prelaunch authority closeout](../../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_49_AUTHORITY.md)
- [Historical declaration](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-49-authority.json)
- [Historical closed schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-49-authority.v1.schema.json)
- [Historical mutation controls](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-49-authority-mutations.json)
- [Nine-role closeout review](../../../../docs/plans/reviews/PULSE-49-PUBLIC-CATALOG-SUCCESSOR-AUTHORITY-ROLE-REVIEW.md)
- [Historical authority validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_49_authority.rs)
- [Prelaunch closeout validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_49_closeout.rs)
