# Pulse 69: cleanup-owning capability executor successor

Status: Complete sealed infrastructure release; no authority or diagnostic
execution

## Goal

Close the exact Pulse 57 native staged-bundle cleanup defect without modifying
any frozen predecessor release. Pulse 69 preserves exact Pulse 57/Pulse 56/
Pulse 51 execution semantics while moving native staged-bundle ownership into
the WSL session itself.

## Released control

The production callable remains injection-free and accepts only `repo_root`,
`descriptor_root`, `private_runtime_root`, `p27_cycle_root`, and
`ubuntu_runtime_parent`. It byte-binds the exact Pulse 57 release, reuses its
descriptor freezing, dispatch construction, normalization, topology
accounting, and terminal event semantics, and changes only native staged-bundle
custody.

Pulse 69 stages the exact Pulse 57 worker/Pulse 56 tree under the caller's
native `ubuntu_runtime_parent`, captures the exact owned bundle identity
(parent/root device+inode plus bounded expected tree shape), retains that
identity for the full worker lifetime, closes the worker/capability first, and
then removes only that owned bundle through a native no-follow bounded tree
walk. It never deletes the caller parent or arbitrary siblings. Bundle
substitution, unexpected entries, non-directory roots, unverifiable absence,
or cleanup uncertainty fail closed as terminal cleanup failure.

## Evidence

- [Release](../../../../docs/simulations/profile-diff-held-out/pulse-69-capability-bound-diagnostic-executor-successor-release/README.md)
- [Qualification receipt](../../../../docs/simulations/profile-diff-held-out/pulse-69-capability-bound-diagnostic-executor-successor-release/qualification-receipt.json)
- [Role review](../../../../docs/plans/reviews/PULSE-69-CLEANUP-OWNING-CAPABILITY-EXECUTOR-SUCCESSOR-ROLE-REVIEW.md)
- [Rust validator](../../../../crates/ferris-cli/tests/pulse_69_capability_bound_diagnostic_executor_successor_release.rs)

Qualification is fake-only: eight receipt-listed behavioral controls, 20 fake
cycles, 2,760 harmless launches, one owned staged-bundle cleanup per cycle,
bundle retention through worker lifetime, zero post-close residue, and no real
FERRIS execution.
