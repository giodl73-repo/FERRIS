# Pulse 84: witnessed capability/materialization diagnostic authority

Status: Complete authority declaration; authorized and unexecuted

## Goal

Authorize one fresh independent `process-exit-agreement` diagnostic invocation
through the exact sealed Pulse 82 callable at immutable cutoff
`f874ebfe29e58460fc0a553418d11d6785e84df9`.

The cutoff contains the complete Pulse 83 readiness review and exact Pulse 82
release, but excludes this authority. Pulse 84 is not a retry, resume,
reinterpretation, or reuse of withdrawn Pulse 68.

## Authority

Pulse 84 authorizes exactly one later independent Pulse 82 callable attempt.
Custody may invoke exactly once:

`run_witness_preserving_capability_materialization_executor(repo_root,
private_runtime_root, p27_cycle_root, p39_checkout_root, p41_final_root,
ubuntu_runtime_parent)`.

The authority is consumed when that invocation is attempted. Public
preparation and reversible environment checks do not consume it. Any failed
gate stops before invocation. Any attempted invocation is non-retryable and
non-resumable.

## Native-platform boundary

Pulse 82's Windows mutex and Linux abstract-socket behavior remains
fake-qualified before launch. Pulse 84 does not upgrade that evidence. The
single authorized invocation is the first real-platform exercise of the exact
sealed callable; unsupported or failed native behavior closes the program with
null conclusions.

## Evidence

- [Authority record](../../../../docs/simulations/profile-diff-held-out/PROCESS_EXIT_DIAGNOSTIC_PULSE_84_AUTHORITY.md)
- [Authority declaration](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-84-authority.json)
- [Closed schema](../../../../docs/simulations/profile-diff-held-out/schemas/ferris.process-exit-diagnostic-pulse-84-authority.v1.schema.json)
- [Mutation controls](../../../../docs/simulations/profile-diff-held-out/fixtures/process-exit-diagnostic-pulse-84-authority-mutations.json)
- [Nine-role review](../../../../docs/plans/reviews/PULSE-84-WITNESSED-CAPABILITY-MATERIALIZATION-AUTHORITY-ROLE-REVIEW.md)
- [Static validator](../../../../crates/ferris-cli/tests/process_exit_diagnostic_pulse_84_authority.rs)

Pulse 84 performs no custody or execution and creates no seed, descriptor,
runtime root, candidate process, result, witness, transfer, or conclusion.
Execution requires a later independent custody action.
