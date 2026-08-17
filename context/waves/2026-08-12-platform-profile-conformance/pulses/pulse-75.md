# Pulse 75: stage-bootstrap/worker-identity capability executor successor

Status: complete sealed infrastructure release; no authority or real diagnostic
execution

## Goal

Close the remaining native WSL cleanup-ownership and worker-launch substitution
gaps in frozen Pulse 72 without changing any frozen predecessor.

## Released control

The production callable remains
`run_capability_bound_diagnostic_executor(repo_root, descriptor_root, private_runtime_root, p27_cycle_root, ubuntu_runtime_parent)`.

Pulse 75 byte-binds exact Pulse 72 through a verified sibling sealed loader,
owns every post-create stage failure inside the WSL bootstrap, classifies
cleanup uncertainty with precedence, and closes the host-revalidate-to-launch
race by passing expected parent/root identity and worker/dependency hashes into
the exact WSL bootstrap process. That process re-validates parent/root identity,
opens and hashes the worker no-follow, executes only from verified bytes, and
requires the worker to re-check staged bundle identity before loading
dependencies.

Cleanup still removes only the original owned tree. Replacement deletion is
forbidden; unresolved ownership becomes fatal `P57-INDETERMINATE-CLEANUP`.

## Evidence

- [Release](../../../../docs/simulations/profile-diff-held-out/pulse-75-capability-bound-diagnostic-executor-stage-bootstrap-worker-identity-successor-release/README.md)
- [Qualification receipt](../../../../docs/simulations/profile-diff-held-out/pulse-75-capability-bound-diagnostic-executor-stage-bootstrap-worker-identity-successor-release/qualification-receipt.json)
- [Role review](../../../../docs/plans/reviews/PULSE-75-STAGE-BOOTSTRAP-WORKER-IDENTITY-CAPABILITY-EXECUTOR-SUCCESSOR-ROLE-REVIEW.md)
- [Rust validator](../../../../crates/ferris-cli/tests/pulse_75_capability_bound_diagnostic_executor_stage_bootstrap_worker_identity_successor_release.rs)

Qualification is fake-only: 15 deterministic control tests, 20 cycles, 2,760
harmless launches, one staged-identity revalidation and one owned-bundle
cleanup per cycle, and no real FERRIS execution.
