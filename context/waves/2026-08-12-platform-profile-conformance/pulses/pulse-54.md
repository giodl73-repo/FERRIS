# Pulse 54: independent witness-preserving diagnostic authority

Status: Authorized-unexecuted

## Goal

Authorize one fresh, independent, single-use diagnostic route over the sealed
Pulse 51, Pulse 52, and Pulse 53 chain without reviving any withdrawn
predecessor or executing it in this authority pulse.

## Authority

Pulse 54 binds the exact self-excluding cutoff
`42a16e298c5af55b05df5ceb8e3477d0dd45c814`, the complete exact current
public P27/P31/P33/P35/P37/P39/P41/P43/P44/P45/P47/P51/P52/P53 identities,
and the only production entrypoint
`run_witness_preserving_ordered_executor`. It permits one invocation after
public-only custody, binary-freeze, gate-catalog, and root-freshness checks.
It does not execute that invocation here.

Pulse 48 remains permanently invalid/null; Pulse 49 remains permanently
withdrawn invalid-prelaunch/null; Pulse 50 remains permanently withdrawn
invalid-prelaunch/null. Pulse 54 is not a retry, resume, reconstruction,
reseed, reuse, correlation, or inference of any of them.

## Fixed route

The pre-call path creates fresh anonymous exact-cutoff checkouts with
`core.autocrlf=false`, exact Windows `/Brepro` and Ubuntu `Ubuntu-24.04` WSL
Pulse 33 freezes, one P44 custody operation per platform, and fresh P39/P41,
runtime, and P27 roots. It creates no seed, descriptor, candidate process,
result, or witness artifact.

The sole P53 call runs P39/P41, Windows P44/P45, Ubuntu P44/P45, P27, P31,
P35/P37, one 32-byte CSPRNG seed, one P35 materializer/verifier, then
`70/69/1` per platform and `140/138/2` total. The first semantic projection
mismatch stops later work. The terminal path is exactly one P47-to-P43 route.

The public transfer rule is exact: P43 `2/2` plus P47 `2/2` goes to the
separate Pulse 54 result and witness destinations; an exact
`published-failure-witness` retains only the P47 `2/2` witness and the P43
destination remains absent; invalid or cleanup-indeterminate states make no
success claim and copy no tree. All conclusions are null.

## Evidence

- [Authority declaration](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-54-authority.json)
- [Closed schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-54-authority.v1.schema.json)
- [Mutation registry](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-54-authority-mutations.json)
- [Authority record](../../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_54_AUTHORITY.md)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-54-WITNESS-PRESERVING-DIAGNOSTIC-AUTHORITY-ROLE-REVIEW.md)
- [Rust authority validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_54_authority.rs)
