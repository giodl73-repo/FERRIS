# Pulse 72: stage-identity capability executor successor

Status: Complete sealed infrastructure release; no authority or diagnostic
execution

## Goal

Close the exact Pulse 69 / Pulse 57 stage-to-identity race without modifying
any frozen predecessor release.

## Released control

The production callable remains injection-free and accepts only `repo_root`,
`descriptor_root`, `private_runtime_root`, `p27_cycle_root`, and
`ubuntu_runtime_parent`. It byte-binds the exact Pulse 69 release through a
verified sibling-path sealed loader, preserving exact Pulse 69 / Pulse 57 /
Pulse 56 / Pulse 51 execution semantics while hardening the native staging
handoff.

Pulse 72 stages the exact Pulse 57 worker tree with one bounded WSL bootstrap
that captures canonical root and parent device/inode/type identity immediately
after exclusive root creation and again after files are finalized. That exact
identity is carried inside the staged bundle, lexical root and parent identity
are revalidated immediately before worker launch, and cleanup removes only the
original stage-time root path/inode. Root or parent substitution, replacement,
non-directory resolution, or cleanup uncertainty fail closed as
`P57-INDETERMINATE-CLEANUP` without deleting replacements or arbitrary
siblings.

## Evidence

- [Release](../../../../docs/simulations/profile-diff-held-out/pulse-72-capability-bound-diagnostic-executor-stage-identity-successor-release/README.md)
- [Qualification receipt](../../../../docs/simulations/profile-diff-held-out/pulse-72-capability-bound-diagnostic-executor-stage-identity-successor-release/qualification-receipt.json)
- [Role review](../../../../docs/plans/reviews/PULSE-72-STAGE-IDENTITY-CAPABILITY-EXECUTOR-SUCCESSOR-ROLE-REVIEW.md)
- [Rust validator](../../../../crates/ferris-cli/tests/pulse_72_capability_bound_diagnostic_executor_stage_identity_successor_release.rs)

Qualification is fake-only: 11 receipt-listed behavioral controls, 20 cycles,
2,760 harmless launches, one stage-identity revalidation and one owned
staged-bundle cleanup per cycle, and no real FERRIS execution.
